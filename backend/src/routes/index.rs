use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::{Body, Response};
use log::info;

pub async fn handle_index() -> Result<Response<Body>, hyper::Error> {
    info!("/index hit");
    let mut response = Response::new(Body::from("Bug with Bags"));
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    Ok(response)
}
