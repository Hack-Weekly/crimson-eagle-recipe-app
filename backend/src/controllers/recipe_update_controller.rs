use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database;
use crate::models::*;
use crate::schema::recipes::dsl::*;
use crate::schema::*;

#[put("/recipes/<recipe_id>", data = "<updaterecipe>")]
pub fn update_recipe(
    recipe_id: i32,
    updaterecipe: Json<RecipePutDTO>,
) -> Result<Json<RecipeResultDTO>, Status> {
    let connection = &mut database::establish_connection();

    let recipe: Recipe = match recipes.find(recipe_id).first::<Recipe>(connection) {
        Ok(result) => {
            let new_title = match &updaterecipe.title {
                Some(t) => {
                    if t.is_empty() {
                        &result.title
                    } else {
                        t
                    }
                }
                None => &result.title,
            };
            let new_servings = match &updaterecipe.servings {
                Some(s) => {
                    if s.is_empty() {
                        &result.servings
                    } else {
                        s
                    }
                }
                None => &result.servings,
            };
            match diesel::update(&result)
                .set((
                    title.eq(new_title),
                    servings.eq(new_servings),
                    updated_at.eq(diesel::dsl::now), // we have to update this even if title or servings were untouched
                ))
                .get_result::<Recipe>(connection)
            {
                Ok(res) => res,
                Err(_) => return Err(Status::InternalServerError),
            }
        }
        Err(_) => return Err(Status::NotFound),
    }; // get updated recipe or return error

    // get updated instructions
    let recipe_instructions =
        match update_instructions(recipe_id, &updaterecipe.instructions, connection) {
            Ok(res) => res,
            Err(_) => return Err(Status::InternalServerError),
        };

    // get updated ingredients
    let recipe_ingredients =
        match update_ingredients(recipe_id, &updaterecipe.ingredients, connection) {
            Ok(res) => res,
            Err(_) => return Err(Status::InternalServerError),
        };

    let mut recipe = RecipeResultDTO::from(recipe);
    recipe.instructions = recipe_instructions
        .into_iter()
        .map(|i| i.instruction)
        .collect::<Vec<String>>();
    recipe.ingredients = recipe_ingredients
        .into_iter()
        .map(IngredientDTO::from)
        .collect::<Vec<IngredientDTO>>();

    Ok(Json(recipe))
}

fn update_instructions(
    recipe_id: i32,
    update: &Option<Vec<String>>,
    connection: &mut PgConnection,
) -> Result<Vec<Instruction>, Status> {
    let recipe_instructions = match instructions::table
        .filter(instructions::recipe_id.eq(recipe_id))
        .order(instructions::display_order.asc())
        .load::<Instruction>(connection)
    {
        Ok(res) => res,
        Err(_) => return Err(Status::InternalServerError),
    };

    if update.is_none() {
        return Ok(recipe_instructions);
    }

    let mut old_instructions: Vec<Instruction> = recipe_instructions;
    let mut new_instructions: Vec<String> = match update.clone() {
        Some(i) => i,
        None => return Err(Status::InternalServerError),
    };

    let diff = old_instructions.len() as i32 - new_instructions.len() as i32;

    // need to delete some
    if diff > 0 {
        let mut delete_ids = Vec::<i32>::new();
        for _ in 0..diff {
            delete_ids.push(old_instructions.pop().unwrap().id);
        }
        match diesel::delete(instructions::table)
            .filter(instructions::id.eq_any(delete_ids))
            .execute(connection)
        {
            Ok(_) => (),
            Err(_) => {
                println!("DB error on delete.");
                return Err(Status::InternalServerError);
            }
        }
    }
    // need to add new
    else if diff < 0 {
        let mut inserts = Vec::<InstructionInsert>::new();
        let mut display_order = new_instructions.len() as i32;
        for _ in 0..(diff.abs()) {
            let new = new_instructions.pop().unwrap();
            inserts.push(InstructionInsert {
                instruction: new,
                display_order,
                recipe_id,
            });
            display_order -= 1;
        }
        match diesel::insert_into(instructions::table)
            .values(
                inserts
                    .into_iter()
                    .rev()
                    .collect::<Vec<InstructionInsert>>(),
            )
            .execute(connection)
        {
            Ok(_) => (),
            Err(_) => {
                println!("DB error on insert.");
                return Err(Status::InternalServerError);
            }
        };
    }

    // need to update
    if !old_instructions.is_empty() {
        // build sql query for batch update
        let mut query = String::from("UPDATE instructions SET instruction = CASE");

        let update_ids: Vec<String> = old_instructions
            .iter()
            .zip(new_instructions)
            .filter_map(|(old, new)| {
                if old.instruction != new {
                    query.push_str(&format!(" WHEN id={} THEN '{}'", old.id, new));
                    Some(old.id.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        // don't update if not needed
        if !update_ids.is_empty() {
            query.push_str(&format!(
                " ELSE instruction END WHERE id IN ({})",
                update_ids.join(",")
            ));
            println!("Query: {query}");
            match diesel::sql_query(query).execute(connection) {
                Ok(_) => (),
                Err(_) => {
                    println!("DB error on batch update.");
                    return Err(Status::InternalServerError);
                }
            };
        }
    }

    // return updated instructions
    match instructions::table
        .filter(instructions::recipe_id.eq(recipe_id))
        .order(instructions::display_order.asc())
        .load::<Instruction>(connection)
    {
        Ok(res) => Ok(res),
        Err(_) => Err(Status::InternalServerError),
    }
}

fn update_ingredients(
    recipe_id: i32,
    update: &Option<Vec<IngredientDTO>>,
    connection: &mut PgConnection,
) -> Result<Vec<(RecipeIngredient, Ingredient)>, Status> {
    let recipe_ingredients: Vec<(RecipeIngredient, Ingredient)> = match recipe_ingredients::table
        .filter(recipe_ingredients::recipe_id.eq(recipe_id))
        .inner_join(ingredients::table)
        .load::<(RecipeIngredient, Ingredient)>(connection)
    {
        Ok(res) => res,
        Err(_) => return Err(Status::InternalServerError),
    };

    if update.is_none() {
        return Ok(recipe_ingredients);
    }

    let mut recipe_ingredients_inserts = Vec::<RecipeIngredientInsert>::new();
    let mut ingredient_inserts = Vec::<IngredientInsert>::new();
    let mut delayed_inserts = Vec::<((Option<String>, String), Vec<Option<f32>>)>::new(); // Ingredient, amount(s)
    let mut updates = Vec::<(RecipeIngredient, IngredientDTO)>::new();
    let mut delete_ids = Vec::<i32>::new();

    let mut update_ingredients: Vec<IngredientDTO> = update.clone().unwrap();
    let mut old_ingredients: Vec<(RecipeIngredient, Ingredient)> = recipe_ingredients;

    // get all ingredients for inserts, easier than filter by what we need
    // unnecessary query if there won't be any inserts
    let available_ingredents = match ingredients::table.load::<Ingredient>(connection) {
        Ok(res) => res,
        Err(_) => return Err(Status::InternalServerError),
    };

    for _ in 0..update_ingredients.len() {
        let new = update_ingredients.pop().unwrap();
        let mut addnew = true;

        old_ingredients.retain(|(r, i)| {
            // discard unaffected rows
            if r.amount == new.amount && i.unit == new.unit && i.label == new.label {
                addnew = false;
                false
            }
            // add update and discard
            else if i.unit == new.unit && i.label == new.label {
                updates.push((r.clone(), new.clone()));
                addnew = false;
                false
            }
            // keep
            else {
                true
            }
        });

        if addnew {
            // simple insert if the ingredient is available
            let mut ingredient: Option<&Ingredient> = None;
            if !available_ingredents.is_empty() {
                ingredient = available_ingredents
                    .iter()
                    .find(|i| i.unit == new.unit && i.label == new.label);
            }
            match ingredient {
                Some(i) => {
                    recipe_ingredients_inserts.push(RecipeIngredientInsert {
                        amount: new.amount,
                        recipe_id,
                        ingredient_id: i.id,
                    });
                }
                // need to add ingredients and then use those ids in another recipe_ingredients insert
                None => {
                    let mut found = false;
                    for ((u, l), a) in &mut delayed_inserts {
                        if u.clone() == new.unit && l.clone() == new.label {
                            a.push(new.amount);
                            found = true;
                        }
                    }
                    if !found {
                        ingredient_inserts.push(IngredientInsert {
                            unit: new.unit.clone(),
                            label: new.label.clone(),
                        });
                        delayed_inserts.push(((new.unit, new.label), vec![new.amount]));
                    }
                }
            }
        }
    }

    // delete
    if !old_ingredients.is_empty() {
        for _ in 0..(old_ingredients.len() as u32) {
            delete_ids.push(old_ingredients.pop().unwrap().0.id);
        }
        match diesel::delete(recipe_ingredients::table)
            .filter(recipe_ingredients::id.eq_any(delete_ids))
            .execute(connection)
        {
            Ok(_) => (),
            Err(_) => {
                println!("DB error on delete.");
                return Err(Status::InternalServerError);
            }
        };
    }

    // insert
    // add ingredients, get ids
    if !ingredient_inserts.is_empty() {
        let new_ingredients = match diesel::insert_into(ingredients::table)
            .values(&ingredient_inserts)
            .get_results::<Ingredient>(connection)
        {
            Ok(res) => res,
            Err(_) => return Err(Status::InternalServerError),
        };

        for ingredient in new_ingredients {
            for ((unit, label), amounts) in &delayed_inserts {
                if ingredient.unit == unit.clone() && ingredient.label == label.clone() {
                    for a in amounts {
                        recipe_ingredients_inserts.push(RecipeIngredientInsert {
                            amount: *a,
                            recipe_id,
                            ingredient_id: ingredient.id,
                        });
                    }
                }
            }
        }
    }
    // add recipe_ingredients
    if !recipe_ingredients_inserts.is_empty() {
        match diesel::insert_into(recipe_ingredients::table)
            .values(
                recipe_ingredients_inserts
                    .into_iter()
                    .rev() // reverse, to keep original order as much as possible
                    .collect::<Vec<RecipeIngredientInsert>>(),
            )
            .execute(connection)
        {
            Ok(_) => (),
            Err(_) => {
                println!("DB error on insert.");
                return Err(Status::InternalServerError);
            }
        };
    }

    // build sql query for batch update
    // TODO binding
    let mut query = String::from("UPDATE recipe_ingredients SET amount = CASE");

    let update_ids: Vec<String> = updates
        .into_iter()
        .map(|(old, new)| {
            query.push_str(&format!(
                " WHEN id={} THEN {}",
                old.id,
                match new.amount {
                    Some(a) => a.to_string(),
                    None => String::from("NULL"),
                }
            ));
            old.id.to_string()
        })
        .collect::<Vec<String>>();

    // don't update if not needed
    if !update_ids.is_empty() {
        query.push_str(&format!(
            " ELSE amount END WHERE id IN ({})",
            update_ids.join(",")
        ));

        match diesel::sql_query(query).execute(connection) {
            Ok(_) => (),
            Err(_) => {
                println!("DB error on batch update.");
                return Err(Status::InternalServerError);
            }
        };
    }

    match recipe_ingredients::table
        .filter(recipe_ingredients::recipe_id.eq(recipe_id))
        .inner_join(ingredients::table)
        .load::<(RecipeIngredient, Ingredient)>(connection)
    {
        Ok(res) => Ok(res),
        Err(_) => Err(Status::InternalServerError),
    }
}
