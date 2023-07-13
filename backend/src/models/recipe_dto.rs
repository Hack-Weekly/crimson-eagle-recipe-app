use crate::models::*;
use crate::schema::*;
use chrono;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use slug::slugify;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Insertable, Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = recipes)]
pub struct RecipesInput {
    #[schema(example = "Veggie Pizza")]
    pub title: String,
    #[schema(example = "4")]
    pub servings: String,
    #[schema(example = 90)]
    pub timer: Option<i16>,
    #[schema(example = 130)]
    pub kcal: Option<i16>,
    #[schema(example = 25)]
    pub carbs: Option<i16>,
    #[schema(example = 3)]
    pub proteins: Option<i16>,
    #[schema(example = 2)]
    pub fats: Option<i16>,
    #[schema(example = json!({
        public_id: "vwgblqojf6tuhezhddy9",
        width: 800,
        height: 533,
        format: "jpg",
        resource_type: "image",
        url: "http://res.cloudinary.com/dgxfgifvw/image/upload/v1688953484/foodly/vwgblqojf6tuhezhddy9.jpg",
        secure_url: "https://res.cloudinary.com/dgxfgifvw/image/upload/v1688953484/foodly/vwgblqojf6tuhezhddy9.jpg",
    }))]
    pub image: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UploadResult {
    pub public_id: String,
    pub width: i32,
    pub height: i32,
    pub format: String,
    pub resource_type: String,
    pub url: String,
    pub secure_url: String,
}

#[derive(Serialize, ToSchema, Debug)]
#[serde(crate = "rocket::serde")]
pub struct RecipeResultDTO {
    #[schema(example = 123)]
    pub id: i32,
    #[schema(example = "Veggie Pizza")]
    pub title: String,
    #[schema(example = "4")]
    pub servings: String,
    #[schema(example = 90)]
    pub timer: Option<i16>,
    #[schema(example = 130)]
    pub kcal: Option<i16>,
    #[schema(example = 25)]
    pub carbs: Option<i16>,
    #[schema(example = 3)]
    pub proteins: Option<i16>,
    #[schema(example = 2)]
    pub fats: Option<i16>,
    #[schema(example = json!({
        public_id: "vwgblqojf6tuhezhddy9",
        width: 800,
        height: 533,
        format: "jpg",
        resource_type: "image",
        url: "http://res.cloudinary.com/dgxfgifvw/image/upload/v1688953484/foodly/vwgblqojf6tuhezhddy9.jpg",
        secure_url: "https://res.cloudinary.com/dgxfgifvw/image/upload/v1688953484/foodly/vwgblqojf6tuhezhddy9.jpg",
    }))]
    pub image: Option<serde_json::Value>,
    #[schema(example = json!(vec!["Open pizza's box", "Put pizza into oven.", "Wait.", "Get pizza out of the oven."]))]
    pub instructions: Vec<String>,
    #[schema(example = json!(vec![
        IngredientDTO { unit: Some(String::from("kg")), label: String::from("flour"), amount: Some(0.5)},
        IngredientDTO { unit: Some(String::from("dl")), label: String::from("water"), amount: Some(3.5)},
        IngredientDTO { unit: Some(String::from("g")), label: String::from("salt"), amount: Some(10.0)}
    ]))]
    pub ingredients: Vec<IngredientDTO>,
    #[schema(example = json!(Some(chrono::Utc::now())))]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[schema(example = json!(Some(chrono::Utc::now())))]
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub tags: Vec<String>,
    pub bookmarked: Option<bool>,
    pub owned: Option<bool>,
}

impl From<Recipe> for RecipeResultDTO {
    fn from(r: Recipe) -> Self {
        Self {
            id: r.id,
            title: r.title,
            servings: r.servings,
            timer: r.timer,
            kcal: r.kcal,
            carbs: r.carbs,
            proteins: r.proteins,
            fats: r.fats,
            image: r.image,
            instructions: Vec::<String>::new(),
            ingredients: Vec::<IngredientDTO>::new(),
            created_at: r.created_at,
            updated_at: r.updated_at,
            tags: Vec::<String>::new(),
            bookmarked: None,
            owned: None,
        }
    }
}

impl From<&Recipe> for RecipeResultDTO {
    fn from(r: &Recipe) -> Self {
        Self {
            id: r.id,
            title: r.title.clone(),
            servings: r.servings.clone(),
            timer: r.timer,
            kcal: r.kcal,
            carbs: r.carbs,
            proteins: r.proteins,
            fats: r.fats,
            image: r.image.clone(),
            instructions: Vec::<String>::new(),
            ingredients: Vec::<IngredientDTO>::new(),
            created_at: r.created_at,
            updated_at: r.updated_at,
            tags: Vec::<String>::new(),
            bookmarked: None,
            owned: None,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct RecipePostDTO {
    #[schema(example = "Veggie Pizza")]
    #[validate(length(max = 120))]
    pub title: String,
    #[schema(example = "4")]
    #[validate(length(max = 120))]
    pub servings: String,
    #[schema(example = 90)]
    #[validate(range(min = 0, max = 30000))]
    pub timer: Option<i16>,
    #[schema(example = 130)]
    #[validate(range(min = 0, max = 30000))]
    pub kcal: Option<i16>,
    #[schema(example = 25)]
    #[validate(range(min = 0, max = 30000))]
    pub carbs: Option<i16>,
    #[schema(example = 3)]
    #[validate(range(min = 0, max = 30000))]
    pub proteins: Option<i16>,
    #[schema(example = 2)]
    #[validate(range(min = 0, max = 30000))]
    pub fats: Option<i16>,
    #[schema(example = json!({
        public_id: "vwgblqojf6tuhezhddy9",
        width: 800,
        height: 533,
        format: "jpg",
        resource_type: "image",
        url: "http://res.cloudinary.com/dgxfgifvw/image/upload/v1688953484/foodly/vwgblqojf6tuhezhddy9.jpg",
        secure_url: "https://res.cloudinary.com/dgxfgifvw/image/upload/v1688953484/foodly/vwgblqojf6tuhezhddy9.jpg",
    }))]
    pub image: Option<serde_json::Value>,
    #[schema(example = json!(Some(vec!["Open pizza's box", "Put pizza into oven.", "Wait.", "Get pizza out of the oven."])))]
    pub instructions: Option<Vec<String>>,
    #[schema(example = json!(Some(vec![
        IngredientDTO { unit: Some(String::from("kg")), label: String::from("flour"), amount: Some(0.5)},
        IngredientDTO { unit: Some(String::from("dl")), label: String::from("water"), amount: Some(3.5)},
        IngredientDTO { unit: Some(String::from("g")), label: String::from("salt"), amount: Some(10.0)}
    ])))]
    #[validate]
    pub ingredients: Option<Vec<IngredientDTO>>,
    #[schema(example = json!(Some(vec!["vegan", "vegetarian"])))]
    tags: Option<Vec<String>>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct RecipePutDTO {
    #[schema(example = "Veggie Pizza")]
    #[validate(length(max = 120))]
    pub title: Option<String>,
    #[schema(example = "4")]
    #[validate(length(max = 120))]
    pub servings: Option<String>,
    #[schema(example = 90)]
    #[validate(range(min = 0, max = 30000))]
    pub timer: Option<i16>,
    #[schema(example = 130)]
    #[validate(range(min = 0, max = 30000))]
    pub kcal: Option<i16>,
    #[schema(example = 25)]
    #[validate(range(min = 0, max = 30000))]
    pub carbs: Option<i16>,
    #[schema(example = 3)]
    #[validate(range(min = 0, max = 30000))]
    pub proteins: Option<i16>,
    #[schema(example = 2)]
    #[validate(range(min = 0, max = 30000))]
    pub fats: Option<i16>,
    #[schema(example = json!({
        public_id: "vwgblqojf6tuhezhddy9",
        width: 800,
        height: 533,
        format: "jpg",
        resource_type: "image",
        url: "http://res.cloudinary.com/dgxfgifvw/image/upload/v1688953484/foodly/vwgblqojf6tuhezhddy9.jpg",
        secure_url: "https://res.cloudinary.com/dgxfgifvw/image/upload/v1688953484/foodly/vwgblqojf6tuhezhddy9.jpg",
    }))]
    pub image: Option<serde_json::Value>,
    #[schema(example = json!(Some(vec!["Open pizza's box", "Put pizza into oven.", "Wait.", "Get pizza out of the oven."])))]
    pub instructions: Option<Vec<String>>,
    #[schema(example = json!(Some(vec![
        IngredientDTO { unit: Some(String::from("kg")), label: String::from("flour"), amount: Some(0.5)},
        IngredientDTO { unit: Some(String::from("dl")), label: String::from("water"), amount: Some(3.5)},
        IngredientDTO { unit: Some(String::from("g")), label: String::from("salt"), amount: Some(10.0)}
    ])))]
    #[validate]
    pub ingredients: Option<Vec<IngredientDTO>>,
    #[schema(example = json!(Some(vec!["vegan", "vegetarian"])))]
    tags: Option<Vec<String>>,
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

#[derive(Serialize, Deserialize, Validate, Clone, ToSchema, Debug)]
#[serde(crate = "rocket::serde")]
pub struct IngredientDTO {
    #[schema(example = "kg")]
    #[validate(length(max = 120))]
    pub unit: Option<String>,
    #[schema(example = "all-purpose flour")]
    #[validate(length(max = 120))]
    pub label: String,
    #[schema(example = 0.5)]
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

#[derive(Deserialize, Validate, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct TagPostDTO {
    #[schema(example = "gluten free")]
    #[validate(length(max = 120))]
    pub label: String,
}

impl From<TagPostDTO> for TagDTO {
    fn from(t: TagPostDTO) -> Self {
        Self {
            label: t.label.clone(),
            slug: slugify(t.label),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Validate, ToSchema)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = tags)]
pub struct TagDTO {
    #[schema(example = "gluten free")]
    #[validate(length(max = 120))]
    pub label: String,
    #[schema(example = "gluten-free")]
    #[validate(length(max = 120))]
    pub slug: String,
}

impl From<Tag> for TagDTO {
    fn from(t: Tag) -> Self {
        Self {
            label: t.label,
            slug: t.slug,
        }
    }
}
