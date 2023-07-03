CREATE TABLE ingredients (
    id SERIAL PRIMARY KEY REFERENCES recipe_ingredients(id),
    unit VARCHAR,
    label VARCHAR
)
