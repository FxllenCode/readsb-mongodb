use std::string::ToString;

use strum;
use strum_macros::{Display, EnumString};

pub struct Airplane {
    pub hex: String,

    }

#[derive(Debug, strum_macros::Display, strum_msacros::EnumString)]
pub enum DispalyedType {
    Hex,
    Icao,
    Flight,
    Squawk,
    Altitude,
    Speed,
    VerticalRate,
    Track,
    Messages,
    Rssi,
}