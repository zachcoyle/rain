use geohash;
use sqlx::FromRow;
use vizia::binding::Data;
use vizia::prelude::*;

#[derive(Lens, FromRow, Data, Debug, Default, Clone)]
pub struct Location {
  pub id: i64,
  pub name: String,
  pub geohash: String,
}

impl Location {
  pub fn coords(&self) -> Option<(f64, f64)> {
    let (_, x, y) = geohash::decode(&self.geohash).ok()?;
    Some((y, x))
  }
}

#[derive(Lens, FromRow, Data, Debug, Default, Clone)]
pub struct HistoricalForecast {
  pub id: i64,
  pub location_id: i64,
  pub response: String,
  pub timestamp: String,
  // pub timestamp: DateTime<Utc>,
  // TODO: figure out the idiomatic way to handle foreign key location_id
}
