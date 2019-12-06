CREATE TABLE ballots (
    user_id VARCHAR(255) NOT NULL,
    election_id uuid REFERENCES elections (id) ON DELETE CASCADE ON UPDATE CASCADE,
    value VARCHAR(255) NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    PRIMARY KEY (user_id, election_id)
);