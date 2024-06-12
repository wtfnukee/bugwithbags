use hyper::{Body, Response};
use mongodb::{bson::doc, Collection};
use serde::{Deserialize, Serialize};
use std::env;
use reqwest::{Client, StatusCode};
use log::info;
use futures::stream::TryStreamExt;
use mongodb::options::InsertManyOptions;
use http::header::{CONTENT_TYPE, HeaderValue};

#[derive(Debug, Serialize, Deserialize)]
pub struct StationData {
    title: String,
    station_type: String,
    longitude: Option<f64>,
    latitude: Option<f64>,
    transport_type: Option<String>,
    direction: Option<String>,
    esr_code: Option<String>,
    yandex_code: String,
}

#[derive(Debug, Serialize)]
pub struct StationResponse {
    pub stations: Vec<StationData>,
}

pub async fn handle_stations(
    collection: Collection<StationData>,
) -> Result<Response<Body>, hyper::Error> {
    let existing_stations: Vec<StationData> = collection
        .find(None, None)
        .await
        .expect("Failed to fetch stations")
        .try_collect()
        .await
        .expect("Failed to collect stations");

    info!("/stations hitted");

    let json = serde_json::to_string(&StationResponse {
        stations: existing_stations,
    })
    .unwrap();

    let mut response = Response::new(Body::from(json));
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );
    Ok(response)
}

pub async fn initialize_stations(collection: &Collection<StationData>) {
    let count = collection.count_documents(None, None).await.expect("Failed to count documents");
    if count > 0 {
        info!("Stations are already initialized");
        return;
    }

    let api_key = env::var("YANDEX_RASP_API_KEY").expect("YANDEX_RASP_API_KEY must be set");
    let api_url = format!("https://api.rasp.yandex.net/v3.0/stations_list/?apikey={}&lang=ru_RU&format=json", api_key);

    let client = Client::new();
    let res = client.get(&api_url)
        .send()
        .await
        .expect("Failed to fetch stations from Yandex API");

    if res.status() != StatusCode::OK {
        panic!("Yandex API returned an error: {}", res.status());
    }

    let json: serde_json::Value = res.json().await.expect("Failed to parse JSON response");

    let countries = json["countries"].as_array().cloned().unwrap_or_default();
    let regions = countries.into_iter()
        .flat_map(|country| country["regions"].as_array().cloned().unwrap_or_default());
    let settlements = regions
        .flat_map(|region| region["settlements"].as_array().cloned().unwrap_or_default());
    let stations = settlements
        .flat_map(|settlement| settlement["stations"].as_array().cloned().unwrap_or_default());

    let station_data: Vec<StationData> = stations
        .filter_map(|station| {
            Some(StationData {
                title: station["title"].as_str()?.to_string(),
                station_type: station["station_type"].as_str()?.to_string(),
                longitude: station["longitude"].as_f64(),
                latitude: station["latitude"].as_f64(),
                transport_type: station["transport_type"].as_str().map(|s| s.to_string()),
                direction: station["direction"].as_str().map(|s| s.to_string()),
                esr_code: station["codes"]["esr_code"].as_str().map(|s| s.to_string()),
                yandex_code: station["codes"]["yandex_code"].as_str()?.to_string(),
            })
        })
        .collect();

    collection.insert_many(station_data, InsertManyOptions::default())
        .await
        .expect("Failed to insert stations into MongoDB");

    info!("Stations initialized successfully");
}
