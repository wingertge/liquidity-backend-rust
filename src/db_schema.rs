table! {
    ballots (user_id, election_id) {
        user_id -> Varchar,
        election_id -> Unsigned<Bigint>,
        value -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    choices (election_id) {
        election_id -> Unsigned<Bigint>,
        ballot_index -> Nullable<Smallint>,
        value -> Nullable<Varchar>,
    }
}

table! {
    elections (id) {
        id -> Unsigned<Bigint>,
        created_by_id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        start_date -> Datetime,
        end_date -> Datetime,
        importance -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    ballots,
    choices,
    elections,
    users,
);
