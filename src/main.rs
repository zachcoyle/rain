use std::str;

use pollster::FutureExt as _;
use vizia::prelude::*;

mod api_models;
mod app_data;
mod db_models;
mod queries;
mod screens;
mod views;

use app_data::{rehydrate_from_db, AppData};
use queries::setup_database;
use screens::{new_location_form::*, today_view::*};

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

    Binding::new(cx, AppData::weather_data, |cx, lens| {
      if let Some(forecast) = lens.get(cx) {
        ForecastScreen::new(cx, forecast);
      } else {
        NewLocationForm::new(cx);
      }
    });
  })
  .title("Rain 🌦️")
  .run()
}
