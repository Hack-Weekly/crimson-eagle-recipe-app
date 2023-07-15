use diesel::prelude::*;
use rocket::serde::json::Json;
use validator::Validate;

use crate::database;
use crate::models::*;
use crate::schema::recipes::dsl::*;
use crate::schema::*;

/// Add recipe
///
/// Create new recipe in the database
#[utoipa::path(
    post,
    path = "/recipes",
    request_body = RecipePostDTO,
    tag = "recipes",
    responses(
        (status = 201, description = "Recipe created succesfully", body = RecipeResultDTO),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal Server Error"),
    ),
    security(
        ("name" = ["Bearer"])
    ),
)]
#[post("/recipes", data = "<addrecipe>")]
pub fn create_recipe(
    addrecipe: Json<RecipePostDTO>,
    key: Result<Jwt, NetworkResponse>,
) -> RecipeResponse<RecipeResultDTO> {
    let connection = &mut database::establish_connection();

    let user_id: i32 = match key {
        Ok(k) => k.claims.subject_id,
        Err(_) => {
            return RecipeResponse::Unauthorized(String::from(
                "Please log in to be able to create recipes.",
            ))
        }
    };

    match addrecipe.validate() {
        Ok(_) => (),
        Err(err) => return RecipeResponse::BadRequest(err.to_string()),
    };
    let addrecipe = addrecipe.into_inner();

    let mut recipe = match diesel::insert_into(recipes)
        .values(RecipesInput::from(&addrecipe))
        .get_result::<Recipe>(connection)
    {
        Ok(res) => RecipeResultDTO::from(res),
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Cannot insert recipe into the database.",
            ))
        }
    };

    // add logged in user as owner
    match diesel::insert_into(recipes_users::table)
        .values((
            recipes_users::recipe_id.eq(recipe.id),
            recipes_users::user_id.eq(user_id),
        ))
        .execute(connection)
    {
        Ok(_) => (),
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Cannot insert owner into the database.",
            ))
        }
    };
    recipe.owned = Some(true);

    // add instructions
    if addrecipe.instructions.is_some() && !addrecipe.instructions.clone().unwrap().is_empty() {
        let instructions = addrecipe
            .instructions
            .clone()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(k, i)| InstructionInsert {
                instruction: i,
                display_order: k as i32,
                recipe_id: recipe.id,
            })
            .collect::<Vec<InstructionInsert>>();
        match diesel::insert_into(instructions::table)
            .values(instructions)
            .execute(connection)
        {
            Ok(_) => (),
            Err(_) => {
                return RecipeResponse::InternalServerError(String::from(
                    "Cannot insert instructions into the database.",
                ))
            }
        };
        recipe.instructions = addrecipe.instructions.unwrap();
    }

    // add instructions
    if addrecipe.ingredients.is_some() && !addrecipe.ingredients.clone().unwrap().is_empty() {
        let available_ingredents = match ingredients::table.load::<Ingredient>(connection) {
            Ok(res) => res,
            Err(_) => {
                return RecipeResponse::InternalServerError(String::from(
                    "Cannot load ingredients from the database.",
                ))
            }
        };

        let mut recipe_ingredients_inserts = Vec::<RecipeIngredientInsert>::new();
        let mut ingredient_inserts = Vec::<IngredientInsert>::new();
        let mut delayed_inserts = Vec::<((Option<String>, String), Vec<Option<f32>>)>::new();
        for addingredient in addrecipe.ingredients.clone().unwrap() {
            let mut ingredient: Option<&Ingredient> = None;
            if !available_ingredents.is_empty() {
                ingredient = available_ingredents
                    .iter()
                    .find(|i| i.unit == addingredient.unit && i.label == addingredient.label);
            }
            match ingredient {
                Some(i) => {
                    recipe_ingredients_inserts.push(RecipeIngredientInsert {
                        amount: addingredient.amount,
                        recipe_id: recipe.id,
                        ingredient_id: i.id,
                    });
                }
                // need to add ingredients and then use those ids in another recipe_ingredients insert
                None => {
                    let mut found = false;
                    for ((u, l), a) in &mut delayed_inserts {
                        if u.clone() == addingredient.unit && l.clone() == addingredient.label {
                            a.push(addingredient.amount);
                            found = true;
                        }
                    }
                    if !found {
                        ingredient_inserts.push(IngredientInsert {
                            unit: addingredient.unit.clone(),
                            label: addingredient.label.clone(),
                        });
                        delayed_inserts.push((
                            (addingredient.unit, addingredient.label),
                            vec![addingredient.amount],
                        ));
                    }
                }
            }
        }

        // add ingredients, get ids
        if !ingredient_inserts.is_empty() {
            let new_ingredients = match diesel::insert_into(ingredients::table)
                .values(&ingredient_inserts)
                .get_results::<Ingredient>(connection)
            {
                Ok(res) => res,
                Err(_) => {
                    return RecipeResponse::InternalServerError(String::from(
                        "Cannot insert ingredients into the database.",
                    ))
                }
            };

            for ingredient in new_ingredients {
                for ((unit, label), amounts) in &delayed_inserts {
                    if ingredient.unit == unit.clone() && ingredient.label == label.clone() {
                        for a in amounts {
                            recipe_ingredients_inserts.push(RecipeIngredientInsert {
                                amount: *a,
                                recipe_id: recipe.id,
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
                    return RecipeResponse::InternalServerError(String::from(
                        "Cannot insert recipe ingredients into the database.",
                    ))
                }
            };
        }
        recipe.ingredients = addrecipe.ingredients.unwrap();
    }

    // add tags
    if addrecipe.tags.is_some() && !addrecipe.tags.clone().unwrap().is_empty() {
        let available_tags = match tags::table.load::<Tag>(connection) {
            Ok(res) => res,
            Err(_) => {
                return RecipeResponse::InternalServerError(String::from(
                    "Cannot load tags from the database.",
                ))
            }
        };

        let mut tag_list = Vec::<TagDTO>::new();
        let mut recipes_tags_inserts = Vec::<RecipeTag>::new();
        let mut tag_inserts = Vec::<TagDTO>::new();
        let mut delayed_inserts = Vec::<String>::new();
        for addtag in addrecipe.tags.clone().unwrap() {
            let mut tag: Option<&Tag> = None;
            if !available_tags.is_empty() {
                tag = available_tags.iter().find(|t| t.label == addtag);
            }
            match tag {
                Some(t) => {
                    recipes_tags_inserts.push(RecipeTag {
                        recipe_id: recipe.id,
                        tag_id: t.id,
                    });
                    tag_list.push(TagDTO::from(t));
                }
                // need to add tags and then use those ids in another recipes_tags insert
                None => {
                    let mut found = false;
                    for t in &mut delayed_inserts {
                        // in case the post has doubled tags
                        if t.clone() == addtag {
                            found = true;
                        }
                    }
                    if !found {
                        tag_inserts.push(TagDTO::from(TagPostDTO {
                            label: addtag.clone(),
                        }));
                        delayed_inserts.push(addtag);
                    }
                }
            }
        }

        // add tags, get ids
        if !tag_inserts.is_empty() {
            let new_tags = match diesel::insert_into(tags::table)
                .values(&tag_inserts)
                .get_results::<Tag>(connection)
            {
                Ok(res) => res,
                Err(_) => {
                    return RecipeResponse::InternalServerError(String::from(
                        "Cannot insert tags into the database.",
                    ))
                }
            };

            for t in &delayed_inserts {
                for tag in &new_tags {
                    if tag.label == t.clone() {
                        recipes_tags_inserts.push(RecipeTag {
                            recipe_id: recipe.id,
                            tag_id: tag.id,
                        });
                        tag_list.push(TagDTO::from(tag));
                    }
                }
            }
        }
        // add recipes_tags
        if !recipes_tags_inserts.is_empty() {
            match diesel::insert_into(recipes_tags::table)
                .values(recipes_tags_inserts)
                .execute(connection)
            {
                Ok(_) => (),
                Err(_) => {
                    return RecipeResponse::InternalServerError(String::from(
                        "Cannot insert recipe tags into the database.",
                    ))
                }
            };
        }
        recipe.tags = tag_list;
    }

    RecipeResponse::Created(Json(recipe))
}
