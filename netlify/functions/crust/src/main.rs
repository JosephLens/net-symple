use aws_lambda_events::encodings::Body;
use http::{HeaderMap, Request, Response};
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

pub(crate) async fn my_handler(event: LambdaEvent<Request<String>>) -> Result<Response<String>, Error> {
    let path = event.payload.uri().path();

    let resp = Response::builder()
        .status(200)
        .body(format!("Alloha from '{}' good", path))?;

    Ok(resp)
}
