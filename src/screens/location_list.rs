use pollster::FutureExt as _;
use vizia::prelude::*;

use super::app_data::AppEvent;
use super::db_models::*;
use super::queries::get_all_locations;

#[derive(Debug)]
enum LocationListEvent {
  LoadLocations,
}

#[derive(Default, Debug, Clone, Lens)]
struct LocationListState {
  locations: Vec<Location>,
}

impl Model for LocationListState {
  fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    event.map(|location_list_event, _meta| match location_list_event {
      LocationListEvent::LoadLocations => match get_all_locations().block_on() {
        Ok(locations) => {
          println!("{:#?}", location_list_event);
          self.locations = locations
        }
        Err(_) => {}
      },
    })
  }
}

pub struct LocationList {}

impl View for LocationList {}

impl LocationList {
  pub fn new(cx: &mut Context) -> Handle<Self> {
    Self {}.build(cx, |cx| {
      LocationListState::default().build(cx);
      cx.emit(LocationListEvent::LoadLocations);
      Label::new(cx, "Saved Locations");
      Dropdown::new(
        cx,
        |cx| {
          Button::new(cx, |cx| Label::new(cx, "Saved Locations"))
            .on_press(|cx| cx.emit(PopupEvent::Switch));
        },
        |cx| {
          List::new(cx, LocationListState::locations, |cx, _, item| {
            let location = item.get(cx);
            HStack::new(cx, |cx| {
              Label::new(cx, location.name);
            });
          })
          .size(Pixels(500.0));
        },
      );
    })
  }
}
