use serde::{Deserialize, Serialize};
use vizia::prelude::*;

#[derive(Copy, Clone, Data, Serialize, Default, Deserialize, Debug)]
pub struct LatLng {
  pub lat: f64,
  pub lng: f64,
}

#[derive(Data, Lens, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Meteo {
  pub latitude: f64,
  pub longitude: f64,
  pub generationtime_ms: f64,
  pub utc_offset_seconds: i64,
  pub timezone: String,
  pub timezone_abbreviation: String,
  pub elevation: f64,
  pub current_units: Units,
  pub current: Current,
  pub hourly_units: Units,
  pub hourly: Hourly,
}

#[derive(Data, Lens, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Current {
  pub time: String,
  pub interval: i64,
  #[serde(rename = "temperature_2m")]
  pub temperature_2_m: f64,
  #[serde(rename = "wind_speed_10m")]
  pub wind_speed_10_m: f64,
}

#[derive(Data, Lens, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Units {
  pub time: String,
  pub interval: Option<String>,
  #[serde(rename = "temperature_2m")]
  pub temperature_2_m: String,
  #[serde(rename = "wind_speed_10m")]
  pub wind_speed_10_m: String,
  #[serde(rename = "relative_humidity_2m")]
  pub relative_humidity_2_m: Option<String>,
}

#[derive(Data, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Hourly {
  pub time: Vec<String>,
  #[serde(rename = "temperature_2m")]
  pub temperature_2_m: Vec<f64>,
  #[serde(rename = "relative_humidity_2m")]
  pub relative_humidity_2_m: Vec<f64>,
  #[serde(rename = "wind_speed_10m")]
  pub wind_speed_10_m: Vec<f64>,
}
