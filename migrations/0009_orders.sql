CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    address_id INT NOT NULL,
    products VARCHAR NOT NULL,
    quantities VARCHAR NOT NULL,
    totals VARCHAR NOT NULL,
    statuses VARCHAR NOT NULL,
    order_dates VARCHAR NOT NULL
);