CREATE TABLE ballots (
    user_id VARCHAR(255) REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
    election_id SERIAL REFERENCES elections (id) ON DELETE CASCADE ON UPDATE CASCADE,
    value VARCHAR(255) NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    PRIMARY KEY (user_id, election_id)
);