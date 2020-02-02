use crate::{db::DatabaseError, Connection, Merge};
use eventstore::{EventData, ResolvedEvent};
use futures::TryStreamExt;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, sync::Arc};
use tracing_futures::Instrument;

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Create,
    Update,
    Delete
}

impl AsRef<str> for EventType {
    fn as_ref(&self) -> &str {
        match self {
            EventType::Create => "create",
            EventType::Update => "update",
            EventType::Delete => "delete"
        }
    }
}

impl From<String> for EventType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "create" => EventType::Create,
            "update" => EventType::Update,
            "delete" => EventType::Delete,
            _ => EventType::Create
        }
    }
}

#[async_trait]
pub trait DbConnection: Clone {
    async fn write_event<S, E, P>(
        &self,
        stream: S,
        event_type: E,
        payload: P
    ) -> Result<(), DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        E: AsRef<str> + Send + Debug,
        P: Serialize + Send + Debug;

    async fn create<S, P>(&self, stream: S, payload: P) -> Result<(), DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        P: Serialize + Send + Debug;

    async fn read<S, T, C, U>(&self, stream: S) -> Result<Option<T>, DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        T: From<C> + Merge<U> + Send + Clone,
        C: DeserializeOwned,
        U: DeserializeOwned;

    async fn update<S, P>(&self, stream: S, payload: P) -> Result<(), DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        P: Serialize + Send + Debug;

    async fn delete<S, P>(&self, stream: S, payload: P) -> Result<(), DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        P: Serialize + Send + Debug;
}

#[async_trait]
impl DbConnection for Arc<Connection> {
    async fn write_event<S, E, P>(
        &self,
        stream: S,
        event_type: E,
        payload: P
    ) -> Result<(), DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        E: AsRef<str> + Send + Debug,
        P: Serialize + Send + Debug
    {
        let event_data = EventData::json(event_type, payload)?;
        self.write_events(stream)
            .push_event(event_data)
            .execute()
            .instrument(trace_span!("store_event"))
            .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn create<S, P>(&self, stream: S, payload: P) -> Result<(), DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        P: Serialize + Send + Debug
    {
        self.write_event(stream, EventType::Create, payload).await
    }

    #[instrument(skip(self))]
    async fn read<S, T, C, U>(&self, stream: S) -> Result<Option<T>, DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        T: From<C> + Merge<U> + Send + Clone,
        C: DeserializeOwned,
        U: DeserializeOwned
    {
        let stream = self
            .read_stream(stream)
            .forward()
            .iterate_over()
            .map_err(DatabaseError::from);

        let res = stream
            .try_fold(None, move |acc: Option<T>, item: ResolvedEvent| async {
                let event = item.event;
                event
                    .map(|event| match event.event_type.to_owned().into() {
                        EventType::Create => {
                            let payload = event.as_json::<C>()?;
                            Ok(Some(payload.into()))
                        }
                        EventType::Update => match acc.clone() {
                            Some(acc) => {
                                let payload = event.as_json::<U>()?;
                                Ok(Some(acc.merge_with(payload)))
                            }
                            None => Ok(None)
                        },
                        EventType::Delete => Err(DatabaseError::NotFound)
                    })
                    .unwrap_or(Ok(acc))
            })
            .await;

        match res.as_ref() {
            Err(DatabaseError::NotFound) => Ok(None),
            _ => res
        }
    }

    #[instrument(skip(self))]
    async fn update<S, P>(&self, stream: S, payload: P) -> Result<(), DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        P: Serialize + Send + Debug
    {
        self.write_event(stream, EventType::Update, payload).await
    }

    #[instrument(skip(self))]
    async fn delete<S, P>(&self, stream: S, payload: P) -> Result<(), DatabaseError>
    where
        S: AsRef<str> + Send + Debug,
        P: Serialize + Send + Debug
    {
        self.write_event(stream, EventType::Delete, payload).await
    }
}
