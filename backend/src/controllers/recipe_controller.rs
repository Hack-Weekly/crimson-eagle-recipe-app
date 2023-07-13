use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database;
use crate::models::*;
use crate::schema::recipes::dsl::*;
use crate::schema::*;

use super::get_recipe_elements;

/// List of recipes
///
/// Get all recipes from the database
#[utoipa::path(
    get,
    path = "/recipes",
    tag = "recipes",
    responses(
        (status = 200, description = "Recipes found succesfully", body = [Vec<RecipeResultDTO>]),
        (status = 500, description = "Error loading recipes"),
    )
)]
#[get("/recipes")]
pub fn recipe(key: Result<Jwt, NetworkResponse>) -> RecipeResponse<Vec<RecipeResultDTO>> {
    let connection = &mut database::establish_connection();

    let recipes_list = match recipes.load::<Recipe>(connection) {
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
        Ok(res) => RecipeResponse::Ok(Json(res)),
        Err(err) => RecipeResponse::InternalServerError(err),
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
        (status = 200, description = "Recipes found succesfully", body = [RecipeResultDTO]),
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
) -> Result<Json<Vec<Recipe>>, Status> {
    let connection = &mut database::establish_connection();

    let page_number = page.unwrap_or(1);
    let recipes_per_page = per_page.unwrap_or(10);

    let offset = (page_number - 1) * recipes_per_page;

    let results = recipes
        .filter(title.ilike(format!("%{}%", query)))
        .order(title.asc())
        .limit(recipes_per_page)
        .offset(offset)
        .load::<Recipe>(connection);

    match results {
        Ok(results) => Ok(Json(results)),
        Err(_) => Err(Status::InternalServerError),
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
pub fn single_recipe(recipe_id: i32) -> Result<Json<RecipeResultDTO>, Status> {
    let connection = &mut database::establish_connection();

    let inst = match instructions::table
        .filter(instructions::recipe_id.eq(recipe_id))
        .order(instructions::display_order.asc())
        .load::<Instruction>(connection)
    {
        Ok(res) => res,
        Err(_) => return Err(Status::InternalServerError),
    };

    match recipes.find(recipe_id).first::<Recipe>(connection) {
        Ok(res) => {
            let mut recipe_with_inst = RecipeResultDTO::from(res);
            recipe_with_inst.instructions = inst
                .iter()
                .map(|i| i.instruction.clone())
                .collect::<Vec<String>>();
            Ok(Json(recipe_with_inst))
        }
        Err(_) => Err(Status::NotFound),
    }
}

/// Add recipe
///
/// Create new recipe in the database
#[utoipa::path(
    post,
    path = "/recipes",
    request_body = RecipesInput,
    tag = "recipes",
    responses(
        (status = 201, description = "Recipe created succesfully", body = RecipeResultDTO),
        (status = 400, description = "Validation error"),
        (status = 500, description = "Internal Server Error"),
    ),
    security(
        ("name" = ["Bearer"])
    ),
)]
#[post("/recipes", data = "<addrecipes>")]
pub fn addrecipes(addrecipes: Json<RecipesInput>) -> Result<Json<Recipe>, Status> {
    use crate::schema::recipes;

    let connection = &mut database::establish_connection();
    match diesel::insert_into(recipes::table)
        .values(addrecipes.into_inner())
        .execute(connection)
    {
        Ok(_) => (),
        Err(_) => return Err(Status::InternalServerError),
    };

    match recipes::table
        .order(recipes::id.desc())
        .first::<Recipe>(connection)
    {
        Ok(recipe) => Ok(Json(recipe)),
        Err(_) => Err(Status::InternalServerError),
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
