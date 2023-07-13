use diesel::prelude::*;

use crate::{models::*, schema::*};

pub fn get_recipe_elements(
    recipes_list: Vec<Recipe>,
    connection: &mut PgConnection,
    user_id: Option<i32>,
) -> Result<Vec<RecipeResultDTO>, String> {
    // get instructions
    let instructions_list: Vec<Instruction> = match Instruction::belonging_to(&recipes_list)
        .order(instructions::display_order.asc())
        .load::<Instruction>(connection)
    {
        Ok(res) => res,
        Err(_) => return Err(String::from("Cannot read instructions from the database.")),
    };

    // get ingredients
    let ingredients_list: Vec<(RecipeIngredient, Ingredient)> =
        match RecipeIngredient::belonging_to(&recipes_list)
            .inner_join(ingredients::table)
            .load::<(RecipeIngredient, Ingredient)>(connection)
        {
            Ok(res) => res,
            Err(_) => return Err(String::from("Cannot read ingredients from the database.")),
        };
    let ingredients_grouped = ingredients_list.grouped_by(&recipes_list);

    // get tags
    let tags_list: Vec<(RecipeTag, Tag)> = match RecipeTag::belonging_to(&recipes_list)
        .inner_join(tags::table)
        .load::<(RecipeTag, Tag)>(connection)
    {
        Ok(res) => res,
        Err(_) => return Err(String::from("Cannot read tags from the database.")),
    };
    let tags_grouped = tags_list.grouped_by(&recipes_list);

    // if user is logged in, get bookmarks
    let mut bookmarks_grouped = Vec::<Vec<Bookmark>>::new();
    let mut owned_grouped = Vec::<Vec<RecipeUser>>::new();
    if user_id.is_some() {
        let id = user_id.unwrap();
        // TODO add ownership
        let bookmarks_list: Vec<Bookmark> = match Bookmark::belonging_to(&recipes_list)
            .filter(bookmarks::user_id.eq(id))
            .load::<Bookmark>(connection)
        {
            Ok(res) => res,
            Err(_) => return Err(String::from("Cannot read bookmarks from the database.")),
        };
        bookmarks_grouped = bookmarks_list.grouped_by(&recipes_list);
        let owned_list: Vec<RecipeUser> = match RecipeUser::belonging_to(&recipes_list)
            .filter(recipes_users::user_id.eq(id))
            .load::<RecipeUser>(connection)
        {
            Ok(res) => res,
            Err(_) => return Err(String::from("Cannot read ownership from the database.")),
        };
        owned_grouped = owned_list.grouped_by(&recipes_list);
    }

    let mut recipe_results = instructions_list
        .grouped_by(&recipes_list)
        .into_iter()
        .zip(recipes_list)
        .zip(ingredients_grouped)
        .zip(tags_grouped)
        .map(|(((instruction, recipe), ingredient), tag)| {
            let mut rec = RecipeResultDTO::from(recipe);
            rec.instructions = instruction
                .into_iter()
                .map(|v| v.instruction)
                .collect::<Vec<String>>();
            rec.ingredients = ingredient
                .into_iter()
                .map(|(ri, i)| IngredientDTO {
                    unit: i.unit,
                    label: i.label,
                    amount: ri.amount,
                })
                .collect::<Vec<IngredientDTO>>();
            rec.tags = tag
                .into_iter()
                .map(|(_rt, t)| t.label)
                .collect::<Vec<String>>();
            rec
        })
        .collect::<Vec<RecipeResultDTO>>();

    if user_id.is_some() {
        recipe_results = recipe_results
            .into_iter()
            .zip(bookmarks_grouped)
            .zip(owned_grouped)
            .map(|((r, b), o)| {
                let mut recipe = r;
                recipe.bookmarked = if b.is_empty() {
                    Some(false)
                } else {
                    Some(true)
                };
                recipe.owned = if o.is_empty() {
                    Some(false)
                } else {
                    Some(true)
                };
                recipe
            })
            .collect::<Vec<RecipeResultDTO>>();
    }

    Ok(recipe_results)
}
