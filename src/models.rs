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
  pub utc_offset_seconds: f64,
  pub timezone: String,
  pub timezone_abbreviation: String,
  pub elevation: f64,
  pub current_units: CurrentUnits,
  pub current: Current,
  pub hourly_units: HourlyUnits,
  pub hourly: Hourly,
  pub daily_units: DailyUnits,
  pub daily: Daily,
}

#[derive(Data, Lens, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Current {
  pub time: String,
  pub interval: f64,
  #[serde(rename = "temperature_2m")]
  pub temperature_2_m: f64,
  #[serde(rename = "relative_humidity_2m")]
  pub relative_humidity_2_m: f64,
  pub apparent_temperature: f64,
  pub is_day: f64,
  pub precipitation: f64,
  pub rain: f64,
  pub showers: f64,
  pub snowfall: f64,
  pub weather_code: f64,
  pub cloud_cover: f64,
  pub pressure_msl: f64,
  pub surface_pressure: f64,
  #[serde(rename = "wind_speed_10m")]
  pub wind_speed_10_m: f64,
  #[serde(rename = "wind_direction_10m")]
  pub wind_direction_10_m: f64,
  #[serde(rename = "wind_gusts_10m")]
  pub wind_gusts_10_m: f64,
}

#[derive(Data, Lens, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct CurrentUnits {
  pub time: String,
  pub interval: String,
  #[serde(rename = "temperature_2m")]
  pub temperature_2_m: String,
  #[serde(rename = "relative_humidity_2m")]
  pub relative_humidity_2_m: String,
  pub apparent_temperature: String,
  pub is_day: String,
  pub precipitation: String,
  pub rain: String,
  pub showers: String,
  pub snowfall: String,
  pub weather_code: String,
  pub cloud_cover: String,
  pub pressure_msl: String,
  pub surface_pressure: String,
  #[serde(rename = "wind_speed_10m")]
  pub wind_speed_10_m: String,
  #[serde(rename = "wind_direction_10m")]
  pub wind_direction_10_m: String,
  #[serde(rename = "wind_gusts_10m")]
  pub wind_gusts_10_m: String,
}

#[derive(Data, Lens, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Daily {
  pub time: Vec<String>,
  pub weather_code: Vec<f64>,
  #[serde(rename = "temperature_2m_max")]
  pub temperature_2_m_max: Vec<f64>,
  #[serde(rename = "temperature_2m_min")]
  pub temperature_2_m_min: Vec<f64>,
  pub sunrise: Vec<String>,
  pub sunset: Vec<String>,
  pub daylight_duration: Vec<f64>,
  pub uv_index_max: Vec<f64>,
  pub precipitation_sum: Vec<f64>,
  pub precipitation_probability_max: Vec<f64>,
}

#[derive(Data, Lens, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct DailyUnits {
  pub time: String,
  pub weather_code: String,
  #[serde(rename = "temperature_2m_max")]
  pub temperature_2_m_max: String,
  #[serde(rename = "temperature_2m_min")]
  pub temperature_2_m_min: String,
  pub sunrise: String,
  pub sunset: String,
  pub daylight_duration: String,
  pub uv_index_max: String,
  pub precipitation_sum: String,
  pub precipitation_probability_max: String,
}

#[derive(Data, Lens, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Hourly {
  pub time: Vec<String>,
  #[serde(rename = "temperature_2m")]
  pub temperature_2_m: Vec<f64>,
  #[serde(rename = "relative_humidity_2m")]
  pub relative_humidity_2_m: Vec<f64>,
  pub precipitation_probability: Vec<f64>,
  pub precipitation: Vec<f64>,
  pub rain: Vec<f64>,
  pub showers: Vec<f64>,
  pub snowfall: Vec<f64>,
  pub weather_code: Vec<f64>,
}

#[derive(Data, Lens, Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct HourlyUnits {
  pub time: String,
  #[serde(rename = "temperature_2m")]
  pub temperature_2_m: String,
  #[serde(rename = "relative_humidity_2m")]
  pub relative_humidity_2_m: String,
  pub precipitation_probability: String,
  pub precipitation: String,
  pub rain: String,
  pub showers: String,
  pub snowfall: String,
  pub weather_code: String,
}
