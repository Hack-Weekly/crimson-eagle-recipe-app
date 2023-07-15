// @generated automatically by Diesel CLI.

diesel::table! {
    ingredients (id) {
        id -> Int4,
        unit -> Nullable<Varchar>,
        label -> Varchar,
    }
}

diesel::table! {
    instructions (id) {
        id -> Int4,
        instruction -> Varchar,
        display_order -> Int4,
        recipe_id -> Int4,
    }
}

diesel::table! {
    recipe_ingredients (id) {
        id -> Int4,
        amount -> Nullable<Float4>,
        recipe_id -> Int4,
        ingredient_id -> Int4,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int4,
        title -> Varchar,
        servings -> Varchar,
        timer -> Nullable<Int2>,
        kcal -> Nullable<Int2>,
        carbs -> Nullable<Int2>,
        proteins -> Nullable<Int2>,
        fats -> Nullable<Int2>,
        image -> Nullable<Json>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    recipes_users (recipe_id, user_id) {
        recipe_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    bookmarks (recipe_id, user_id) {
        recipe_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        label -> Varchar,
        slug -> Varchar,
    }
}

diesel::table! {
    recipes_tags (recipe_id, tag_id) {
        recipe_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(recipe_ingredients -> ingredients (ingredient_id));
diesel::joinable!(instructions -> recipes (recipe_id));
diesel::joinable!(recipe_ingredients -> recipes (recipe_id));
diesel::joinable!(bookmarks -> recipes (recipe_id));
diesel::joinable!(bookmarks -> users (user_id));
diesel::joinable!(recipes_users -> recipes (recipe_id));
diesel::joinable!(recipes_users -> users (user_id));
diesel::joinable!(recipes_tags -> recipes (recipe_id));
diesel::joinable!(recipes_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    ingredients,
    instructions,
    recipe_ingredients,
    recipes,
    bookmarks,
    recipes_users,
    tags,
    recipes_tags,
    users,
);
