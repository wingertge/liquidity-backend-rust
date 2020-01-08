use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::{
    fmt,
    str::FromStr
};

#[derive(juniper::GraphQLInputObject)]
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

#[derive(juniper::GraphQLEnum, Debug, Serialize, Deserialize, Clone, PartialEq)]
/// The importance of an election. Affects sorting and filtering.
pub enum Importance {
    Important,
    Regular,
    Minor
}

impl fmt::Display for Importance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(juniper::GraphQLObject, Clone, PartialEq, Debug)]
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

impl Default for Election {
    fn default() -> Self {
        Election {
            id: Uuid::new_v4(),
            name: "".to_string(),
            description: "".to_string(),
            choices: vec![],
            start_date: Utc::now(),
            end_date: Utc::now(),
            importance: Importance::Regular
        }
    }
}

#[derive(juniper::GraphQLInputObject)]
/// Input to create a new election
pub struct ElectionInput {
    /// The name of the election
    pub name: String,
    /// A set of roles allowed to interact with this election.
    /// Base permissions are always required in addition to these.
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
