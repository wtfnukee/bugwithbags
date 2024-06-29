use hyper::{Body, Response};
use log::info;

pub async fn handle_index() -> Result<Response<Body>, hyper::Error> {
    info!(target:"API usage", "/index hit");
    let response = Response::builder()
        .status(hyper::StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "GET")
        .header(hyper::header::CONTENT_TYPE, "text/plain")
        .body(Body::from("Rust backend"))
        .unwrap();
    Ok(response)
}
