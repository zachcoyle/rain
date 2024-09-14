use std::str;

use pollster::FutureExt as _;
use vizia::prelude::*;

mod api_models;
mod app_data;
mod db_models;
mod new_location_form;
mod queries;
mod views;

use app_data::{rehydrate_from_db, AppData, AppEvent};
use new_location_form::*;
use queries::setup_database;
use views::{DataCell, WeatherCode};

#[tokio::main]
async fn main() -> Result<(), vizia::ApplicationError> {
  env_logger::init();
  let _ = setup_database().await;

  Application::new(|cx| {
    let _ = rehydrate_from_db(cx).block_on();

    if let Ok(style) = str::from_utf8(include_bytes!("style.css")) {
      cx.add_stylesheet(style)
        .expect("Failed to add stylesheet");
    }

    AppData::default().build(cx);

    NewLocationForm::new(cx);

    Binding::new(cx, AppData::weather_data, |cx, lens| {
      if let Some(forecast) = lens.get(cx) {
        WeatherCode::new(
          cx,
          forecast
            .current
            .weather_code,
        );

        HStack::new(cx, |cx| {
          let xs = vec![
            (
              "time",
              forecast
                .current
                .time,
              "".to_string(),
            ),
            (
              "temp",
              forecast
                .current
                .temperature_2_m
                .to_string(),
              forecast
                .current_units
                .temperature_2_m,
            ),
            (
              "wind speed",
              forecast
                .current
                .wind_speed_10_m
                .to_string(),
              forecast
                .current_units
                .wind_speed_10_m,
            ),
            (
              "high temp",
              forecast
                .daily
                .temperature_2_m_max[0]
                .to_string(),
              forecast
                .daily_units
                .temperature_2_m_max,
            ),
            (
              "low temp",
              forecast
                .daily
                .temperature_2_m_min[0]
                .to_string(),
              forecast
                .daily_units
                .temperature_2_m_min,
            ),
          ];

          for x in xs {
            DataCell::new(
              cx,
              x.0
                .to_string(),
              x.1,
              x.2,
            );
          }
        });
      }
    })
  })
  .title("Rain 🌦️")
  .run()
}
