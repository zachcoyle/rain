use std::str;

use pollster::FutureExt as _;
use vizia::prelude::*;

mod api_models;
mod app_data;
mod db_models;
mod new_location_form;
mod queries;
mod today_view;
mod views;

use app_data::{rehydrate_from_db, AppData};
use new_location_form::*;
use queries::setup_database;
use today_view::*;

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
      if lens
        .get(cx)
        .is_none()
      {
        NewLocationForm::new(cx);
      }
    });

    Binding::new(cx, AppData::weather_data, |cx, lens| {
      if let Some(forecast) = lens.get(cx) {
        ForecastScreen::new(cx, forecast);
      }
    });
  })
  .title("Rain 🌦️")
  .run()
}
