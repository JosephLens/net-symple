use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_utc_timestamps().with_level(LevelFilter::Info).init().unwrap();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<ApiGatewayProxyResponse, Error> {
    let path = event.payload.path.unwrap();

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: aws_lambda_events::http::HeaderMap::new(),
        multi_value_headers: aws_lambda_events::http::HeaderMap::new(),
        body: Some(Body::Text(format!("Alloha from '{}' good", path))),
        is_base64_encoded: false,
    };

    Ok(resp)
}
