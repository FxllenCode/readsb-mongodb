use log::{debug, error, info};
mod utils;
use std::fs::{self};
use std::process::exit;
use utils::config::Data;
mod controller;
use controller::db_interface::DbInterface;
use controller::metadata::Metadata;
use controller::process_data::RawData;
use controller::registration::registration;

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
    println!("{}", registration("71BD52").await.unwrap());
    setup_logger().expect("Failed to setup logger");
    info!("Starting mission...");

    let contents = match fs::read_to_string("Configuration.toml") {
        Ok(contents) => contents,
        Err(e) => {
            error!("Failed to read config file: {}", e);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to parse config file: {}", e);
            exit(1);
        }
    };

    info!("Config file parsed successfully!");
    debug!("Config file contents: {:?}", data);

    let mut db_interface: DbInterface = DbInterface::new(data.config.db).await;
    db_interface
        .connect()
        .await
        .expect("Database did not connect!");

    info!("Initalizing metadata and constants...");
    let mut metadata: Metadata = Metadata::new();
    if data.config.check_time.is_some() {
        info!("You supplied a manual check time in `Conguration.toml`. Setting check time to {} seconds", data.config.check_time.unwrap());
        metadata.set_time_to_check(data.config.check_time.unwrap() * 1000);
    } else {
        info!("There was not a manual check time supplied in `Configuration.toml`. Setting check time to the refresh time in `reciever.json` * 30 in seconds!! (Default: 1000 * 30, so 30 seconds)");
        let res = reqwest::get(format!("{}/data/receiver.json", data.config.url))
            .await
            .expect("Failed to get reciever.json!");
        let res_json = res
            .json::<serde_json::Value>()
            .await
            .expect("Failed to parse reciever.json!");
        let refresh_time = res_json["refresh"].as_u64().unwrap();
        metadata.set_time_to_check(refresh_time * 30);
        info!("Check time set to {} seconds", refresh_time * 30);
    }
    async fn checkh() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get("http://192.168.86.71/tar1090/data/aircraft.json")
            .await?
            .json::<serde_json::Value>()
            .await?;
        println!("{resp:?}");
        let typed: RawData = serde_json::from_value(resp).unwrap();
        println!("{:?}", typed.aircraft[0].alt_baro.unwrap());
        Ok(())
    }

    // checkh().await;

    // // Check data/aircraft.json every refresh_time, and add it to the mongodb database using the db_interface function.
    // loop {
    //     let res = reqwest::get(format!("http://{}:{}/data/aircraft.json", data.config.ip, data.config.port)).await.expect("Failed to get aircraft.json!");
    //     let res_json = res.json::<serde_json::Value>().await.expect("Failed to parse aircraft.json!");
    //     // Save the JsonFile to t i washe database
    //     // Convert res_json to a vector of Aircraft, include the current time in the struct. Then insert it into the database.

    //         db_interface.insert_aircraft(&res_json).await.expect("Failed to insert aircraft into database!");

    //     info!("Inserted {} aircraft into the database!", aircraft.len());
    //     tokio::time::sleep(Duration::from_millis(metadata.get_time_to_check())).await;
    // }
}
