use std::fs;

use pollster::FutureExt as _;
use reqwest::Client;
use sqlx::sqlite::SqlitePoolOptions;
use vizia::{context::Context, icons::ICON_SUN, prelude::*};
use xdg::BaseDirectories;

mod db_models;
use db_models::*;

mod api_models;
use api_models::*;

mod views;
use views::*;

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

.weather_icon {
  height: 50px;
  width: 50px;
}
"#;

const BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Default, Debug, Lens)]
pub struct AppData {
  weather_data: Option<Meteo>,
  geohash: String,
  latlng: Option<LatLng>,
  location_confirmed: bool,
}

pub enum AppEvent {
  SetWeatherData(Option<Meteo>),
  UpdateGeohash(String),
  ConfirmLocation,
  Rehydrate(Location),
}

impl Model for AppData {
  fn event(&mut self, ex: &mut EventContext, event: &mut Event) {
    event.map(|app_event, _meta| match app_event {
      AppEvent::SetWeatherData(meteo) => {
        println!("AppEvent::SetWeatherData({:#?})", meteo);
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
        self.location_confirmed = true;
        let add_result = add_location_to_db("yee", &self.geohash).block_on();
        println!("add result: {:?}", add_result);
        if let Some(ll) = self.latlng {
          let weather_data = get_weather_data(&ll);
          let _ = ex.emit(AppEvent::SetWeatherData(weather_data));
        }
      }

      AppEvent::Rehydrate(loc) => {
        println!("AppEvent::Rehydrate({:#?})", loc);
        self.geohash = loc
          .geohash
          .clone();
        // TODO: clean this up
        if let Some((geohash::Coord { x, y }, _, _)) = geohash::decode(&loc.geohash).ok() {
          // Some(LatLng { lat: y, lng: x })
          self.latlng = Some(LatLng { lat: y, lng: x });
          ex.emit(AppEvent::ConfirmLocation);
        }
      }
    });
  }
}

async fn rehydrate_from_db(cx: &mut Context) -> anyhow::Result<()> {
  let state_home = get_state_home()?;
  if let Some(pool) = get_database_connection(state_home).await {
    let query_result = sqlx::query_as!(
      Location,
      r#"
select
  *
from
  Location
limit
  1;
"#,
    )
    .fetch_one(&pool)
    .await?;

    cx.emit(AppEvent::Rehydrate(query_result));
  }

  Ok(())
}

async fn add_location_to_db(name: &str, geohash: &str) -> anyhow::Result<()> {
  println!("INSIDE ADD_LOCATION_TO_DB");
  let state_home = get_state_home()?;
  if let Some(pool) = get_database_connection(state_home).await {
    let query_result = sqlx::query(
      r#"
insert into
  Location (geohash, name)
values
  (?, ?);
"#,
    )
    .bind(geohash)
    .bind(name)
    .execute(&pool)
    .await?;

    println!("query_result: {:?}", query_result);
  }
  Ok(())
}

fn get_state_home() -> anyhow::Result<std::path::PathBuf> {
  let bd = BaseDirectories::with_prefix("rain")?;
  let state_home = bd.get_state_home();
  if !state_home.exists() {
    fs::create_dir(state_home.clone())?;
  }
  Ok(state_home)
}

// refactor me
async fn get_database_connection(
  state_home: std::path::PathBuf,
) -> Option<sqlx::Pool<sqlx::Sqlite>> {
  println!("getting database connection");
  let path = state_home;
  println!("path exists");
  let db_url = format!("sqlite://{}rain.db?mode=rwc", path.to_str()?);
  println!("db url: {}", db_url);
  let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect(&db_url)
    .await;
  println!("pool result: {:?}", pool);
  Some(pool.ok()?)
}

// refactor me
async fn setup_database() -> anyhow::Result<()> {
  println!("starting db setup");
  let state_home = get_state_home()?;
  if let Some(pool) = get_database_connection(state_home).await {
    println!("pool exists");
    let migrate_result = sqlx::migrate!()
      .run(&pool)
      .await;
    println!("migration successful? {:?}", migrate_result);
    migrate_result?;
  }

  Ok(())
}

fn get_weather_data(coords: &LatLng) -> Option<Meteo> {
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
    (
      "latitude",
      coords
        .lat
        .to_string(),
    ),
    (
      "longitude",
      coords
        .lng
        .to_string(),
    ),
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

  let response = Client::new()
    .get(BASE_URL)
    .query(&query)
    .send()
    .block_on()
    .ok()?;
  println!("{:#?}", response);
  let json = response.json::<Meteo>();
  // println!("{:#?}", json);
  json
    .block_on()
    .ok()
}

fn convert_geohash_to_coords(gh: &str) -> Option<LatLng> {
  if gh == "" {
    return None;
  }
  let (geohash::Coord { x, y }, _, _) = geohash::decode(gh).ok()?;
  Some(LatLng { lat: y, lng: x })
}

#[tokio::main]
async fn main() -> Result<(), vizia::ApplicationError> {
  env_logger::init();
  let _ = setup_database().await;

  Application::new(|cx| {
    let _ = rehydrate_from_db(cx).block_on();

    cx.add_stylesheet(STYLE)
      .expect("Failed to add stylesheet");

    AppData::default().build(cx);

    Textbox::new(cx, AppData::geohash)
      .on_edit(|ex, new_geohash| ex.emit(AppEvent::UpdateGeohash(new_geohash)));

    Label::new(cx, AppData::geohash);
    Label::new(cx, AppData::latlng.map(|x| format!("{:?}", x)));

    Binding::new(cx, AppData::latlng, |cx, lens| {
      let coords = lens.get(cx);
      Button::new(cx, |cx| Label::new(cx, "Get Weather!"))
        .on_press(move |ex| {
          ex.emit(AppEvent::ConfirmLocation);
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
