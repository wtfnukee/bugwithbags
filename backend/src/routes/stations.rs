use hyper::{Body, Request, Response};
use log::info;
use mongodb::{bson::Document, Collection};
use parsers::yandex::stations::{fetch_stations, StationData, StationResponse};


pub async fn handle_stations(
    req: Request<Body>,
    collection: Collection<StationData>,
) -> Result<Response<Body>, hyper::Error> {
    let query_params = req.uri().query().unwrap_or("");
    let params: Vec<&str> = query_params.split('&').collect();

    let mut offset = 0;
    let mut limit = 10;
    let mut filters = Document::new();

    for param in &params {
        let key_value: Vec<&str> = param.split('=').collect();
        if key_value.len() == 2 {
            match key_value[0] {
                "offset" => offset = key_value[1].parse().unwrap_or(0),
                "limit" => limit = key_value[1].parse().unwrap_or(10),
                "station_type" => { filters.insert("station_type", key_value[1]); },
                "transport_type" => { filters.insert("transport_type", key_value[1]); },
                "country" => { filters.insert("country", key_value[1]); },
                "region" => { filters.insert("region", key_value[1]); },
                "settlement" => { filters.insert("settlement", key_value[1]); },
                _ => (),
            }
        }
    }

    let (existing_stations, total_stations) = fetch_stations(&collection, offset, limit, Some(filters))
        .await
        .expect("Failed to fetch stations");

    info!(target:"API usage", "/stations {} hit", params.join("&"));

    let json = serde_json::to_string(&StationResponse {
        stations: existing_stations,
        total_stations,
        offset,
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
