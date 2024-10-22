CREATE TABLE artists (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    image_path VARCHAR
);

CREATE INDEX idx_artist_name ON artists(name);
