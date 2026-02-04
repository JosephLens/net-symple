use lambda_http::{run, Body, Error, Request, Response};
use lambda_runtime::service_fn;
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_utc_timestamps().with_level(LevelFilter::Info).init().unwrap();
    run(service_fn(my_handler)).await?;
    Ok(())
}

async fn my_handler(event: Request) -> Result<Response<Body>, Error> {
    let path = event.uri().path();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(Body::Text(format!("Alloha from '{}' good", path)))
        .map_err(|e| Error::from(e.to_string()))?;

    Ok(resp)
}
