use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::ApiGatewayProxyResponse;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_utc_timestamps().with_level(LevelFilter::Info).init().unwrap();
    lambda_runtime::run(service_fn(my_handler)).await?;
    Ok(())
}

async fn my_handler(event: LambdaEvent<Value>) -> Result<ApiGatewayProxyResponse, Error> {
    let path = event.payload.get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("/");

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: aws_lambda_events::http::HeaderMap::new(),
        multi_value_headers: aws_lambda_events::http::HeaderMap::new(),
        body: Some(Body::Text(format!("Alloha from '{}' good", path))),
        is_base64_encoded: false,
    };

    Ok(resp)
}
