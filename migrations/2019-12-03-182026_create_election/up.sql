CREATE TABLE elections (
    id uuid PRIMARY KEY,
    created_by_id VARCHAR(255) NOT NULL REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
    name VARCHAR(255) NOT NULL,
    description text NOT NULL,
    start_date timestamptz NOT NULL,
    end_date timestamptz NOT NULL,
    importance VARCHAR(255) NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);

CREATE TABLE choices (
    election_id uuid PRIMARY KEY REFERENCES elections(id) ON DELETE CASCADE ON UPDATE CASCADE,
    ballot_index SMALLINT NOT NULL,
    value VARCHAR(255) NOT NULL
);