use futures::stream::TryStreamExt;
use hyper::{Body, Response};
use log::info;
use mongodb::options::InsertManyOptions;
use mongodb::{bson::doc, Collection};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

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
    country: String,
    country_code: String,
    region: String,
    region_code: String,
    settlement: String,
    settlement_code: String,
}

#[derive(Debug, Serialize)]
pub struct StationResponse {
    pub stations: Vec<StationData>,
}

async fn fetch_stations(
    collection: &Collection<StationData>,
) -> Result<Vec<StationData>, mongodb::error::Error> {
    collection.find(None, None).await?.try_collect().await
}

pub async fn handle_stations(
    collection: Collection<StationData>,
) -> Result<Response<Body>, hyper::Error> {
    let existing_stations = fetch_stations(&collection)
        .await
        .expect("Failed to fetch stations");

    info!("/stations hit");

    let json = serde_json::to_string(&StationResponse {
        stations: existing_stations,
    })
    .unwrap();

    let response = Response::builder()
        .status(hyper::StatusCode::OK)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))
        .unwrap();
    Ok(response)
}

async fn fetch_api_key() -> String {
    env::var("YANDEX_RASP_API_KEY").expect("YANDEX_RASP_API_KEY must be set")
}

async fn fetch_station_data(api_key: &str) -> serde_json::Value {
    let api_url = format!(
        "https://api.rasp.yandex.net/v3.0/stations_list/?apikey={}&lang=ru_RU&format=json",
        api_key
    );
    let client = Client::new();
    let res = client
        .get(&api_url)
        .send()
        .await
        .expect("Failed to fetch stations from Yandex API");

    if res.status() != reqwest::StatusCode::OK {
        panic!("Yandex API returned an error: {}", res.status());
    }

    res.json().await.expect("Failed to parse JSON response")
}

fn parse_station_data(json: serde_json::Value) -> Vec<StationData> {
    let mut station_data = Vec::new();

    if let Some(countries) = json["countries"].as_array() {
        for country in countries {
            let country_name = country["title"].as_str().unwrap_or_default().to_string();
            let country_code = country["codes"]["yandex_code"]
                .as_str()
                .unwrap_or_default()
                .to_string();

            if let Some(regions) = country["regions"].as_array() {
                for region in regions {
                    let region_name = region["title"].as_str().unwrap_or_default().to_string();
                    let region_code = region["codes"]["yandex_code"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string();

                    if let Some(settlements) = region["settlements"].as_array() {
                        for settlement in settlements {
                            let settlement_name =
                                settlement["title"].as_str().unwrap_or_default().to_string();
                            let settlement_code = settlement["codes"]["yandex_code"]
                                .as_str()
                                .unwrap_or_default()
                                .to_string();

                            if let Some(stations) = settlement["stations"].as_array() {
                                for station in stations {
                                    if let (Some(title), Some(station_type), Some(yandex_code)) = (
                                        station["title"].as_str(),
                                        station["station_type"].as_str(),
                                        station["codes"]["yandex_code"].as_str(),
                                    ) {
                                        station_data.push(StationData {
                                            title: title.to_string(),
                                            station_type: station_type.to_string(),
                                            longitude: station["longitude"].as_f64(),
                                            latitude: station["latitude"].as_f64(),
                                            transport_type: station["transport_type"]
                                                .as_str()
                                                .map(|s| s.to_string()),
                                            direction: station["direction"]
                                                .as_str()
                                                .map(|s| s.to_string()),
                                            esr_code: station["codes"]["esr_code"]
                                                .as_str()
                                                .map(|s| s.to_string()),
                                            yandex_code: yandex_code.to_string(),
                                            country: country_name.clone(),
                                            country_code: country_code.clone(),
                                            region: region_name.clone(),
                                            region_code: region_code.clone(),
                                            settlement: settlement_name.clone(),
                                            settlement_code: settlement_code.clone(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    station_data
}

pub async fn initialize_stations(collection: &Collection<StationData>) {
    let count = collection
        .count_documents(None, None)
        .await
        .expect("Failed to count documents");
    if count > 0 {
        info!("{} stations are already initialized", count);
        return;
    }

    let api_key = fetch_api_key().await;
    let json = fetch_station_data(&api_key).await;
    let station_data = parse_station_data(json);

    collection
        .insert_many(station_data, InsertManyOptions::default())
        .await
        .expect("Failed to insert stations into MongoDB");

    info!(
        "{} stations initialized successfully",
        collection
            .count_documents(None, None)
            .await
            .expect("Failed to count documents")
    );
}
