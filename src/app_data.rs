use pollster::FutureExt as _;
use reqwest::Client;
use vizia::prelude::*;

use crate::api_models::*;
use crate::db_models::*;
use crate::queries::*;

pub enum AppEvent {
  SetWeatherData(Option<Meteo>),
  ConfirmLocation(String, String),
  RefreshForecast,
  Rehydrate(Location, HistoricalForecast),
  // FailedToRetrieveForecast,
  // UhOh,
  // BigUhOh,

  // might be extraneous now
  UpdateGeohash(String),
  UpdateLocationName(String),
}

#[derive(Default, Debug, Lens, Clone)]
pub struct AppData {
  pub weather_data: Option<Meteo>,
  pub geohash: String,
  pub location_confirmed: bool,
  pub location: Option<Location>,
  pub forecast: Option<HistoricalForecast>,
}

impl Model for AppData {
  fn event(&mut self, ex: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _meta| match app_event {
      AppEvent::SetWeatherData(meteo) => {
        println!("AppEvent::SetWeatherData({:#?})", meteo);
        self.weather_data = meteo.clone();
      }

      AppEvent::ConfirmLocation(_, _) => {
        println!("AppEvent::ConfirmLocation");
        self.location_confirmed = true;
        let add_result = add_location_to_db("yee", &self.geohash).block_on();
        println!("add result: {:?}", add_result);
        // if let Some(ll) = self.latlng {
        //   let weather_data = get_weather_data(&ll);
        //   let _ = ex.emit(AppEvent::SetWeatherData(weather_data));
        // }
      }

      AppEvent::RefreshForecast => {
        println!("AppEvent::RefreshForecast");
        handle_app_event_refresh_forecast(ex, self.clone());
      }

      AppEvent::Rehydrate(loc, hf) => {
        println!("AppEvent::Rehydrate({:#?})", loc);
        self.location = Some(loc.clone());
      }

      AppEvent::UpdateLocationName(new_location_name) => {
        println!("AppEvent::UpdateLocationName({})", new_location_name);
      }

      AppEvent::UpdateGeohash(new_geohash) => {
        println!("AppEvent::UpdateGeohash({:?})", new_geohash);
        if new_geohash.len() <= 12 {
          self.geohash = String::from(new_geohash);
          // self.latlng = convert_geohash_to_coords(new_geohash);
        }
      }
    });
  }
}

fn handle_app_event_refresh_forecast(ex: &mut EventContext, app_data: AppData) -> Option<()> {
  let (lat, lng) = app_data
    .location
    .clone()?
    .coords()?;
  let api_response = get_weather_data(lat, lng)?;
  let _ = add_forecast_to_db(&app_data.location?, &api_response);
  // ex.emit(AppEvent::Rehydrate(loc, ()));
  Some(())
}

const BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";

fn get_weather_data(lat: f64, lng: f64) -> Option<Meteo> {
  let current_params: String = vec![
    "temperature_2m",
    "relative_humidity_2m",
    "apparent_temperature",
    "is_day",
    "precipitation",
    "rain",
    "showers",
    "snowfall",
    "weather_code",
    "cloud_cover",
    "pressure_msl",
    "surface_pressure",
    "wind_speed_10m",
    "wind_direction_10m",
    "wind_gusts_10m",
  ]
  .join(",");

  let hourly_params: String = vec![
    "temperature_2m",
    "relative_humidity_2m",
    "dew_point_2m",
    "precipitation_probability",
    "precipitation",
    "rain",
    "showers",
    "snowfall",
    "weather_code",
    "visibility",
    "wind_gusts_10m",
  ]
  .join(",");

  let daily_params: String = vec![
    "weather_code",
    "temperature_2m_max",
    "temperature_2m_min",
    "sunrise",
    "sunset",
    "daylight_duration",
    "uv_index_max",
    "precipitation_sum",
    "rain_sum",
    "snowfall_sum",
    "precipitation_probability_max",
  ]
  .join(",");

  let query = vec![
    ("latitude", lat.to_string()),
    ("longitude", lng.to_string()),
    ("current", current_params),
    ("hourly", hourly_params),
    ("daily", daily_params),
    ("temperature_unit", String::from("fahrenheit")),
    ("wind_speed_unit", String::from("mph")),
    ("precipitation_unit", String::from("inch")),
    ("timezone", String::from("America/New_York")),
    ("forecast_days", String::from("1")),
    ("forecast_hours", String::from("24")),
    ("past_hours", String::from("24")),
  ];

  Client::new()
    .get(BASE_URL)
    .query(&query)
    .send()
    .block_on()
    .ok()?
    .json::<Meteo>()
    .block_on()
    .ok()
}

pub async fn rehydrate_from_db(cx: &mut Context) -> anyhow::Result<()> {
  println!("Rehydrating 🚰");
  let saved_location = get_latest_location().await?;
  let historical_forecast = get_latest_historical_forecast(saved_location.id).await;
  match historical_forecast {
    Ok(hf) => {
      cx.emit(AppEvent::Rehydrate(saved_location, hf));
    }
    Err(e) => {
      println!("{:?}", e);
    }
  }
  cx.emit(AppEvent::RefreshForecast);

  Ok(())
}
