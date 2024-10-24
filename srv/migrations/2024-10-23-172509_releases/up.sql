CREATE TABLE releases (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    performer_id INTEGER NOT NULL,
    description TEXT,
    image_path VARCHAR,
    FOREIGN KEY (performer_id) REFERENCES performers(id)
);

CREATE INDEX idx_release_name ON releases(name);
CREATE INDEX idx_release_performer ON releases(performer_id);
