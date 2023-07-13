CREATE TABLE recipes_tags (
    recipe_id INTEGER REFERENCES recipes(id),
    tag_id INTEGER REFERENCES tags(id),
    PRIMARY KEY(recipe_id, tag_id)
);
