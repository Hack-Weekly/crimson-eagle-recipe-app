// @generated automatically by Diesel CLI.

diesel::table! {
    ingredient (id) {
        id -> Int4,
        unit -> Nullable<Varchar>,
        label -> Nullable<Varchar>,
    }
}

diesel::table! {
    instruction (id) {
        id -> Int4,
        instruction -> Nullable<Varchar>,
        display_order -> Nullable<Int4>,
        recipe_id -> Nullable<Int4>,
    }
}

diesel::table! {
    recipe (id) {
        id -> Int4,
        title -> Varchar,
        servings -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    recipe_ingredient (id) {
        id -> Int4,
        amount -> Nullable<Float8>,
        recipe_id -> Nullable<Int4>,
        ingredient_id -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ingredient,
    instruction,
    recipe,
    recipe_ingredient,
);
