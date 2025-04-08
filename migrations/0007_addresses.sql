CREATE TABLE IF NOT EXISTS addresses (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    first_names VARCHAR NOT NULL,
    last_names VARCHAR NOT NULL,
    addresses VARCHAR NOT NULL,
    town_cities VARCHAR NOT NULL,
    countries VARCHAR NOT NULL,
    zip_codes INT NOT NULL,
    phone_numbers INT NOT NULL,
    emails VARCHAR NOT NULL,
    passwords VARCHAR NOT NULL,
    notes VARCHAR NOT NULL
);