CREATE TABLE performers (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    image_path VARCHAR
);

CREATE INDEX idx_performer_name ON performers(name);
