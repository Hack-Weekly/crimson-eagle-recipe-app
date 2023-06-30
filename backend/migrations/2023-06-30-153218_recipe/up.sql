CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    servings VARCHAR NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
)
