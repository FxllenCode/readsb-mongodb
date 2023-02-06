use std::any::{Any, self};
use serde::{Deserialize, Serialize}; 
use::serde;
// The annoying part about this is that it is not tolerant to updates. It can break with breaking changes!
// Maybe someday in the future it could be possible to use an AI to scrape https://github.com/wiedehopf/readsb/blob/dev/README-json.md and create the types automatically through a GitHub bot/pull request but I digressgress.
// For now, the type will need to update if readsb adds new fields. 
use chrono::DateTime;

#[derive(Serialize, Deserialize)]
pub struct RawData {
    #[serde(rename(deserialize = "now"))]
    pub date : f64,
    // #[serde(skip_deserializing)]
    pub aircraft: Vec<Aircraft>
}
#[derive(Serialize, Deserialize)]
pub struct Aircraft {
    #[serde(rename(deserialize = "hex"))]
    pub icao24 : Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub data_type : Option<Type>,
    pub callsign : Option<String>,
    pub alt_baro : Option<Altitude>,
    pub alt_geom : Option<i64>,
    pub gs : Option<f64>,
    pub ias : Option<f64>

}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]

pub enum Type {
    AdsbIcao,
    AdsbIcaoNt,
    AdsrIcao,
    TisbIcao,
    Adsc,
    Mlat,
    Other,
    ModeS,
    AdsbOther,
    AdsrOther,
    TisbOther,
    TisbTrackfile,

}
#[derive(Debug, Deserialize, Serialize)]

pub enum Altitude {
    Ground,
    Altitude(f64),
}