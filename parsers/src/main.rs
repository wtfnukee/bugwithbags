mod logger;
use log::info;

fn main() -> (){
    match logger::logger_init() {
        Ok(_) => info!("Parsers logger is running"),
        Err(e) => println!("Error {e} encountered while initing logger"),
    }
}