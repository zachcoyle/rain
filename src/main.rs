use std::{str, time};

use pollster::FutureExt as _;
use vizia::prelude::*;

mod api_models;
mod app_data;
mod db_models;
mod queries;
mod screens;
mod views;

use app_data::{rehydrate_from_db, AppEvent, AppState};
use queries::setup_database;
use screens::{location_list::*, new_location_form::*, today_view::*};

#[tokio::main]
async fn main() -> Result<(), vizia::ApplicationError> {
  env_logger::init();
  let _ = setup_database().await;

  Application::new(|cx| {
    let timer = cx.add_timer(
      time::Duration::from_secs(30),
      None,
      |ex, action| match action {
        TimerAction::Start => {
          println!("timer started");
        }
        TimerAction::Stop => {
          println!("timer stopped");
        }
        TimerAction::Tick(_delta) => {
          println!("Tick!");
          ex.emit(AppEvent::Timer);
        }
      },
    );

    AppState {
      weather_data: None,
      new_geohash: "".to_string(),
      location_confirmed: false,
      new_location_name: "".to_string(),
      forecast: None,
      saved_location: None,
      timer,
    }
    .build(cx);

    cx.start_timer(timer);

    let _ = rehydrate_from_db(cx).block_on();

    if let Ok(style) = str::from_utf8(include_bytes!("style.css")) {
      cx.add_stylesheet(style)
        .expect("Failed to add stylesheet");
    }

    LocationList::new(cx);

    Binding::new(cx, AppState::weather_data, |cx, lens| {
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
