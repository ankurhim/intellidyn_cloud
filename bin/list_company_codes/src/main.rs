use lambda_http::{service_fn, Request, RequestExt, Response, Body};
use db::DynamodbClient;
use aws_sdk_dynamodb::model::AttributeValue;
use models::company_code_model::CompanyCode;
use intellidyn_error::CustomError;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

    lambda_http::run(service_fn(list_company_codes)).await?;
    
    Ok(())
}

pub async fn list_company_codes(request: Request) -> Result<Response<Body>, CustomError> {
    
    let _context = request.lambda_context();
    
    let db_client = DynamodbClient::init().await?;
    
    let company_codes_list: Vec<CompanyCode> = db_client.list_items("company_code").await?;

    let response = match &company_codes_list.len() {
        0 => Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .body(Body::from(format!("No company codes found")))?,

        _ => Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .body(Body::from(format!("{:?}", company_codes_list)))?,
    };

    Ok(response)
}