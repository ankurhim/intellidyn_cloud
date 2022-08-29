use lambda_http::{service_fn, Request, RequestExt, Response, Body, tower::ServiceBuilder, http};
use tower_http::cors::{ CorsLayer, Any};
use intellidyn_error::CustomError;
use aws_sdk_lambda::{Client, Region};
use aws_config::meta::region::RegionProviderChain;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

    let cors_layer = CorsLayer::new()
    .allow_methods(Any)
    .allow_origin(Any);

    let handler = ServiceBuilder::new()
    .layer(cors_layer)
    .service(service_fn(assign_company_code_to_company));

    lambda_http::run(handler).await?;
    
    Ok(())
}

pub async fn assign_company_code_to_company(request: Request) -> Result<Response<Body>, CustomError> {
    
    let _context = request.lambda_context();

    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let lambda_client = Client::new(&shared_config);
    
    let invoke_response = lambda_client
    .invoke()
    .function_name("arn:aws:lambda:us-east-1:105390037103:function:ListCompanies")
    .set_payload(None)
    .invocation_type(aws_sdk_lambda::model::InvocationType::RequestResponse)
    .log_type(aws_sdk_lambda::model::LogType::Tail)
    .send()
    .await?;

    let response = Response::builder()
        .status(http::StatusCode::from_u16(*&invoke_response.status_code() as u16).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(format!("{:#?}", invoke_response.payload().unwrap())))?;

    Ok(response)
}