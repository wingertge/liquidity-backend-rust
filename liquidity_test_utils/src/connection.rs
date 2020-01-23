use std::collections::HashMap;
use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::{Mutex, Arc};
use liquidity::db::{DbConnection, EventType, DatabaseError};
use liquidity::Merge;
use serde_json::Value;

type Data = Arc<Mutex<HashMap<String, Vec<(EventType, Value)>>>>;

#[derive(Default)]
pub struct MockConnection {
    pub data: Data
}

impl Clone for MockConnection {
    fn clone(&self) -> Self {
        MockConnection {
            data: self.data.clone()
        }
    }
}

#[async_trait]
impl DbConnection for MockConnection {
    async fn write_event<S, P>(&self, stream: S, event_type: EventType, payload: P) -> Result<(), DatabaseError>
        where S: AsRef<str> + Send + Debug, P: Serialize + Send + Debug {
        let data = serde_json::to_value(payload)?;
        self.data.lock().unwrap().entry(stream.as_ref().to_string())
            .and_modify(|vec| vec.push((event_type.clone(), data.clone())))
            .or_insert_with(|| vec![(event_type.clone(), data.clone())]);
        Ok(())
    }

    async fn create<S, P>(&self, stream: S, payload: P) -> Result<(), DatabaseError> where S: AsRef<str> + Send + Debug, P: Serialize + Send + Debug {
        self.write_event(stream, EventType::Create, payload).await
    }

    async fn read<S, T, C, U>(&self, stream: S) -> Result<Option<T>, DatabaseError> where S: AsRef<str> + Send + Debug, T: From<C> + Merge<U> + Send + Clone, C: DeserializeOwned, U: DeserializeOwned {
        let data = self.data.lock().unwrap();
        let iter = data.get(stream.as_ref()).cloned();

        if iter.is_none() { return Ok(None) }

        let mut iter = iter.unwrap().into_iter().map(Ok::<(EventType, Value), DatabaseError>);

        let res = iter.try_fold(None, move |acc: Option<T>, event| {
            let (event_type, value) = event?;
            match event_type {
                EventType::Create => {
                    let payload = serde_json::from_value::<C>(value)?;
                    Ok(Some(payload.into()))
                },
                EventType::Update => {
                    match acc {
                        Some(acc) => {
                            let payload = serde_json::from_value::<U>(value)?;
                            Ok(Some(acc.merge_with(payload)))
                        },
                        None => Ok(None)
                    }
                },
                EventType::Delete => {
                    Err(DatabaseError::NotFound)
                }
            }
        });

        match res.as_ref() {
            Err(DatabaseError::NotFound) => Ok(None),
            _ => res
        }
    }

    async fn update<S, P>(&self, stream: S, payload: P) -> Result<(), DatabaseError> where S: AsRef<str> + Send + Debug, P: Serialize + Send + Debug {
        self.write_event(stream, EventType::Update, payload).await
    }

    async fn delete<S, P>(&self, stream: S, payload: P) -> Result<(), DatabaseError> where S: AsRef<str> + Send + Debug, P: Serialize + Send + Debug {
        self.write_event(stream, EventType::Delete, payload).await
    }
}