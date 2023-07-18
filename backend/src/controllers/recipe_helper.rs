use diesel::prelude::*;

use crate::{models::*, schema::*, LogsDbConn};

pub async fn get_recipe_elements(
    recipes_list: Vec<Recipe>,
    conn: LogsDbConn,
    user_id: Option<i32>,
) -> Result<Vec<RecipeResultDTO>, String> {
    let recipe_results = conn
        .run(move |c| {
            // get instructions
            let instructions_list: Vec<Instruction> = match Instruction::belonging_to(&recipes_list)
                .order(instructions::display_order.asc())
                .load::<Instruction>(c)
            {
                Ok(res) => res,
                Err(_) => return Err(String::from("Cannot read instructions from the database.")),
            };

            // get ingredients
            let ingredients_list: Vec<(RecipeIngredient, Ingredient)> =
                match RecipeIngredient::belonging_to(&recipes_list)
                    .inner_join(ingredients::table)
                    .load::<(RecipeIngredient, Ingredient)>(c)
                {
                    Ok(res) => res,
                    Err(_) => {
                        return Err(String::from("Cannot read ingredients from the database."))
                    }
                };
            let ingredients_grouped = ingredients_list.grouped_by(&recipes_list);

            // get tags
            let tags_list: Vec<(RecipeTag, Tag)> = match RecipeTag::belonging_to(&recipes_list)
                .inner_join(tags::table)
                .load::<(RecipeTag, Tag)>(c)
            {
                Ok(res) => res,
                Err(_) => return Err(String::from("Cannot read tags from the database.")),
            };
            let tags_grouped = tags_list.grouped_by(&recipes_list);

            // if user is logged in, get bookmarks
            let mut bookmarks_grouped = Vec::<Vec<Bookmark>>::new();
            let mut owned_grouped = Vec::<Vec<RecipeUser>>::new();
            if let Some(id) = user_id {
                // TODO add ownership
                let bookmarks_list: Vec<Bookmark> = match Bookmark::belonging_to(&recipes_list)
                    .filter(bookmarks::user_id.eq(id))
                    .load::<Bookmark>(c)
                {
                    Ok(res) => res,
                    Err(_) => return Err(String::from("Cannot read bookmarks from the database.")),
                };
                bookmarks_grouped = bookmarks_list.grouped_by(&recipes_list);
                let owned_list: Vec<RecipeUser> = match RecipeUser::belonging_to(&recipes_list)
                    .filter(recipes_users::user_id.eq(id))
                    .load::<RecipeUser>(c)
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
                        .map(|(_rt, t)| TagDTO::from(t))
                        .collect::<Vec<TagDTO>>();
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
            Ok::<_, String>(recipe_results)
        })
        .await?;

    Ok(recipe_results)
}

pub fn pagination(page: Option<i64>, per_page: Option<i64>, total: i64) -> (i64, i64, i64) {
    let page_number = page.unwrap_or(1);
    let elements_per_page = per_page.unwrap_or(10);
    let per_page = if elements_per_page < 1 {
        10
    } else {
        elements_per_page
    };
    let max_page = (total - 1) / per_page + 1;
    let current_page = if page_number < 1 {
        1
    } else if page_number > max_page {
        max_page
    } else {
        page_number
    };
    let offset = elements_per_page * (current_page - 1);

    (current_page, per_page, offset)
}
