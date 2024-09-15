use anyhow::Error;
use pollster::FutureExt as _;
use validator::{Validate, ValidationError, ValidationErrors};
use vizia::prelude::*;

use super::{app_data::AppEvent, queries::add_location_to_db};

pub enum NewLocationFormEvent {
  SetName(String),
  SetGeohash(String),
  Submit,
  FormError(Error),
}

#[derive(Default, Debug, Lens, Clone, Validate)]
struct FormState {
  #[validate(custom(function = "validate_geohash"))]
  pub geohash: String,
  #[validate(length(min = 1))]
  pub name: String,
  pub submitting: bool,
  pub show_validation_errors: bool,
  pub validation_errors: Option<ValidationErrors>,
  pub error_message: Option<String>,
}

fn validate_geohash(geohash: &str) -> Result<(), ValidationError> {
  match geohash::decode(geohash) {
    Ok(_) => Ok(()),
    Err(_) => Err(ValidationError::new("Invalid Geohash")),
  }
}

// fn validate(form_data: NewLocationFormData) -> bool {
//   form_data.name != ""
//     && form_data.geohash != ""
//     && geohash::decode(&form_data.geohash)
//       .ok()
//       .is_some()
// }

impl Model for FormState {
  fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    event.map(
      |new_location_form_event, _meta| match new_location_form_event {
        NewLocationFormEvent::SetName(name) => {
          println!("NewLocationFormEvent::SetName({:#?})", name);
          self.name = name.to_string();
          // self.valid = validate(self.clone());
          println!("New State: {:#?}", self);
        }

        NewLocationFormEvent::SetGeohash(geohash) => {
          println!("NewLocationFormEvent::SetGeohash({:#?})", geohash);
          self.geohash = geohash.to_string();
          self.validation_errors = self
            .validate()
            .err();
          println!("New State: {:#?}", self);
        }

        NewLocationFormEvent::Submit => {
          println!("NewLocationFormEvent::Submit");
          self.submitting = true;
          if self
            .validation_errors
            .is_none()
          {
            match add_location_to_db(&self.name, &self.geohash).block_on() {
              Ok(()) => cx.emit(AppEvent::ConfirmLocation(
                self
                  .name
                  .clone(),
                self
                  .geohash
                  .clone(),
              )),
              Err(e) => {
                cx.emit(NewLocationFormEvent::FormError(e));
              }
            };
          }
          self.submitting = false;
        }

        NewLocationFormEvent::FormError(e) => {
          println!("NewLocationFormEvent::DisplayError");
          self.error_message = Some(e.to_string());
          println!("New State: {:#?}", self);
        }
      },
    )
  }
}

pub struct NewLocationForm {}

impl NewLocationForm {
  pub fn new(cx: &mut Context) -> Handle<Self> {
    Self {}.build(cx, |cx| {
      FormState::default().build(cx);
      Binding::new(cx, FormState::error_message, |cx, lens| {
        if let Some(error_message) = lens.get(cx) {
          Label::new(cx, format!("Error: {}", error_message)).class("error");
        }
      });
      VStack::new(cx, |cx| {
        HStack::new(cx, |cx| {
          Textbox::new(cx, FormState::geohash)
            .on_edit(|ex, geohash| {
              ex.emit(NewLocationFormEvent::SetGeohash(geohash));
            })
            .class("form_input");
          Textbox::new(cx, FormState::name)
            .on_edit(|ex, name| {
              ex.emit(NewLocationFormEvent::SetName(name));
            })
            .class("form_input");
        })
        .class("row");

        Button::new(cx, |cx| Label::new(cx, "Save Location"))
          .on_press(|ex| {
            ex.emit(NewLocationFormEvent::Submit);
          })
          .disabled(false);
      })
      .class("col");
    })
  }
}

impl View for NewLocationForm {}
