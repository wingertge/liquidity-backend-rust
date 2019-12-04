CREATE TABLE elections (
    id SERIAL PRIMARY KEY,
    created_by_id VARCHAR(255) NOT NULL REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    start_date DATETIME NOT NULL,
    end_date DATETIME NOT NULL,
    importance VARCHAR(255) NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TABLE choices (
    election_id SERIAL PRIMARY KEY REFERENCES elections(id) ON DELETE CASCADE ON UPDATE CASCADE,
    ballot_index SMALLINT,
    value VARCHAR(255)
);