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

async fn my_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let path = event.payload.get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("/");

    let body = serde_json::json!({
        "message": format!("Alloha from '{}' good", path),
        "path": path
    });

    Ok(serde_json::json!({
        "statusCode": 200,
        "headers": {
            "Content-Type": "application/json"
        },
        "body": body.to_string()
    }))
}
