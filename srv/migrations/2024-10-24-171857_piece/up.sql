CREATE TABLE pieces (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    movements INTEGER,
    composer_id INTEGER NOT NULL,
    songwriter_id INTEGER,
    description TEXT,
    FOREIGN KEY (composer_id) REFERENCES composers(id),
    FOREIGN KEY (songwriter_id) REFERENCES songwriters(id)
);

CREATE INDEX idx_piece_name ON pieces(name);
CREATE INDEX idx_piece_composer ON pieces(composer_id);
CREATE INDEX idx_piece_songwriter ON pieces(songwriter_id);
