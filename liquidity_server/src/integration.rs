use crate::Config;
use liquidity::Uuid;
use reqwest::{Client, Response, Url};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Deserialize, Debug, PartialEq)]
struct GQLError {
    message: String
}

#[derive(Deserialize)]
struct GQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GQLError>>
}

#[derive(Deserialize)]
struct Election {
    id: Option<Uuid>,
    name: Option<String>,
    description: Option<String>,
    choices: Option<Vec<String>>
}

#[derive(Serialize)]
struct GQLRequest<T: Serialize> {
    operation_name: Option<String>,
    variables: Option<T>,
    query: String
}

#[tokio::test]
async fn gateway_integration_test() {
    let config = Config {
        playground_enabled: false,
        cache_ttl: Duration::from_secs(10),
        cache_size: 500,
        issuer: "".to_string(),
        audience: "".to_string(),
        jwks_url: "".to_string(),
        database_url: ([127, 0, 0, 1], 1113).into(),
        database_login: "admin".to_string(),
        database_password: "changeit".to_string(),
        port: 4000
    };

    let (server, close_handle) = futures::future::abortable(crate::run(config));
    tokio::spawn(server);
    tokio::time::delay_for(Duration::from_millis(100)).await; // Let it bind and connect to DB

    let client: Client = Client::new();
    let url: Url = "http://localhost:4000/graphql".parse().unwrap();

    let id = test_create_election(client.clone(), url.clone()).await;
    test_edit_election(&id, client.clone(), url.clone()).await;
    test_get_election(&id, client.clone(), url.clone()).await;

    close_handle.abort();
}

async fn test_create_election(client: Client, url: Url) -> Uuid {
    println!("- create election");

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct CreateElectionResponse {
        createElection: Election
    }

    let query_string = r#"
        mutation {
          createElection(input:{name:"test", choices:["test1", "test2"]}) {
            id
          }
        }
    "#;
    let query = make_query(query_string);
    let result: Response = client.post(url).json(&query).send().await.unwrap();
    let gql_response: GQLResponse<CreateElectionResponse> = result.json().await.unwrap();

    assert_eq!(gql_response.errors, None);

    let data = gql_response.data.unwrap().createElection;

    assert_ne!(data.id, None);

    data.id.unwrap()
}

async fn test_get_election(id: &Uuid, client: Client, url: Url) {
    println!("- get election");

    #[derive(Serialize)]
    struct GetElectionVariables {
        id: Uuid
    }

    #[derive(Deserialize)]
    struct GetElectionResponse {
        election: Election
    }

    let query_str = r#"
        query($id:Uuid!) {
          election(id: $id) {
            id
            name
            description
            choices
          }
        }
    "#;
    let variables = GetElectionVariables { id: id.clone() };

    let query = make_query_with_variables(query_str, variables);
    let result: Response = client.post(url).json(&query).send().await.unwrap();
    let gql_response: GQLResponse<GetElectionResponse> = result.json().await.unwrap();

    assert_eq!(gql_response.errors, None);

    let data = gql_response.data.unwrap().election;

    assert_eq!(data.id.unwrap(), id.clone());
    assert_eq!(data.name.unwrap(), "test".to_string());
    assert_eq!(data.description.unwrap(), "test_description_2".to_string());
    assert_eq!(
        data.choices.unwrap(),
        vec!["test1".to_string(), "test2".to_string()]
    );
}

async fn test_edit_election(id: &Uuid, client: Client, url: Url) {
    #[derive(Serialize)]
    struct EditElectionVariables {
        id: Uuid
    }

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct EditElectionResult {
        editElection: Election
    }

    let query_string = r#"
        mutation($id:Uuid) {
          editElection(id:$id, input:{description:"test_description_2"}) {
            id
            name
            description
            choices
          }
        }
    "#;
    let variables = EditElectionVariables { id: id.clone() };
    let query = make_query_with_variables(query_string, variables);
    let result: Response = client.post(url).json(&query).send().await.unwrap();
    let gql_response: GQLResponse<EditElectionResult> = result.json().await.unwrap();

    assert_eq!(gql_response.errors, None);

    let data = gql_response.data.unwrap().editElection;

    assert_eq!(data.id.unwrap(), id.clone());
    assert_eq!(data.name.unwrap(), "test".to_string());
    assert_eq!(data.description.unwrap(), "test_description_2".to_string());
    assert_eq!(
        data.choices.unwrap(),
        vec!["test1".to_string(), "test2".to_string()]
    );
}

fn make_query_with_variables<V: Serialize>(query_string: &str, variables: V) -> serde_json::Value {
    let request: GQLRequest<V> = GQLRequest {
        operation_name: None,
        variables: Some(variables),
        query: query_string.to_string()
    };

    serde_json::to_value(request).unwrap()
}

fn make_query(query_string: &str) -> serde_json::Value {
    let request: GQLRequest<()> = GQLRequest {
        operation_name: None,
        variables: None,
        query: query_string.to_string()
    };

    serde_json::to_value(request).unwrap()
}
