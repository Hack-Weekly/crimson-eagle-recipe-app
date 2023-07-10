use crate::models::*;
use crate::schema::*;
use chrono;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = recipes)]
pub struct RecipesInput {
    pub title: String,
    pub servings: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RecipeResultDTO {
    pub id: i32,
    pub title: String,
    pub servings: String,
    pub instructions: Vec<String>,
    pub ingredients: Vec<IngredientDTO>,
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
            ingredients: Vec::<IngredientDTO>::new(),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

impl From<&Recipe> for RecipeResultDTO {
    fn from(r: &Recipe) -> Self {
        Self {
            id: r.id,
            title: r.title.clone(),
            servings: r.servings.clone(),
            instructions: Vec::<String>::new(),
            ingredients: Vec::<IngredientDTO>::new(),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct RecipePutDTO {
    #[validate(length(max = 120))]
    pub title: Option<String>,
    #[validate(length(max = 120))]
    pub servings: Option<String>,
    pub instructions: Option<Vec<String>>,
    #[validate]
    pub ingredients: Option<Vec<IngredientDTO>>,
}

#[derive(Insertable, Associations, Debug)] //PartialEq
#[diesel(belongs_to(Recipe))]
#[diesel(table_name = instructions)]
pub struct InstructionInsert {
    pub instruction: String,
    pub display_order: i32,
    pub recipe_id: i32,
}

#[derive(Insertable, Debug)] //PartialEq
#[diesel(table_name = ingredients)]
pub struct IngredientInsert {
    pub unit: Option<String>,
    pub label: String,
}

#[derive(Insertable, Associations, Debug)]
#[diesel(belongs_to(Recipe))]
#[diesel(belongs_to(Ingredient))]
#[diesel(table_name = recipe_ingredients)]
pub struct RecipeIngredientInsert {
    pub amount: Option<f32>,
    pub recipe_id: i32,
    pub ingredient_id: i32,
}

#[derive(Serialize, Deserialize, Validate, Clone)]
#[serde(crate = "rocket::serde")]
pub struct IngredientDTO {
    #[validate(length(max = 120))]
    pub unit: Option<String>,
    #[validate(length(max = 120))]
    pub label: String,
    #[validate(range(min = 0.0, max = 100000.0))]
    pub amount: Option<f32>,
}

impl From<(RecipeIngredient, Ingredient)> for IngredientDTO {
    fn from(r: (RecipeIngredient, Ingredient)) -> Self {
        Self {
            unit: r.1.unit,
            label: r.1.label,
            amount: r.0.amount,
        }
    }
}
