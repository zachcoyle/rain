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

const ITEM_SIZE: usize = 28;

impl LocationList {
  pub fn new(cx: &mut Context) -> Handle<Self> {
    Self {}.build(cx, |cx| {
      LocationListState::default().build(cx);
      cx.emit(LocationListEvent::LoadLocations);
      Label::new(cx, "Saved Locations");
      Binding::new(
        cx,
        LocationListState::locations.map(|x| x.len()),
        |cx, lens| {
          let item_count = lens.get(cx);
          Dropdown::new(
            cx,
            |cx| {
              Button::new(cx, |cx| Label::new(cx, "Saved Locations"))
                .on_press(|cx| cx.emit(PopupEvent::Switch));
            },
            move |cx| {
              ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                List::new(cx, LocationListState::locations, |cx, _, item| {
                  let location = item.get(cx);
                  Label::new(
                    cx,
                    location
                      .clone()
                      .name,
                  )
                  .size(Pixels(ITEM_SIZE as f32))
                  .on_press(move |cx| {
                    cx.emit(AppEvent::SelectLocation(location.clone()));
                    cx.emit(PopupEvent::Close);
                  });
                })
                .size(Pixels((ITEM_SIZE * item_count) as f32));
              })
              .height(Pixels(250.0));
            },
          );
        },
      );
    })
  }
}
