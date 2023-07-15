CREATE TABLE bookmarks (
    recipe_id INTEGER REFERENCES recipes(id),
    user_id INTEGER REFERENCES users(id),
    PRIMARY KEY(recipe_id, user_id)
);
