use lambda_http::{service_fn, Request, RequestExt, Response, Body};
use db::DynamodbClient;
use intellidyn_error::CustomError;
use serde::Deserialize;
use aws_sdk_dynamodb::model::AttributeValue;
use uuid::Uuid;

#[derive(Debug, Default, Deserialize, Clone)]
struct CompanyCode {
    #[serde(default)]
    pub company_code_pk: Uuid,
    #[serde(default)]
    pub company_code: String,
    #[serde(default)]
    pub company_name: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub search_terms: String,
    #[serde(default)]
    pub street_house_no: String,
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub telephone: Option<u64>,
    #[serde(default)]
    pub tel_ext: Option<u16>,
    #[serde(default)]
    pub mobile_phone: Option<u64>,
    #[serde(default)]
    pub fax: Option<u64>,
    #[serde(default)]
    pub fax_ext: Option<u16>,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub comments: Option<String>,
    #[serde(default)]
    pub request: Option<String>,
    #[serde(default)]
    pub short_desc: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

    lambda_http::run(service_fn(define_company_code)).await?;
    
    Ok(())
}

async fn define_company_code(
    request: Request
) -> Result<Response<Body>, CustomError> {
    let new_company_code: Option<CompanyCode> = request.payload().unwrap();

    let company_code_pk_av = AttributeValue::S(Uuid::new_v4().to_string());
    let company_code_av = AttributeValue::S(new_company_code.clone().unwrap().company_code.into());
    let company_name_av = AttributeValue::S(new_company_code.clone().unwrap().company_name.into());
    let city_av = AttributeValue::S(new_company_code.clone().unwrap().city.into());
    let country_av = AttributeValue::S(new_company_code.clone().unwrap().country.into());
    let language_av = AttributeValue::S(new_company_code.clone().unwrap().language.into());
    let currency_av = AttributeValue::S(new_company_code.clone().unwrap().currency.into());
    let search_terms_av = AttributeValue::S(new_company_code.clone().unwrap().search_terms.into());
    let street_house_no_av = AttributeValue::S(new_company_code.clone().unwrap().street_house_no.into());
    let postal_code_av = AttributeValue::S(new_company_code.clone().unwrap().postal_code.into());
    let telephone_av = match new_company_code.clone().unwrap().telephone.unwrap().to_string().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::N(new_company_code.clone().unwrap().telephone.unwrap().to_string())
    };
    let tel_ext_av = match new_company_code.clone().unwrap().tel_ext.unwrap().to_string().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::N(new_company_code.clone().unwrap().tel_ext.unwrap().to_string())
    };
    let mobile_phone_av = match new_company_code.clone().unwrap().mobile_phone.unwrap().to_string().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::N(new_company_code.clone().unwrap().mobile_phone.unwrap().to_string())
    };
    let fax_av = match new_company_code.clone().unwrap().fax.unwrap().to_string().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::N(new_company_code.clone().unwrap().fax.unwrap().to_string())
    };
    let fax_ext_av = match new_company_code.clone().unwrap().fax_ext.unwrap().to_string().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::N(new_company_code.clone().unwrap().fax_ext.unwrap().to_string())
    };
    let email_av = AttributeValue::S(new_company_code.clone().unwrap().email.into());
    let comments_av = match new_company_code.clone().unwrap().comments.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company_code.clone().unwrap().comments.unwrap().into())
    };
    let request_av = match new_company_code.clone().unwrap().request.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company_code.clone().unwrap().request.unwrap().into())
    };
    let short_desc_av = match new_company_code.clone().unwrap().short_desc.unwrap().len() {
        0 => AttributeValue::Null(true),
        _ => AttributeValue::S(new_company_code.clone().unwrap().short_desc.unwrap().into())
    };

    let client = DynamodbClient::init().await?;

    let response = match client.db_client
    .put_item()
    .table_name("company_code")
    .item("company_code_pk", company_code_pk_av)
    .item("company_code", company_code_av)
    .item("company_name", company_name_av)
    .item("city", city_av)
    .item("country", country_av)
    .item("language", language_av)
    .item("currency", currency_av)
    .item("search_terms", search_terms_av)
    .item("street_house_no", street_house_no_av)
    .item("postal_code", postal_code_av)
    .item("telephone", telephone_av)
    .item("tel_ext", tel_ext_av)
    .item("mobile_phone", mobile_phone_av)
    .item("fax", fax_av)
    .item("fax_ext", fax_ext_av)
    .item("email", email_av)
    .item("comments", comments_av)
    .item("request", request_av)
    .item("short_desc", short_desc_av)
    .send()
    .await
    {
        Ok(_) => Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .body(Body::from(format!("Added a new company code as {} company code", new_company_code.as_ref().unwrap().company_code)))?,
        
        Err(e) => Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(Body::from(format!("New company code not created due to error: {:?}", e)))?
    };

    Ok(response)
}