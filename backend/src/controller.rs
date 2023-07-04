use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use validator::Validate;

use crate::database;
use crate::models::*;
use crate::schema::recipes::dsl::*;
use crate::schema::users::dsl::*;
use crate::schema::*;
use bcrypt::{hash, verify, DEFAULT_COST};

#[get("/recipes")]
pub fn recipe() -> Json<Vec<Recipe>> {
    let connection = &mut database::establish_connection();
    recipes
        .load::<Recipe>(connection)
        .map(Json)
        .expect("Error loading recipes")
}

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

#[post("/register", data = "<new_user>")]
pub fn register(new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    new_user.validate().map_err(|_err| Status::BadRequest)?;

    let connection = &mut database::establish_connection();
    let hashed_password = hash(new_user.password, DEFAULT_COST).unwrap();
    let new_user = NewUser {
        username: &new_user.username,
        password: &hashed_password,
    };

    match diesel::insert_into(users)
        .values(new_user)
        .get_result::<User>(connection)
    {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", data = "<login_user>")]
pub fn login(login_user: Json<LoginUser>) -> Result<Json<User>, Status> {
    login_user.validate().map_err(|_err| Status::BadRequest)?;

    let connection = &mut database::establish_connection();
    let result = users
        .filter(username.eq(&login_user.username))
        .first::<User>(connection);

    match result {
        Ok(user) => {
            if verify(&login_user.password, &user.password).unwrap() {
                Ok(Json(user))
            } else {
                Err(Status::Unauthorized)
            }
        }
        Err(_) => Err(Status::NotFound),
    }
}
