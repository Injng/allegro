CREATE TABLE composers (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    image_path VARCHAR
);

CREATE INDEX idx_composer_name ON composers(name);
