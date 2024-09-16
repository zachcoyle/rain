use vizia::prelude::*;

use super::db_models::*;
use super::queries::get_all_locations;

struct LocationListState {
  locations: Vec<Location>,
}

pub struct LocationList {}

impl View for LocationList {}

impl LocationList {
  pub fn new(cx: &mut Context) -> Handle<Self> {
    Self {}.build(cx, |cx| {
      Label::new(cx, "Saved Locations");
    })
  }
}
