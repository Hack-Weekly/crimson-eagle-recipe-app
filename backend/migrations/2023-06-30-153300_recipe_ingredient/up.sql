CREATE TABLE recipe_ingredients (
    id SERIAL PRIMARY KEY,
    amount float,
    recipe_id INT,
    ingredient_id INT
)
