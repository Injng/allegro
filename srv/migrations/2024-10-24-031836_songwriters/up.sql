CREATE TABLE songwriters (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    image_path VARCHAR
);

CREATE INDEX idx_songwriter_name ON songwriters(name);
