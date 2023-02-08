use ::serde;
use serde::{Deserialize, Serialize};
use std::any::{self, Any};
// The annoying part about this is that it is not tolerant to updates. It can break with breaking changes!
// Maybe someday in the future it could be possible to use an AI to scrape https://github.com/wiedehopf/readsb/blob/dev/README-json.md and create the types automatically through a GitHub bot/pull request but I digressgress.
// For now, the type will need to update if readsb adds new fields.
use chrono::DateTime;

#[derive(Serialize, Deserialize)]
pub struct RawData {
    #[serde(rename(deserialize = "now"))]
    pub date: f64,
    pub aircraft: Vec<Aircraft>,
}
#[derive(Serialize, Deserialize)]
pub struct Aircraft {
    #[serde(rename(deserialize = "hex"))]
    pub icao24: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub data_type: Option<Type>,
    #[serde(rename(deserialize = "flight"))]
    pub callsign: Option<String>,
    pub alt_baro: Option<AltitudeUnion>,
    pub alt_geom: Option<i64>,
    pub gs: Option<f64>,
    pub ias: Option<f64>,
    pub tas: Option<f64>,
    pub mach: Option<f64>,
    pub track: Option<f64>,
    pub track_rate: Option<f64>,
    pub roll: Option<f64>,
    pub mag_heading: Option<f64>,
    pub r#true_heading: Option<f64>,
    pub baro_rate: Option<f64>,
    pub geom_rate: Option<f64>,
    pub squawk: Option<String>,
    pub emergency: Option<Emergency>,
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
    Altitude(i64),
}

#[derive(Serialize, Deserialize)]
pub struct TopLevelElement {
    #[serde(rename = "altitude")]
    altitude: AltitudeUnion,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(untagged)]
pub enum AltitudeUnion {
    Enum(AltitudeEnum),

    Integer(i64),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AltitudeEnum {
    #[serde(rename = "ground")]
    Ground,
}
#[derive(Debug, Deserialize, Serialize)]

pub enum Emergency {
    None,
    General,
    Lifeguard,
    Minfuel,
    Nordo,
    Unlawful,
    Downed,
    Reserved,
}

#[derive(Debug, Deserialize, Serialize)]

pub enum SilType {
    Unknown,
    Perhour,
    Persample,
}
