use lambda_http::{service_fn, Request, RequestExt, Response, Body};
use db::DynamodbClient;
use intellidyn_error::CustomError;
use serde::Deserialize;
use aws_sdk_dynamodb::model::AttributeValue;

#[derive(Debug, Default, Deserialize, Clone)]
struct Company {
    #[serde(default)]
    company_code: String,
    #[serde(default)]
    company_name: String
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

    lambda_http::run(service_fn(define_company)).await?;
    
    Ok(())
}

async fn define_company(
    request: Request
) -> Result<Response<Body>, CustomError> {
    let new_company: Option<Company> = request.payload().unwrap();

    let company_code_av = AttributeValue::S(new_company.clone().unwrap().company_code.into());
    let company_name_av = AttributeValue::S(new_company.clone().unwrap().company_name.into());

    let client = DynamodbClient::init().await?;

    let response = match client.db_client
    .put_item()
    .table_name("company")
    .item("company_code", company_code_av)
    .item("company_name", company_name_av)
    .send()
    .await
    {
        Ok(_) => Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .body(Body::from(format!("Added company {} as {} company_code", new_company.as_ref().unwrap().company_name, new_company.as_ref().unwrap().company_code)))?,
        
        Err(e) => Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(Body::from(format!("New company not created due to error: {:?}", e)))?
    };

    Ok(response)
}