use diesel::prelude::*;
use rocket::serde::json::Json;

use crate::database;
use crate::models::*;
use crate::schema::*;

use super::get_recipe_elements;
use super::pagination;

/// List of bookmarked recipes
///
/// Get all recipes that are bookmarked by the logged in user from the database.
#[utoipa::path(
    get,
    path = "/bookmarks?{page}&{per_page}",
    tag = "recipes",
    responses(
        (status = 200, description = "Bookmarked recipes found succesfully", body = [PaginatedResult<RecipeResultDTO>]),
        (status = 500, description = "Error loading recipes"),
    ),
    params(
        ("page" = Option<i64>, Query, description = "Pagination: page number"),
        ("per_page" = Option<i64>, Query, description = "Pagination: results per page"),
    ),
)]
#[get("/bookmarks?<page>&<per_page>")]
pub fn bookmarked_list(
    page: Option<i64>,
    per_page: Option<i64>,
    key: Result<Jwt, NetworkResponse>,
) -> RecipeResponse<PaginatedResult<RecipeResultDTO>> {
    let user_id: Option<i32> = match key {
        Ok(k) => Some(k.claims.subject_id),
        Err(_) => None,
    };

    if user_id.is_none() {
        return RecipeResponse::Unauthorized(String::from(
            "Please log in to see your bookmarked recipes.",
        ));
    }

    let connection = &mut database::establish_connection();

    let total: i64 = match recipes::table
        .inner_join(bookmarks::table)
        .filter(bookmarks::user_id.eq(user_id.unwrap()))
        .count()
        .get_result(connection)
    {
        Ok(c) => c,
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Database error while counting records.",
            ))
        }
    };

    let (current_page, per_page, offset) = pagination(page, per_page, total);

    let recipes_list = match recipes::table
        .inner_join(bookmarks::table)
        .filter(bookmarks::user_id.eq(user_id.unwrap()))
        .select(Recipe::as_select())
        .order(recipes::updated_at.desc())
        .offset(offset)
        .limit(per_page)
        .load::<Recipe>(connection)
    {
        Ok(res) => res,
        Err(err) => return RecipeResponse::InternalServerError(err.to_string()),
    };

    match get_recipe_elements(recipes_list, connection, user_id) {
        Ok(res) => {
            let paginated = PaginatedResult {
                records: res,
                total,
                current_page,
                per_page,
            };
            RecipeResponse::Ok(Json(paginated))
        }
        Err(_) => RecipeResponse::InternalServerError(String::from(
            "Cannot read recipes from the database.",
        )),
    }
}

/// Toggle bookmark on a recipe
///
/// Set or unset bookmark relationship with a recipe.
/// Returns bookmarked state after update (true if bookmarked, false if not).
#[utoipa::path(
    put,
    path = "/bookmarks/{recipe_id}",
    tag = "recipes",
    responses(
        (status = 200, description = "Bookmark relationship handled succesfully", body = bool),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("recipe_id" = i32, description = "Recipe id", example = 2),
    ),
    security(
        ("name" = ["Bearer"])
    ),
)]
#[put("/bookmarks/<recipe_id>")]
pub fn toggle_bookmark(recipe_id: i32, key: Result<Jwt, NetworkResponse>) -> RecipeResponse<bool> {
    let user_id: Option<i32> = match key {
        Ok(k) => Some(k.claims.subject_id),
        Err(_) => None,
    };

    if user_id.is_none() {
        return RecipeResponse::Unauthorized(String::from("Please log in to edit your bookmarks."));
    }
    let user_id = user_id.unwrap();

    let connection = &mut database::establish_connection();

    match bookmarks::table
        .find((recipe_id, user_id))
        .first::<Bookmark>(connection)
    {
        Ok(res) => {
            // remove
            match diesel::delete(&res).execute(connection) {
                Ok(_) => RecipeResponse::Ok(Json(false)),
                Err(_) => {
                    RecipeResponse::InternalServerError(String::from("Error removing bookmark."))
                }
            }
        }
        Err(diesel::NotFound) => {
            // add
            match diesel::insert_into(bookmarks::table)
                .values((
                    bookmarks::recipe_id.eq(recipe_id),
                    bookmarks::user_id.eq(user_id),
                ))
                .execute(connection)
            {
                Ok(_) => RecipeResponse::Ok(Json(true)),
                Err(_) => {
                    RecipeResponse::InternalServerError(String::from("Error adding bookmark."))
                }
            }
        }
        Err(_) => RecipeResponse::InternalServerError(String::from("Error finding bookmark.")),
    }
}
