use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database;
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
pub fn recipe(
    page: Option<i64>,
    per_page: Option<i64>,
    key: Result<Jwt, NetworkResponse>,
) -> RecipeResponse<PaginatedResult<RecipeResultDTO>> {
    let connection = &mut database::establish_connection();

    let total: i64 = match recipes.count().get_result(connection) {
        Ok(c) => c,
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Database error while counting records.",
            ))
        }
    };

    let (current_page, per_page, offset) = pagination(page, per_page, total);

    let recipes_list = match recipes
        .order(updated_at.desc())
        .offset(offset)
        .limit(per_page)
        .load::<Recipe>(connection)
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
pub fn search(
    query: String,
    page: Option<i64>,
    per_page: Option<i64>,
    key: Result<Jwt, NetworkResponse>,
) -> RecipeResponse<PaginatedResult<RecipeResultDTO>> {
    let connection = &mut database::establish_connection();

    let total: i64 = match recipes.count().get_result(connection) {
        Ok(c) => c,
        Err(_) => {
            return RecipeResponse::InternalServerError(String::from(
                "Database error while counting records.",
            ))
        }
    };

    let (current_page, per_page, offset) = pagination(page, per_page, total);

    let recipes_list = match recipes
        .filter(title.ilike(format!("%{}%", query)))
        .order(updated_at.desc())
        .offset(offset)
        .limit(per_page)
        .load::<Recipe>(connection)
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
pub fn single_recipe(
    recipe_id: i32,
    key: Result<Jwt, NetworkResponse>,
) -> RecipeResponse<RecipeResultDTO> {
    let connection = &mut database::establish_connection();

    let recipes_list = match recipes.find(recipe_id).load::<Recipe>(connection) {
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

    match get_recipe_elements(recipes_list, connection, user_id) {
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
pub fn delete(del_id: i32) -> Result<Status, Status> {
    use crate::schema::recipes;

    let connection = &mut database::establish_connection();
    let num_deleted = diesel::delete(recipes::table.find(del_id))
        .execute(connection)
        .map_err(|_| Status::InternalServerError)?;

    match num_deleted {
        0 => Err(Status::NotFound),
        _ => Ok(Status::Ok),
    }
}
