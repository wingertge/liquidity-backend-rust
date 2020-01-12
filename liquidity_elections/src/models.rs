use chrono::{DateTime, Utc};
use crate::schema::{Importance, Election};
use serde::{Serialize, Deserialize};
use liquidity::Uuid;

#[derive(Clone, Serialize, Deserialize)]
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