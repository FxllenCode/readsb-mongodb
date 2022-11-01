use mongodb::{bson::doc, bson::to_document, bson::Document, options::ClientOptions, Client};
use log::{debug, error, info, trace, warn};

use serde::{Deserialize, Serialize};
use super::super::utils::config::Data;

#[derive(Clone, Debug)]
pub struct DbInterface {
    client_options: ClientOptions,
    client: Option<Client>,
}

impl DbInterface {
    pub async fn new(uri: String) -> DbInterface {

        // Parse your connection string into an options struct
        let client_options = ClientOptions::parse(uri).await;

        match client_options {
            Ok(mut x) => {
                x.app_name = Some("".to_string());
                Self {
                    client_options: x,
                    client: None,
                }
            }
            Err(e) => {
                panic!("Failed to parse client options: {:?}", e)
            }
        }
    } 


    pub async fn connect(&mut self) -> mongodb::error::Result<()> {
        let client = Client::with_options(self.client_options.clone())?;
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;

        self.client = Some(client);
        info!("Connected to database!");
        Ok(())
    }


}