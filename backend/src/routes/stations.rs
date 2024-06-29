use futures::stream::StreamExt;
use hyper::{Body, Request, Response};
use log::info;
use mongodb::options::{FindOptions, InsertManyOptions};
use mongodb::Collection;
use mongodb::bson::{Document, doc};
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
    pub total_pages: u32,
    pub current_page: u32,
}

pub async fn handle_stations(
    req: Request<Body>,
    collection: Collection<StationData>,
) -> Result<Response<Body>, hyper::Error> {
    let query_params = req.uri().query().unwrap_or("");
    let params: Vec<&str> = query_params.split('&').collect();

    let mut page = 1;
    let mut page_size = 10;
    let mut filters = Document::new();

    for param in params {
        let key_value: Vec<&str> = param.split('=').collect();
        if key_value.len() == 2 {
            match key_value[0] {
                "page" => page = key_value[1].parse().unwrap_or(1),
                "page_size" => page_size = key_value[1].parse().unwrap_or(10),
                "station_type" => { filters.insert("station_type", key_value[1]); },
                "transport_type" => { filters.insert("transport_type", key_value[1]); },
                "country" => { filters.insert("country", key_value[1]); },
                "region" => { filters.insert("region", key_value[1]); },
                "settlement" => { filters.insert("settlement", key_value[1]); },
                _ => (),
            }
        }
    }

    let (existing_stations, total_pages) = fetch_stations(&collection, page, page_size, Some(filters))
        .await
        .expect("Failed to fetch stations");

    info!("/stations hit");

    let json = serde_json::to_string(&StationResponse {
        stations: existing_stations,
        total_pages,
        current_page: page,
    })
    .unwrap();

    let response = Response::builder()
        .status(hyper::StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Allow-Methods", "GET")
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))
        .unwrap();
    Ok(response)
}

async fn fetch_stations(
    collection: &Collection<StationData>,
    page: u32,
    page_size: u32,
    filters: Option<Document>,
) -> Result<(Vec<StationData>, u32), mongodb::error::Error> {
    let skip = (page - 1) * page_size;
    let total_documents = collection.count_documents(filters.clone(), None).await?;
    let total_pages = (total_documents as f64 / page_size as f64).ceil() as u32;

    let find_options = FindOptions::builder()
        .skip(skip as u64)
        .limit(page_size as i64)
        .build();

    let mut cursor = collection.find(filters, find_options).await?;
    let mut stations = Vec::new();

    while let Some(station) = cursor.next().await {
        stations.push(station?);
    }

    Ok((stations, total_pages))
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

    info!("Stations initialized successfully");
}
