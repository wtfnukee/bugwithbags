use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::header::{CONTENT_TYPE, HeaderValue};
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use std::collections::HashMap;
use futures::future::join_all;
use log::{
    debug, info, warn, error, 
    Record, Level, Metadata,
    SetLoggerError, LevelFilter
};

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}


static LOGGER: Logger = Logger;

pub fn logger_init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}


async fn fetch_address(vokzal_id: u32) -> Option<(u32, String)> {
    let url = format!("https://www.rzd.ru/ru/11705/page/2012302?vokzalId={}", vokzal_id);
    let response = reqwest::get(&url).await.ok()?;
    
    let html_content = response.text_with_charset("utf-8").await.ok()?;
    let document = Html::parse_document(&html_content);
    let address_selector = Selector::parse("body > div.body-content > header > div.bread-crumbs-menu.bread-crumbs-menu-img.bread-crumbs-menu-img_index > section > div.header-custom__note").unwrap();
    
    let address = document.select(&address_selector)
        .next()
        .map(|elem| elem.text().collect::<Vec<_>>().concat());

    match address {
        Some(address) => Some((vokzal_id, address)),
        None => None
    }
}

#[derive(Serialize)]
struct AddressResponse {
    addresses: HashMap<u32, String>,
}

async fn handle_stations() -> Result<Response<Body>, hyper::Error> {
    let mut futures = Vec::new();

    for vokzal_id in 1..=1000 {
        futures.push(tokio::spawn(async move {
            let result = fetch_address(vokzal_id).await;
            result
        }));
    }

    let results = join_all(futures).await;

    let mut addresses = HashMap::new();

    for result in results {
        if let Ok(Some((id, address))) = result {
            addresses.insert(id, address);
        }
    }

    let json = serde_json::to_string(&AddressResponse { addresses }).unwrap();

    let mut response = Response::new(Body::from(json));
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    Ok(response)
}

async fn handle_index() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("Bug with Bags"));
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    info!("Index hitted!\n");
    Ok(response)
}

async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/stations") => handle_stations().await,
        (&Method::GET, "/") => handle_index().await,
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match logger_init() {
        Ok(_) => info!("Logger is running"),
        Err(e) => error!("Error{e} is encountered while initing logger")
    }

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(route)) }
    });

    let addr = ([0, 0, 0, 0], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
