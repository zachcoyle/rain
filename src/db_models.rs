use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Location {
  pub id: i64,
  pub name: String,
  pub geohash: String,
}

#[derive(FromRow, Debug)]
pub struct HistoricalForecast {
  pub id: i64,
  pub location_id: i64,
  pub response: String,
  pub timestamp: String,
  // TODO: figure out the idiomatic way to handle foreign key location_id
}
