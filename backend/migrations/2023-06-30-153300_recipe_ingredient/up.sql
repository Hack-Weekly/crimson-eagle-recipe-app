CREATE TABLE recipe_ingredients (
    id SERIAL PRIMARY KEY,
    amount float,
    recipe_id SERIAL REFERENCES recipes(id),
    ingredient_id INT
)
