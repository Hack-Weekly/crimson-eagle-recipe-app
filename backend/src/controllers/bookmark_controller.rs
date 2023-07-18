use diesel::prelude::*;
use rocket::serde::json::Json;

use crate::LogsDbConn;
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
pub async fn bookmarked_list(
    conn: LogsDbConn,
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

    let total: i64 = match conn.run(move |c| recipes::table
        .inner_join(bookmarks::table)
        .filter(bookmarks::user_id.eq(user_id.unwrap()))
        .count()
        .get_result(c)).await
    {
        Ok(c) => c,
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Database error while counting records.",
            ))
        }
    };

    let (current_page, per_page, offset) = pagination(page, per_page, total);

    let recipes_list = match conn.run(move |c| recipes::table
        .inner_join(bookmarks::table)
        .filter(bookmarks::user_id.eq(user_id.unwrap()))
        .select(Recipe::as_select())
        .order(recipes::updated_at.desc())
        .offset(offset)
        .limit(per_page)
        .load::<Recipe>(c)).await
    {
        Ok(res) => res,
        Err(err) => return RecipeResponse::InternalServerError(err.to_string()),
    };

    match get_recipe_elements(recipes_list, conn, user_id).await {
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
pub async fn toggle_bookmark(conn: LogsDbConn, recipe_id: i32, key: Result<Jwt, NetworkResponse>) -> RecipeResponse<bool> {
    let user_id: Option<i32> = match key {
        Ok(k) => Some(k.claims.subject_id),
        Err(_) => None,
    };

    if user_id.is_none() {
        return RecipeResponse::Unauthorized(String::from("Please log in to edit your bookmarks."));
    }
    let user_id = user_id.unwrap();

    match conn.run(move |c| bookmarks::table
        .find((recipe_id, user_id))
        .first::<Bookmark>(c)).await
    {
        Ok(res) => {
            // remove
            match conn.run(move |c| diesel::delete(&res).execute(c)).await {
                Ok(_) => RecipeResponse::Ok(Json(false)),
                Err(_) => {
                    RecipeResponse::InternalServerError(String::from("Error removing bookmark."))
                }
            }
        }
        Err(diesel::NotFound) => {
            // add
            match conn.run(move |c| diesel::insert_into(bookmarks::table)
                .values((
                    bookmarks::recipe_id.eq(recipe_id),
                    bookmarks::user_id.eq(user_id),
                ))
                .execute(c)).await
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
