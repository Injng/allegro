CREATE TABLE releases (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    image_path VARCHAR
);

CREATE TABLE release_performers (
    release_id INTEGER NOT NULL,
    performer_id INTEGER NOT NULL,
    PRIMARY KEY (release_id, performer_id),
    FOREIGN KEY (release_id) REFERENCES releases(id),
    FOREIGN KEY (performer_id) REFERENCES performers(id)
);

CREATE INDEX idx_release_name ON releases(name);
CREATE INDEX idx_release_performers_release ON release_performers(release_id);
CREATE INDEX idx_release_performers_performer ON release_performers(performer_id);
