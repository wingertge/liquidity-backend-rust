use chrono::{NaiveDateTime};

#[derive(Queryable, Debug)]
pub struct Election {
    pub id: u64,
    pub created_by_id: String,
    pub name: String,
    pub description: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub importance: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str
}