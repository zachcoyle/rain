use std::str;

use pollster::FutureExt as _;
use vizia::prelude::*;

mod api_models;
mod app_data;
mod db_models;
mod queries;
mod views;

use app_data::*;
use queries::*;
use views::*;

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

    if let Ok(ls) = get_all_locations().block_on() {
      for l in ls {
        // TabView::new(cx, lens, content)
      }
    }

    // FIXME: update these `.on_edit`s
    Textbox::new(
      cx,
      AppData::location.map(|maybe_location| {
        maybe_location
          .clone()
          .map(|l| l.geohash)
          .unwrap_or("".to_string())
      }),
    )
    .on_edit(|ex, new_geohash| ex.emit(AppEvent::UpdateGeohash(new_geohash)));
    Textbox::new(
      cx,
      AppData::location.map(|maybe_location| {
        maybe_location
          .clone()
          .map(|l| l.name)
          .unwrap_or("".to_string())
      }),
    )
    .on_edit(|ex, new_location_name| ex.emit(AppEvent::UpdateLocationName(new_location_name)));

    Binding::new(cx, AppData::location, |cx, lens| {
      let loc = lens.get(cx);
      let loc_for_button = loc.clone();
      Button::new(cx, move |cx| Label::new(cx, "Get Weather!"))
        .on_press(move |ex| {
          if let Some((lat, lng)) = loc
            .clone()
            .and_then(|x| x.coords())
          {
            ex.emit(AppEvent::ConfirmLocation(lat.to_string(), lng.to_string()));
          }
          ex.spawn(move |cx| {
            // if let Some(ll) = &coords.clone() {
            //   let weather_data = get_weather_data(ll);
            //   let _ = cx.emit(AppEvent::SetWeatherData(weather_data));
            // }
          });
        })
        .disabled(
          loc_for_button
            .and_then(|x| x.coords())
            .is_none(),
        );
    });

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
