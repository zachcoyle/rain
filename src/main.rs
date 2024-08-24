use reqwest::blocking::Client;
use vizia::prelude::*;

mod models;
use models::*;

const STYLE: &str = r#"
:root {
  background-color: #282828;
  color: #ebdbb2;
  width: auto;
  height: auto;
}

button {
  background-color: #3c3836;
  border-color: #928374;
}

button:hover {
  background-color: #504945;
}

.input {
  width: 100;
}

.row {
  background-color: #83a598;
}
"#;

const BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Default, Debug, Lens)]
pub struct AppData {
  weather_data: Option<Meteo>,
  geohash: String,
  latlng: Option<LatLng>,
  confirmed: bool,
}

pub enum AppEvent {
  SetWeatherData(Option<Meteo>),
  UpdateGeohash(String),
  ConfirmLocation,
}

impl Model for AppData {
  fn event(&mut self, _: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _meta| match app_event {
      AppEvent::SetWeatherData(meteo) => {
        println!("AppEvent::SetWeatherData({:?})", meteo);
        self.weather_data = meteo.clone();
      }

      AppEvent::UpdateGeohash(new_geohash) => {
        println!("AppEvent::UpdateGeohash({:?})", new_geohash);
        if new_geohash.len() <= 12 {
          self.geohash = String::from(new_geohash);
          self.latlng = convert_geohash_to_coords(new_geohash);
        }
      }

      AppEvent::ConfirmLocation => {
        println!("AppEvent::ConfirmLocation");
        self.confirmed = true;
      }
    });
  }
}

fn get_weather_data(coords: &LatLng) -> Option<Meteo> {
  let query = vec![
    ("latitude", coords.lat.to_string()),
    ("longitude", coords.lng.to_string()),
    ("current", String::from("temperature_2m,wind_speed_10m")),
    (
      "hourly",
      String::from("temperature_2m,relative_humidity_2m,wind_speed_10m"),
    ),
    ("temperature_unit", String::from("fahrenheit")),
    ("wind_speed_unit", String::from("mph")),
    ("precipitation_unit", String::from("inch")),
  ];

  Client::new()
    .get(BASE_URL)
    .query(&query)
    .send()
    .ok()?
    .json::<Meteo>()
    .ok()
}

fn convert_geohash_to_coords(gh: &str) -> Option<LatLng> {
  if gh == "" {
    return None;
  }
  let (geohash::Coord { x, y }, _, _) = geohash::decode(gh).ok()?;
  Some(LatLng { lat: y, lng: x })
}

fn main() -> Result<(), vizia::ApplicationError> {
  env_logger::init();
  Application::new(|cx| {
    cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

    AppData::default().build(cx);

    Textbox::new(cx, AppData::geohash)
      .on_edit(|ex, new_geohash| ex.emit(AppEvent::UpdateGeohash(new_geohash)));

    Label::new(cx, AppData::geohash);
    Label::new(cx, AppData::latlng.map(|x| format!("{:?}", x)));

    Binding::new(cx, AppData::latlng, |cx, lens| {
      let coords = lens.get(cx);
      Button::new(cx, |cx| Label::new(cx, "Get Weather!"))
        .on_press(move |ex| {
          ex.spawn(move |cx| {
            if let Some(ll) = &coords.clone() {
              let weather_data = get_weather_data(ll);
              let _ = cx.emit(AppEvent::SetWeatherData(weather_data));
            }
          });
        })
        .disabled(coords.is_none());
    });

    Binding::new(cx, AppData::weather_data, |cx, lens| {
      if let Some(forecast) = lens.get(cx) {
        HStack::new(cx, |cx| {
          VStack::new(cx, |cx| {
            Label::new(cx, "time");
            Label::new(cx, forecast.current.time);
          });
          VStack::new(cx, |cx| {
            Label::new(cx, "temp");
            Label::new(cx, forecast.current.temperature_2_m);
          });
          VStack::new(cx, |cx| {
            Label::new(cx, "wind speed");
            Label::new(cx, forecast.current.wind_speed_10_m);
          });
        });
      }
    })
  })
  .title("Rain 🌦️")
  .run()
}
