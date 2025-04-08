CREATE TABLE IF NOT EXISTS payments (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    order_id INT NOT NULL,
    names VARCHAR NOT NULL,
    card_numbers INT NOT NULL UNIQUE,
    cvvs INT NOT NULL UNIQUE,
    amounts INT NOT NULL,
    payment_statuses VARCHAR NOT NULL
);