CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    names VARCHAR NOT NULL,
    emails VARCHAR NOT NULL UNIQUE,
    passwords VARCHAR NOT NULL,
    roles VARCHAR NOT NULL
);