use lambda_http::{service_fn, Request, RequestExt, Response, Body};
use db::DynamodbClient;
use intellidyn_error::CustomError;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct Table {
    #[serde(default)]
    name: String,
    #[serde(default)]
    key: String
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

    lambda_http::run(service_fn(create_table)).await?;
    
    Ok(())
}

async fn create_table(
    request: Request
) -> Result<Response<Body>, CustomError> {
    let table: Table = Table {
        name: request.query_string_parameters().first("name").unwrap_or_default().to_string(),
        key: request.query_string_parameters().first("key").unwrap_or_default().to_string()
    };

    let db_client = DynamodbClient::init().await?;

    let resp = match db_client.create_table(&table.name, &table.key).await {
        Ok(_) => format!("Table {} created with key {}", &table.name, &table.key),
        Err(e) => format!("Table is not created due to error: {:?}", e)
    };

    let response = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(resp))?;

    Ok(response)
}