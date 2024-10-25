CREATE TABLE recordings (
    id SERIAL PRIMARY KEY,
    piece_id INTEGER NOT NULL,
    release_id INTEGER NOT NULL,
    track_number INTEGER UNIQUE NOT NULL,
    file_path VARCHAR,
    FOREIGN KEY (piece_id) REFERENCES pieces(id),
    FOREIGN KEY (release_id) REFERENCES releases(id)
);

CREATE TABLE recording_performers (
    recording_id INTEGER NOT NULL,
    performer_id INTEGER NOT NULL,
    PRIMARY KEY (recording_id, performer_id),
    FOREIGN KEY (recording_id) REFERENCES recordings(id),
    FOREIGN KEY (performer_id) REFERENCES performers(id)
);

CREATE INDEX idx_recording_piece ON recordings(piece_id);
CREATE INDEX idx_recording_release ON recordings(release_id);
CREATE INDEX idx_recording_performers_recording ON recording_performers(recording_id);
CREATE INDEX idx_recording_performers_performer ON recording_performers(performer_id);
