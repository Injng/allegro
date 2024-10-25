CREATE TABLE pieces (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    movements INTEGER,
    description TEXT
);

CREATE TABLE piece_composers (
    piece_id INTEGER NOT NULL,
    composer_id INTEGER NOT NULL,
    PRIMARY KEY (piece_id, composer_id),
    FOREIGN KEY (piece_id) REFERENCES pieces(id),
    FOREIGN KEY (composer_id) REFERENCES composers(id)
);

CREATE TABLE piece_songwriters (
    piece_id INTEGER NOT NULL,
    songwriter_id INTEGER NOT NULL,
    PRIMARY KEY (piece_id, songwriter_id),
    FOREIGN KEY (piece_id) REFERENCES pieces(id),
    FOREIGN KEY (songwriter_id) REFERENCES songwriters(id)
);

CREATE INDEX idx_piece_name ON pieces(name);
CREATE INDEX idx_piece_composers_piece ON piece_composers(piece_id);
CREATE INDEX idx_piece_composers_composer ON piece_composers(composer_id);
CREATE INDEX idx_piece_songwriters_piece ON piece_songwriters(piece_id);
CREATE INDEX idx_piece_songwriters_songwriter ON piece_songwriters(songwriter_id);
