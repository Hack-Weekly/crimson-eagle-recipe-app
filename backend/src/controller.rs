use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::http::Status;

use crate::database;
use crate::models::*;
use crate::schema::recipes::dsl::*;


#[get("/recipes")]
pub fn recipe() -> Json<Vec<Recipes>> {
    let connection = &mut database::establish_connection();
    recipes.load::<Recipes>(connection).map(Json).expect("Error loading recipes")
}

#[get("/recipes/search/<query>?<page>&<per_page>")]
pub fn search(query: String, page: Option<i64>, per_page: Option<i64>) -> Result<Json<Vec<Recipes>>, Status> {
    let connection = &mut database::establish_connection();
    
    let page_number = page.unwrap_or(1);
    let recipes_per_page = per_page.unwrap_or(10);
    
    let offset = (page_number - 1) * recipes_per_page;

    let results = recipes
        .filter(title.ilike(format!("%{}%", query)))
        .order(title.asc())
        .limit(recipes_per_page)
        .offset(offset)
        .load::<Recipes>(connection);

    match results {
        Ok(results) => Ok(Json(results)),
        Err(_) => Err(Status::InternalServerError),
    }
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
