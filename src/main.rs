use log::{debug, error, info, trace, warn};
mod utils;
use utils::config::Data;
use std::process::exit;
use std::fs;
mod controller;
use controller::db_interface::DbInterface;
use controller::metadata::Metadata;
use reqwest;
fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
#[tokio::main]
async fn main() {
    setup_logger().expect("Failed to setup logger");
    info!("Starting mission...");

    let contents = match fs::read_to_string("Configuration.toml") {
        Ok(contents) => contents,
        Err(e) => {
            error!("Failed to read config file: {}", e);
            exit(1);
        }
    };

    let data: Data = match  toml::from_str(&contents) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to parse config file: {}", e);
            exit(1);
        }
    };
        info!("Config file parsed successfully!");
        debug!("Config file contents: {:?}", data);

        let mut db_interface: DbInterface = DbInterface::new(data.config.db).await;
        db_interface.connect().await.expect("Database did not connect!");

        info!("Initalizing metadata and constants...");
        let mut metadata: Metadata = Metadata::new();
        if data.config.check_time.is_some() {
            info!("You supplied a manual check time in `Conguration.toml`. Setting check time to {} seconds", data.config.check_time.unwrap());
            metadata.set_time_to_check(data.config.check_time.unwrap() * 1000);
        } else {
            info!("There was not a manual check time supplied in `Configuration.toml`. Setting check time to the refresh time in `reciever.json` * 30 in seconds!! (Default: 1000 * 30, so 30 seconds)");
            let res = reqwest::get(format!("http://{}:{}/data/receiver.json", data.config.ip, data.config.port)).await.expect("Failed to get reciever.json!");
            let res_json = res.json::<serde_json::Value>().await.expect("Failed to parse reciever.json!");
            let refresh_time = res_json["refresh"].as_u64().unwrap();
            metadata.set_time_to_check(refresh_time * 30);
            info!("Check time set to {} seconds", refresh_time * 30);
        }


}
