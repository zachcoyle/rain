use chrono::Utc;
use sqlx::{query_as, sqlite::SqlitePoolOptions};
use std::fs;
use xdg::BaseDirectories;

use crate::api_models::*;
use crate::db_models::*;

fn get_state_home() -> anyhow::Result<std::path::PathBuf> {
  let bd = BaseDirectories::with_prefix("rain")?;
  let state_home = bd.get_state_home();
  if !state_home.exists() {
    fs::create_dir(state_home.clone())?;
  }
  Ok(state_home)
}

async fn get_database_connection(
  state_home: std::path::PathBuf,
) -> Option<sqlx::Pool<sqlx::Sqlite>> {
  let path = state_home;
  let db_url = format!("sqlite://{}rain.db?mode=rwc", path.to_str()?);

  SqlitePoolOptions::new()
    .max_connections(1)
    .connect(&db_url)
    .await
    .ok()
}

pub async fn setup_database() -> anyhow::Result<()> {
  let state_home = get_state_home()?;
  if let Some(pool) = get_database_connection(state_home).await {
    let _ = sqlx::migrate!()
      .run(&pool)
      .await?;
  }
  Ok(())
}

pub async fn get_all_locations() -> anyhow::Result<Vec<Location>> {
  let state_home = get_state_home()?;
  // if let Some(pool) = get_database_connection(state_home).await {
  //   let all_locations = sqlx::query_as::<_, Location>("select * from Location;")
  //     .fetch_all(&pool)
  //     .await?;
  //   Ok(all_locations)
  // } else {
  //   Err(anyhow::anyhow!("got no locations from db"))
  // }
  Err(anyhow::anyhow!(
    "get_all_locations currently de-implemented"
  ))
}

pub async fn get_latest_location() -> anyhow::Result<Location> {
  // let state_home = get_state_home()?;
  // if let Some(pool) = get_database_connection(state_home).await {
  //   let latest_loc = query_as!(
  //     Location,
  //     "
  //       select
  //         *
  //       from
  //         Location
  //       limit
  //         1;
  //     ",
  //   )
  //   .fetch_one(&pool)
  //   .await?;
  //
  //   Ok(latest_loc)
  // } else {
  //   Err(anyhow::anyhow!("could not get db connection"))
  // }
  Err(anyhow::anyhow!(
    "get_latest_location currently de-implemented"
  ))
}

pub async fn get_latest_historical_forecast(location: i64) -> anyhow::Result<HistoricalForecast> {
  // let state_home = get_state_home()?;
  // if let Some(pool) = get_database_connection(state_home).await {
  //   let x = sqlx::query_as!(
  //     HistoricalForecast,
  //     "
  //       select
  //         *
  //       from
  //         HistoricalForecast
  //       where
  //         location_id = ?
  //       limit
  //         1
  //     ",
  //     location
  //   )
  //   .fetch_one(&pool)
  //   .await?;
  //   Ok(x)
  // } else {
  //   Err(anyhow::anyhow!("could not get db connection"))
  // }
  Err(anyhow::anyhow!(
    "get_latest_historical_forecast currently de-implemented"
  ))
}

pub async fn add_forecast_to_db(location: &Location, meteo: &Meteo) -> anyhow::Result<()> {
  let forecast_json_string = serde_json::to_string(meteo)?;
  let state_home = get_state_home()?;

  if let Some(pool) = get_database_connection(state_home).await {
    let _ = sqlx::query(
      "
        insert into
          HistoricalForecast (location_id, response, timestamp)
        values
          (?, ?, ?);
      ",
    )
    .bind(location.id)
    .bind(forecast_json_string)
    .bind(Utc::now().to_string())
    .execute(&pool)
    .await?;
  }
  Ok(())
}

pub async fn add_location_to_db(name: &str, geohash: &str) -> anyhow::Result<()> {
  let state_home = get_state_home()?;
  if let Some(pool) = get_database_connection(state_home).await {
    let _ = sqlx::query(
      "
        insert into
          Location (geohash, name)
        values
          (?, ?);
    ",
    )
    .bind(geohash)
    .bind(name)
    .execute(&pool)
    .await?;
  }
  Ok(())
}
