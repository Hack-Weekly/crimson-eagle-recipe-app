#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::{Build, Rocket};
use rocket::serde::json::Json;

use self::models::*;
use self::schema::recipes::dsl::*;

mod database;
mod models;
mod schema;

#[cfg(test)] mod tests;

#[get("/recipes")]
fn recipe() -> Json<Vec<Recipes>> {
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

#[delete("/recipes/<del_id>")]
pub fn delete(del_id: i32) -> NoContent {
    use crate::schema::recipes;

    let connection = &mut database::establish_connection();
    diesel::delete(recipes::table.find(del_id))
        .execute(connection)
        .expect("Error deleting recipe");

    NoContent
}


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![recipe, addrecipes, delete])
}
