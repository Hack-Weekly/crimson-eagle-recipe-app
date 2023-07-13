use crate::models::*;
use crate::schema::*;
use chrono;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Serialize)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub servings: String,
    pub timer: Option<i16>,
    pub kcal: Option<i16>,
    pub carbs: Option<i16>,
    pub proteins: Option<i16>,
    pub fats: Option<i16>,
    pub image: Option<serde_json::Value>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Identifiable, Clone, Associations, PartialEq, Debug)] //PartialEq
#[diesel(belongs_to(Recipe))]
pub struct Instruction {
    pub id: i32,
    pub instruction: String,
    pub display_order: i32,
    pub recipe_id: i32,
}

#[derive(Queryable, Identifiable, Associations, Clone, Debug)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(Ingredient))]
#[diesel(table_name = recipe_ingredients)]
pub struct RecipeIngredient {
    pub id: i32,
    pub amount: Option<f32>,
    pub recipe_id: i32,
    pub ingredient_id: i32,
}

#[derive(Queryable, Identifiable, Clone, Debug)]
#[diesel(table_name = ingredients)]
pub struct Ingredient {
    pub id: i32,
    pub unit: Option<String>,
    pub label: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub label: String,
    pub slug: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = recipes_tags)]
#[diesel(primary_key(recipe_id, tag_id))]
pub struct RecipeTag {
    pub recipe_id: i32,
    pub tag_id: i32,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(User))]
#[diesel(table_name = recipes_users)]
#[diesel(primary_key(recipe_id, user_id))]
pub struct RecipeUser {
    pub recipe_id: i32,
    pub user_id: i32,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(User))]
#[diesel(table_name = bookmarks)]
#[diesel(primary_key(recipe_id, user_id))]
pub struct Bookmark {
    pub recipe_id: i32,
    pub user_id: i32,
}

#[derive(Responder, Debug)]
pub enum RecipeResponse<T> {
    #[response(status = 200)]
    Ok(Json<T>),
    #[response(status = 201)]
    Created(Json<T>),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    InternalServerError(String),
}
