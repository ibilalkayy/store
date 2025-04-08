CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    category_id INT NOT NULL,
    names VARCHAR NOT NULL,
    prices VARCHAR NOT NULL,
    images VARCHAR NOT NULL
);