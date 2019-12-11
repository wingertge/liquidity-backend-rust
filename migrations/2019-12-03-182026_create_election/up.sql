CREATE TABLE elections (
    id uuid PRIMARY KEY,
    created_by_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description text NOT NULL,
    start_date timestamptz NOT NULL,
    end_date timestamptz NOT NULL,
    importance VARCHAR(255) NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);

CREATE TABLE choices (
    id uuid,
    election_id uuid REFERENCES elections(id) ON DELETE CASCADE ON UPDATE CASCADE,
    ballot_index SMALLINT NOT NULL,
    value VARCHAR(255) NOT NULL,
    PRIMARY KEY (id, election_id)
);