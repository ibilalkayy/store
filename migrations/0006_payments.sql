CREATE TABLE IF NOT EXISTS payments (
    id SERIAL PRIMARY KEY,
    names VARCHAR NOT NULL,
    card_numbers INT NOT NULL UNIQUE,
    cvvs INT NOT NULL UNIQUE,
    amounts FLOAT4 NOT NULL,
    payment_statuses VARCHAR NOT NULL
);