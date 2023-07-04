// @generated automatically by Diesel CLI.

diesel::table! {
    instructions (id) {
        id -> Int4,
        instruction -> Nullable<Varchar>,
        display_order -> Nullable<Int4>,
        recipe_id -> Nullable<Int4>,
    }
}

diesel::table! {
    recipe_ingredients (id) {
        id -> Int4,
        amount -> Nullable<Float8>,
        recipe_id -> Nullable<Int4>,
        ingredient_id -> Nullable<Int4>,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int4,
        title -> Varchar,
        servings -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    instructions,
    recipe_ingredients,
    recipes,
);
