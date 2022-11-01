use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub config: Config
}
#[derive(Deserialize, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub db: String,
    pub check_time: Option<u64>,
}