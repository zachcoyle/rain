use vizia::prelude::*;

use super::app_data::AppEvent;

pub enum NewLocationFormEvent {
  SetName(String),
  SetGeohash(String),
  Submit,
}

#[derive(Default, Debug, Lens, Clone)]
pub struct NewLocationFormData {
  pub geohash: String,
  pub name: String,
  pub submitted: bool,
}

impl Model for NewLocationFormData {
  fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    event.map(
      |new_location_form_event, _meta| match new_location_form_event {
        NewLocationFormEvent::SetName(name) => {
          println!("NewLocationFormEvent::SetName({:#?})", name);
          self.name = name.to_string();
          println!("New State: {:#?}", self);
        }

        NewLocationFormEvent::SetGeohash(geohash) => {
          println!("NewLocationFormEvent::SetGeohash({:#?})", geohash);
          self.geohash = geohash.to_string();
          println!("New State: {:#?}", self);
        }

        NewLocationFormEvent::Submit => {
          println!("NewLocationFormEvent::Submit");
          self.submitted = true;
          println!("New State: {:#?}", self);
          cx.emit(AppEvent::ConfirmLocation(
            self
              .name
              .clone(),
            self
              .geohash
              .clone(),
          ))
        }
      },
    )
  }
}

pub struct NewLocationForm {}

impl NewLocationForm {
  pub fn new(cx: &mut Context) -> Handle<Self> {
    Self {}.build(cx, |cx| {
      NewLocationFormData::default().build(cx);
      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          Textbox::new(cx, NewLocationFormData::geohash).on_edit(|ex, geohash| {
            ex.emit(NewLocationFormEvent::SetGeohash(geohash));
          });
          Textbox::new(cx, NewLocationFormData::name).on_edit(|ex, name| {
            ex.emit(NewLocationFormEvent::SetName(name));
          });
        });
        Binding::new(cx, NewLocationFormData::geohash, |cx, lens| {
          let geohash = lens.get(cx);
          Binding::new(cx, NewLocationFormData::name, |cx, lens| {
            let name = lens.get(cx);
            Button::new(cx, |cx| Label::new(cx, "Submit")).on_press(|ex| {
              ex.emit(NewLocationFormEvent::Submit);
            });
          });
        });
      });
    })
  }
}

impl View for NewLocationForm {}
