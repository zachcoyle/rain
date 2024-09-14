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
  pub valid: bool,
  pub submitted: bool,
}

fn validate(form_data: NewLocationFormData) -> bool {
  form_data.name != ""
    && form_data.geohash != ""
    && geohash::decode(&form_data.geohash)
      .ok()
      .is_some()
}

impl Model for NewLocationFormData {
  fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    event.map(
      |new_location_form_event, _meta| match new_location_form_event {
        NewLocationFormEvent::SetName(name) => {
          println!("NewLocationFormEvent::SetName({:#?})", name);
          self.name = name.to_string();
          self.valid = validate(self.clone());
          println!("New State: {:#?}", self);
        }

        NewLocationFormEvent::SetGeohash(geohash) => {
          println!("NewLocationFormEvent::SetGeohash({:#?})", geohash);
          self.geohash = geohash.to_string();
          self.valid = validate(self.clone());
          println!("New State: {:#?}", self);
        }

        NewLocationFormEvent::Submit => {
          println!("NewLocationFormEvent::Submit");
          if self.valid {
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
        })
        .class("location_form_hstack");
        Binding::new(cx, NewLocationFormData::valid, |cx, lens| {
          let valid = lens.get(cx);
          Button::new(cx, |cx| Label::new(cx, "Save Location"))
            .on_press(|ex| {
              ex.emit(NewLocationFormEvent::Submit);
            })
            .disabled(!valid);
        });
      })
      .class("location_form_vstack");
    })
  }
}

impl View for NewLocationForm {}
