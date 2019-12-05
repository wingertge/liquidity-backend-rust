use chrono::{DateTime, Utc};
use uuid::Uuid;
use core::fmt;
use failure::_core::fmt::{Formatter, Error};
use failure::_core::str::FromStr;

#[derive(juniper::GraphQLInputObject)]
pub struct PermissionSet {
    pub view_roles: Option<Vec<String>>,
    pub vote_roles: Option<Vec<String>>,
    pub edit_roles: Option<Vec<String>>,
    pub admin_roles: Option<Vec<String>>
}

impl Default for PermissionSet {
    fn default() -> Self {
        PermissionSet {
            view_roles: Some(vec!["@all".to_string()]),
            vote_roles: Some(vec!["@all".to_string()]),
            edit_roles: Some(Vec::new()),
            admin_roles: Some(Vec::new())
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct User {
    pub username: String
}

#[derive(juniper::GraphQLEnum, Debug)]
pub enum Importance {
    Important,
    Regular,
    Minor
}

impl fmt::Display for Importance {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Importance {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Important" => Ok(Importance::Important),
            "Regular" => Ok(Importance::Regular),
            "Minor" => Ok(Importance::Minor),
            _ => Err(())
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Election {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub choices: Vec<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub importance: Importance
}

#[derive(juniper::GraphQLInputObject)]
pub struct ElectionInput {
    pub name: String,
    pub permissions: Option<PermissionSet>,
    pub description: Option<String>,
    pub choices: Option<Vec<String>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub importance: Option<Importance>
}
