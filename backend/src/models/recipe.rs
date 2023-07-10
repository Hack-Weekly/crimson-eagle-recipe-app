use crate::schema::*;
use chrono;
use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Queryable, Identifiable, Debug, Serialize)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub servings: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Queryable, Identifiable, Clone, Associations, Debug)] //PartialEq
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
