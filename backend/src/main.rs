#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::{Build, Rocket};
use rocket::serde::json::Json;

use self::models::*;
use self::schema::recipes::dsl::*;

mod database;
mod models;
mod schema;

#[cfg(test)] mod tests;

#[get("/")]
fn index() -> Json<Vec<Recipes>> {
    let connection = &mut database::establish_connection();
    recipes.load::<Recipes>(connection).map(Json).expect("Error loading recipes")
}

#[post("/recipes", data = "<addrecipes>")]
pub fn addrecipes(addrecipes: Json<RecipesInput>) -> Json<Recipes> {
    use crate::schema::recipes;

    let connection = &mut database::establish_connection();
    diesel::insert_into(recipes::table)
        .values(addrecipes.into_inner())
        .execute(connection)
        .expect("Error adding recipe");
   

    Json(recipes::table
        .order(recipes::id.desc())
        .first(connection).unwrap()
    )
}


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, addrecipes])
}
