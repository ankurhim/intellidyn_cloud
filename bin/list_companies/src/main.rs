use lambda_http::{service_fn, Request, RequestExt, Response, Body};
use db::DynamodbClient;
use aws_sdk_dynamodb::model::AttributeValue;
use models::company_model::Company;
use intellidyn_error::CustomError;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

    lambda_http::run(service_fn(list_companies)).await?;
    
    Ok(())
}

pub async fn list_companies(request: Request) -> Result<Response<Body>, CustomError> {
    
    let _context = request.lambda_context();
    
    let db_client = DynamodbClient::init().await?;
    
    let companies_list: Vec<Company> = db_client.list_items("company").await?;

    let response = match &companies_list.len() {
        0 => Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .body(Body::from(format!("No companies found")))?,

        _ => Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .body(Body::from(format!("{:?}", companies_list)))?,
    };

    Ok(response)
}