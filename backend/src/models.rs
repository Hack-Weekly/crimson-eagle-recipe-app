use crate::schema::{instructions, recipes, users};
use chrono;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub servings: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = recipes)]
pub struct RecipesInput {
    pub title: String,
    pub servings: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Recipe))]
pub struct Instruction {
    pub id: i32,
    pub instruction: String,
    pub display_order: i32,
    pub recipe_id: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeResultDTO {
    pub id: i32,
    pub title: String,
    pub servings: String,
    pub instructions: Vec<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl From<Recipe> for RecipeResultDTO {
    fn from(r: Recipe) -> Self {
        Self {
            id: r.id,
            title: r.title,
            servings: r.servings,
            instructions: Vec::<String>::new(),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: &'a str,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: &'a str,
}

#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct LoginUser {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}