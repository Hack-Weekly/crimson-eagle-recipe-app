use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::LogsDbConn;
use crate::models::*;
use crate::schema::recipes::dsl::*;

use super::get_recipe_elements;
use super::pagination;

/// List of recipes
///
/// Get all recipes from the database
#[utoipa::path(
    get,
    path = "/recipes?{page}&{per_page}",
    tag = "recipes",
    responses(
        (status = 200, description = "Recipes found succesfully", body = [PaginatedResult<RecipeResultDTO>]),
        (status = 500, description = "Error loading recipes"),
    ),
    params(
        ("page" = Option<i64>, Query, description = "Pagination: page number"),
        ("per_page" = Option<i64>, Query, description = "Pagination: results per page"),
    ),
)]
#[get("/recipes?<page>&<per_page>")]
pub async fn recipe(
    conn: LogsDbConn,
    page: Option<i64>,
    per_page: Option<i64>,
    key: Result<Jwt, NetworkResponse>,
) -> RecipeResponse<PaginatedResult<RecipeResultDTO>> {

    let total: i64 = match conn.run(|c| recipes.count().get_result(c)).await {
        Ok(c) => c,
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Database error while counting records.",
            ))
        }
    };

    let (current_page, per_page, offset) = pagination(page, per_page, total);

    let recipes_list = match conn.run(move |c| recipes
        .order(updated_at.desc())
        .offset(offset)
        .limit(per_page)
        .load::<Recipe>(c)).await
    {
        Ok(res) => res,
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Cannot read recipes from the database.",
            ))
        }
    };

    let user_id: Option<i32> = match key {
        Ok(k) => Some(k.claims.subject_id),
        Err(_) => None,
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

/// List of recipes, filtered
///
/// Get filtered and paginated list of recipes from the database
#[utoipa::path(
    get,
    path = "/recipes/search/{query}?{page}&{per_page}",
    tag = "recipes",
    responses(
        (status = 200, description = "Recipes found succesfully", body = [PaginatedResult<RecipeResultDTO>]),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("query" = String, Path, description = "Search term", example = "shrimp"),
        ("page" = Option<i64>, Query, description = "Pagination: page number"),
        ("per_page" = Option<i64>, Query, description = "Pagination: results per page"),
    )
)]
#[get("/recipes/search/<query>?<page>&<per_page>")]
pub async fn search(
    conn: LogsDbConn,
    query: String,
    page: Option<i64>,
    per_page: Option<i64>,
    key: Result<Jwt, NetworkResponse>,
) -> RecipeResponse<PaginatedResult<RecipeResultDTO>> {
    let total: i64 = match conn.run(|c| recipes.count().get_result(c)).await {
        Ok(c) => c,
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Database error while counting records.",
            ))
        }
    };

    let (current_page, per_page, offset) = pagination(page, per_page, total);

    let recipes_list = match conn.run(move |c| recipes
        .filter(title.ilike(format!("%{}%", query)))
        .order(updated_at.desc())
        .offset(offset)
        .limit(per_page)
        .load::<Recipe>(c)).await
    {
        Ok(res) => res,
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Cannot read recipes from the database.",
            ))
        }
    };

    let user_id: Option<i32> = match key {
        Ok(k) => Some(k.claims.subject_id),
        Err(_) => None,
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

/// Find recipe
///
/// Get recipe by id from the database
#[utoipa::path(
    get,
    path = "/recipes/{recipe_id}",
    tag = "recipes",
    responses(
        (status = 200, description = "Recipe found succesfully", body = RecipeResultDTO),
        (status = NOT_FOUND, description = "Recipe was not found"),
    ),
    params(
        ("recipe_id" = i32, description = "Recipe id", example = 2),
    )
)]
#[get("/recipes/<recipe_id>")]
pub async fn single_recipe(
    conn: LogsDbConn,
    recipe_id: i32,
    key: Result<Jwt, NetworkResponse>,
) -> RecipeResponse<RecipeResultDTO> {
    let recipes_list = match conn.run(move |c| recipes.find(recipe_id).load::<Recipe>(c)).await {
        Ok(res) => res,
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Cannot read recipes from the database.",
            ))
        }
    };

    let user_id: Option<i32> = match key {
        Ok(k) => Some(k.claims.subject_id),
        Err(_) => None,
    };

    match get_recipe_elements(recipes_list, conn, user_id).await {
        Ok(res) => match res.first() {
            Some(r) => RecipeResponse::Ok(Json(r.clone())),
            None => RecipeResponse::NotFound(String::from("The recipe was not found.")),
        },
        Err(_) => RecipeResponse::InternalServerError(String::from(
            "Cannot read recipes from the database.",
        )),
    }
}

/// Delete recipe
///
/// Find recipe by id and remove it from the database
#[utoipa::path(
    delete,
    path = "/recipes/{id}",
    tag = "recipes",
    responses(
        (status = 200, description = "Recipe deleted succesfully"),
        (status = NOT_FOUND, description = "Recipe was not found"),
    ),
    params(
        ("del_id" = i32, description = "Recipe id"),
    ),
    security(
        ("name" = ["Bearer"])
    ),
)]
#[delete("/recipes/<del_id>")]
pub async fn delete(
    conn: LogsDbConn, 
    del_id: i32,
    key: Result<Jwt, NetworkResponse>
) -> Result<Status, RecipeResponse<RecipeResultDTO>> {
    use crate::schema::recipes;

    match key {
        Ok(k) => k.claims.subject_id,
        Err(_) => {
            return Err(RecipeResponse::Unauthorized(String::from(
                "Please log in to be able to delete recipes.",
            )))
        }
    };
    // TODO: Fix deletion error: 
    // "update or delete on table "recipes" violates foreign key constraint "recipes_users_recipe_id_fkey" on table "recipes_users""
    let num_deleted = match conn.run(move |c| diesel::delete(recipes::table.find(del_id)).execute(c)).await {
        Ok(num) => num,
        Err(err) => {
            return Err(RecipeResponse::InternalServerError(format!(
                "Database error while deleting the recipe: {}",
                err.to_string()
            )))
        }
    };

    match num_deleted {
        0 => Err(RecipeResponse::NotFound(String::from("Recipe not found."))),
        _ => Ok(Status::NoContent),
    }
}
