use std::sync::{Arc, Mutex};

use futures::future::join_all;
use hyper::header::{HeaderValue, CONTENT_TYPE};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use log::{error, info};
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;

mod logger;

#[derive(Serialize)]
struct StationData {
    station_id: u32,
    title: String,
    address: String,
}

async fn fetch_station_data(station_id: u32) -> Option<StationData> {
    let url = format!(
        "https://www.rzd.ru/ru/11705/page/2012302?vokzalId={}",
        station_id
    );
    let response = reqwest::get(&url).await.ok()?;

    let html_content = response.text_with_charset("utf-8").await.ok()?;
    let document = Html::parse_document(&html_content);

    let address_selector = Selector::parse("body > div.body-content > header > div.bread-crumbs-menu.bread-crumbs-menu-img.bread-crumbs-menu-img_index > section > div.header-custom__note").unwrap();
    let address = document
        .select(&address_selector)
        .next()
        .map(|elem| elem.text().collect::<Vec<_>>().concat());

    let title_selector = Selector::parse("body > div.body-content > header > div.bread-crumbs-menu.bread-crumbs-menu-img.bread-crumbs-menu-img_index > section > div.header-custom__title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|elem| elem.text().collect::<Vec<_>>().concat());

    Some(StationData {
        station_id: station_id,
        title: title?,
        address: address?,
    })
}

#[derive(Serialize)]
struct StationResponse {
    stations: Vec<StationData>,
}

async fn handle_stations() -> Result<Response<Body>, hyper::Error> {
    let mut futures = Vec::new();
    let fetched_stations = Arc::new(Mutex::new(0));

    for station_id in 1..=1000 {
        let fetched_stations = Arc::clone(&fetched_stations);
        futures.push(tokio::spawn(async move {
            let result = fetch_station_data(station_id).await;
            let mut fetched = fetched_stations.lock().unwrap();
            *fetched += 1;
            if *fetched % 10 == 0 {
                info!("Progress: {}/1000 stations fetched", *fetched);
            }
            result
        }));
    }

    let stations = join_all(futures)
        .await
        .into_iter()
        .filter_map(|result| result.ok().flatten())
        .collect();

    info!("All stations fetched");

    let json = serde_json::to_string(&StationResponse { stations }).unwrap();

    let mut response = Response::new(Body::from(json));
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    Ok(response)
}

async fn handle_index() -> Result<Response<Body>, hyper::Error> {
    info!("Index hitted");
    let mut response = Response::new(Body::from("Hello World"));
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
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
    match logger::logger_init() {
        Ok(_) => info!("Logger is running"),
        Err(e) => println!("Error {e} encountered while initing logger"),
    }
    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(route)) });

    let addr = ([0, 0, 0, 0], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
