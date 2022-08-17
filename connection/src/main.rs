use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt, Response, Body};
use db::DynamodbClient;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .without_time()
    .init();

    lambda_http::run(service_fn(connect)).await?;
    
    Ok(())
}

async fn connect(
    request: Request
) -> Result<Response<Body>, Error> {
    let _context = request.lambda_context();

    let resp = match DynamodbClient::init().await {
        Ok(_) => "Connection Established".into(),
        Err(e) => format!("Connection error: {}", e)
    };

    let response = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(resp))
        .map_err(Box::new)?;

    Ok(response)
}