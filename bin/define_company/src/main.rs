use lambda_http::{service_fn, Request, RequestExt, Response, Body};
use db::DynamodbClient;
use intellidyn_error::CustomError;
use serde::Deserialize;
use aws_sdk_dynamodb::model::AttributeValue;
use uuid::Uuid;

#[derive(Debug, Default, Deserialize, Clone)]
struct Company {
    #[serde(default)]
    company_key: String,
    #[serde(default)]
    company_name: String,
    #[serde(default)]
    company_name_2: Option<String>,
    #[serde(default)]
    street: String,
    #[serde(default)]
    postal_code: String,
    #[serde(default)]
    city: String,
    #[serde(default)]
    country: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    currency: String,
    #[serde(default)]
    view_maintenance: Option<String>,
    #[serde(default)]
    request: Option<String>,
    #[serde(default)]
    short_desc: Option<String>
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

    let company_pk_av = AttributeValue::S(Uuid::new_v4().to_string());
    let company_key_av = AttributeValue::S(new_company.clone().unwrap().company_key.into());
    let company_name_av = AttributeValue::S(new_company.clone().unwrap().company_name.into());
    let company_name_2_av = match new_company.clone().unwrap().company_name_2.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company.clone().unwrap().company_name_2.unwrap().into())
    };
    let street_av = AttributeValue::S(new_company.clone().unwrap().street.into());
    let postal_code_av = AttributeValue::S(new_company.clone().unwrap().postal_code.into());
    let city_av = AttributeValue::S(new_company.clone().unwrap().city.into());
    let country_av = AttributeValue::S(new_company.clone().unwrap().country.into());
    let language_av = AttributeValue::S(new_company.clone().unwrap().language.into());
    let currency_av = AttributeValue::S(new_company.clone().unwrap().currency.into());
    let view_maintenance_av = match new_company.clone().unwrap().view_maintenance.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company.clone().unwrap().view_maintenance.unwrap().into())
    };
    let request_av = match new_company.clone().unwrap().request.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company.clone().unwrap().request.unwrap().into())
    };
    let short_desc_av = match new_company.clone().unwrap().short_desc.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company.clone().unwrap().short_desc.unwrap().into())
    };

    let client = DynamodbClient::init().await?;

    let response = match client.db_client
    .put_item()
    .table_name("company")
    .item("company_pk", company_pk_av)
    .item("company_key", company_key_av)
    .item("company_name", company_name_av)
    .item("company_name_2", company_name_2_av)
    .item("street", street_av)
    .item("postal_code", postal_code_av)
    .item("city", city_av)
    .item("country", country_av)
    .item("language", language_av)
    .item("currency", currency_av)
    .item("view_maintenance", view_maintenance_av)
    .item("request", request_av)
    .item("short_desc", short_desc_av)
    .send()
    .await
    {
        Ok(_) => Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .body(Body::from(format!("Added a new company as {} company_key", new_company.as_ref().unwrap().company_key)))?,
        
        Err(e) => Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(Body::from(format!("New company not created due to error: {:?}", e)))?
    };

    Ok(response)
}