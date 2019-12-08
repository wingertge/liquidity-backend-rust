use chrono::{DateTime, Utc};
use uuid::Uuid;
use core::fmt;
use failure::_core::fmt::{Formatter, Error};
use failure::_core::str::FromStr;

#[derive(juniper::GraphQLInputObject)]
/// A set of roles allowed to interact with this election.
/// Base permissions are always required in addition to these.
pub struct PermissionSet {
    /// The roles allowed to view the election and its results. Defaults to all.
    pub view_roles: Option<Vec<String>>,
    /// The roles allowed to vote in the election. Defaults to all
    pub vote_roles: Option<Vec<String>>,
    /// The roles allowed to edit the election metadata, but not the election timing or choices.
    /// Defaults to none
    pub edit_roles: Option<Vec<String>>,
    /// The roles allowed full access to the election, to modify and delete it.
    /// Defaults to none
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
/// The user object. Currently not used
pub struct User {
    pub username: String
}

#[derive(juniper::GraphQLEnum, Debug)]
/// The importance of an election. Affects sorting and filtering.
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
/// An election
pub struct Election {
    /// The id of the election (not user facing)
    pub id: Uuid,
    /// The name of the election
    pub name: String,
    /// The description of the election
    pub description: String,
    /// The available choices to vote for
    pub choices: Vec<String>,
    /// The start date of the vote
    pub start_date: DateTime<Utc>,
    /// The end date of the vote
    pub end_date: DateTime<Utc>,
    /// The importance of the election
    pub importance: Importance
}

#[derive(juniper::GraphQLInputObject)]
/// Input to create a new election
pub struct ElectionInput {
    /// The name of the election
    pub name: String,
    /// The permissions object (see [PermissionSet](struct.PermissionSet.html))
    pub permissions: Option<PermissionSet>,
    /// The description of the election
    pub description: Option<String>,
    /// The choices to make available to voters
    pub choices: Option<Vec<String>>,
    /// The date for voting to start at
    pub start_date: Option<DateTime<Utc>>,
    /// The date for voting to end at
    pub end_date: Option<DateTime<Utc>>,
    /// The importance of the election
    pub importance: Option<Importance>
}
