CREATE TABLE releases (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    artist_id INTEGER NOT NULL,
    description TEXT,
    image_path VARCHAR,
    FOREIGN KEY (artist_id) REFERENCES artists(id)
);

CREATE INDEX idx_release_name ON releases(name);
CREATE INDEX idx_release_artist ON releases(artist_id);
