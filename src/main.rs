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

// latitude=39.7672
// longitude=-85.898
// current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,rain,showers,snowfall,weather_code,cloud_cover,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m
// hourly=temperature_2m,relative_humidity_2m,dew_point_2m,precipitation_probability,precipitation,rain,showers,snowfall,weather_code,visibility,wind_gusts_10m
// daily=weather_code,temperature_2m_max,temperature_2m_min,sunrise,sunset,daylight_duration,uv_index_max,precipitation_sum,rain_sum,snowfall_sum,precipitation_probability_max
// temperature_unit=fahrenheit
// forecast_hours=24
// wind_speed_unit=mph
// precipitation_unit=inch
// timezone=America%2FNew_York
// forecast_days=1
// past_hours=24
fn get_weather_data(coords: &LatLng) -> Option<Meteo> {
  let query = vec![
    ("latitude", coords.lat.to_string()),
    ("longitude", coords.lng.to_string()),
    ("current", String::from("temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,rain,showers,snowfall,weather_code,cloud_cover,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m")),
    ("hourly", String::from("temperature_2m,relative_humidity_2m,dew_point_2m,precipitation_probability,precipitation,rain,showers,snowfall,weather_code,visibility,wind_gusts_10m"),),
    ("daily", String::from("weather_code,temperature_2m_max,temperature_2m_min,sunrise,sunset,daylight_duration,uv_index_max,precipitation_sum,rain_sum,snowfall_sum,precipitation_probability_max")),
    ("temperature_unit", String::from("fahrenheit")),
    ("wind_speed_unit", String::from("mph")),
    ("precipitation_unit", String::from("inch")),
    ("timezone", String::from("America/New_York")),
    ("forecast_days", String::from("1")),
    ("forecast_hours", String::from("24")),
    ("past_hours", String::from("24")),
  ];

  let response = Client::new().get(BASE_URL).query(&query).send().ok()?;
  println!("{:?}", response);
  let json = response.json::<Meteo>();
  println!("{:?}", json);
  json.ok()
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
            // let time = forecast
            //   .clone()
            //   .current
            //   .and_then(|x| x.time)
            //   .or_else(|| Some(String::from("n/a")))
            //   .unwrap();
            // Label::new(cx, time);
            Label::new(cx, forecast.current.time);
          });
          VStack::new(cx, |cx| {
            Label::new(cx, "temp");
            // let temp_2_m = forecast
            //   .clone()
            //   .current
            //   .and_then(|x| x.temperature_2_m)
            //   .or_else(|| Some(0.0))
            //   .unwrap();
            // Label::new(cx, temp_2_m);
            Label::new(cx, forecast.current.temperature_2_m);
          });
          VStack::new(cx, |cx| {
            Label::new(cx, "wind speed");
            // let wind_speed_10_m = forecast
            //   .clone()
            //   .current
            //   .and_then(|x| x.wind_speed_10_m)
            //   .or_else(|| Some(0.0))
            //   .unwrap();
            // Label::new(cx, wind_speed_10_m);
            Label::new(cx, forecast.current.wind_speed_10_m);
          });
        });
      }
    })
  })
  .title("Rain 🌦️")
  .run()
}
