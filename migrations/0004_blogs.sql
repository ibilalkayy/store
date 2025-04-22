CREATE TABLE IF NOT EXISTS blogs (
    id SERIAL PRIMARY KEY,
    titles VARCHAR NOT NULL,
    publish_dates VARCHAR NOT NULL,
    descriptions VARCHAR NOT NULL
);