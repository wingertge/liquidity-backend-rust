table! {
    ballots (user_id, election_id) {
        user_id -> Varchar,
        election_id -> Uuid,
        value -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    choices (election_id) {
        election_id -> Uuid,
        ballot_index -> Int2,
        value -> Varchar,
    }
}

table! {
    elections (id) {
        id -> Uuid,
        created_by_id -> Varchar,
        name -> Varchar,
        description -> Text,
        start_date -> Timestamptz,
        end_date -> Timestamptz,
        importance -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(ballots -> elections (election_id));
joinable!(ballots -> users (user_id));
joinable!(choices -> elections (election_id));
joinable!(elections -> users (created_by_id));

allow_tables_to_appear_in_same_query!(
    ballots,
    choices,
    elections,
    users,
);
