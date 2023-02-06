use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub config: Config
}
#[derive(Deserialize, Debug)]
pub struct Config {
    pub url: String,
    pub db: String,
    pub collection: String,
    pub check_time: Option<u64>,
}