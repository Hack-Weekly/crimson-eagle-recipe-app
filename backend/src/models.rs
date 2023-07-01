use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use crate::schema::recipes;
use chrono;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Recipes {
    pub id: i32,
    pub title: String,
    pub servings: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>
}


#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = recipes)]
pub struct RecipesInput {
    pub title: String,
    pub servings: String
}
