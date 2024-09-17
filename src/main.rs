use std::str;

use pollster::FutureExt as _;
use vizia::prelude::*;

mod api_models;
mod app_data;
mod db_models;
mod queries;
mod screens;
mod views;

#[tokio::main]
async fn main() -> Result<(), vizia::ApplicationError> {
  env_logger::init();
  let _ = queries::setup_database().await;

  Application::new(|cx| {
    app_data::AppState::default().build(cx);
    // TODO: needs to default to true lol
    cx.emit(app_data::AppEvent::ToggleAutoRefresh);
    cx.emit(app_data::AppEvent::Timer);
    let _ = app_data::rehydrate_from_db(cx).block_on();

    if let Ok(style) = str::from_utf8(include_bytes!("style.css")) {
      cx.add_stylesheet(style)
        .expect("Failed to add stylesheet");
    }

    screens::location_list::LocationList::new(cx);

    Binding::new(cx, app_data::AppState::weather_data, |cx, lens| {
      if let Some(forecast) = lens.get(cx) {
        screens::today_view::ForecastScreen::new(cx, forecast);
      } else {
        screens::new_location_form::NewLocationForm::new(cx);
      }
    });
  })
  .title("Rain 🌦️")
  .run()
}
