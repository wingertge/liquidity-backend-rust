use chrono::{DateTime, Utc};
use crate::schema::{Importance, Election};
use liquidity::{Uuid, Merge};

pub(crate) enum ElectionEventType {
    Create,
    Update
}

impl AsRef<str> for ElectionEventType {
    fn as_ref(&self) -> &str {
        match self {
            ElectionEventType::Create => "election-create",
            ElectionEventType::Update => "election-update"
        }
    }
}

impl From<String> for ElectionEventType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "election-create" => ElectionEventType::Create,
            "election-update" => ElectionEventType::Update,
            _ => ElectionEventType::Create
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct CreateElectionEvent {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub importance: Importance,
    pub created_by_id: String,
    pub choices: Vec<String>
}

impl From<CreateElectionEvent> for Election {
    fn from(e: CreateElectionEvent) -> Self {
        Election {
            id: e.id,
            name: e.name,
            description: e.description,
            start_date: e.start_date,
            end_date: e.end_date,
            importance: e.importance,
            choices: e.choices
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct UpdateElectionEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub importance: Option<Importance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub choices: Option<Vec<String>>
}

impl Merge<UpdateElectionEvent> for Election {
    fn merge_with(self, new: UpdateElectionEvent) -> Self {
        Election {
            id: self.id,
            name: new.name.unwrap_or(self.name),
            description: new.description.unwrap_or(self.description),
            choices: new.choices.unwrap_or(self.choices),
            start_date: new.start_date.unwrap_or(self.start_date),
            end_date: new.end_date.unwrap_or(self.end_date),
            importance: new.importance.unwrap_or(self.importance)
        }
    }
}