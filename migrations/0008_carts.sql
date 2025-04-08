CREATE TABLE IF NOT EXISTS carts (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    product_id INT NOT NULL,
    products VARCHAR NOT NULL,
    quantities VARCHAR NOT NULL,
    totals VARCHAR NOT NULL,
    grand_total VARCHAR NOT NULL,
    discount_codes VARCHAR NOT NULL
);