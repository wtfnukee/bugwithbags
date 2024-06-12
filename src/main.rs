use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use log::info;
use mongodb::{options::ClientOptions, Client, Collection};
use std::env;

mod logger;
mod routes;

use routes::index::handle_index;
use routes::stations::{handle_stations, initialize_stations, StationData};

async fn route(
    req: Request<Body>,
    collection: Collection<StationData>,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/stations") => handle_stations(collection).await,
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

    // Initialize MongoDB client using the environment variable
    let mongo_uri =
        env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let mut client_options = ClientOptions::parse(&mongo_uri).await?;
    client_options.app_name = Some("StationFetcher".to_string());
    let client = Client::with_options(client_options)?;

    let database = client.database("station_db");
    let collection = database.collection::<StationData>("stations");

    // Initialize station data if not already present
    initialize_stations(&collection).await;

    let make_svc = make_service_fn(move |_conn| {
        let collection = collection.clone();
        async move { Ok::<_, hyper::Error>(service_fn(move |req| route(req, collection.clone()))) }
    });

    let addr = ([0, 0, 0, 0], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
