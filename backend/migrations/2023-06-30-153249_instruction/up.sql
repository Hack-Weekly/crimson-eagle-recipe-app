CREATE TABLE instructions (
    id SERIAL PRIMARY KEY,
    instruction VARCHAR,
    display_order INT,
    recipe_id SERIAL REFERENCES recipes(id)
)
