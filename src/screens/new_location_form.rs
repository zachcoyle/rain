use anyhow::Error;
use pollster::FutureExt as _;
use validator::{Validate, ValidationError, ValidationErrors};
use vizia::prelude::*;

use super::{app_data::AppEvent, queries::add_location_to_db};

enum FormEvent {
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
  pub validation_errors: Option<ValidationErrors>,
  pub error_message: Option<String>,
}

fn validate_geohash(geohash: &str) -> Result<(), ValidationError> {
  if geohash == "" {
    return Err(ValidationError::new("Empty Geohash"));
  }
  match geohash::decode(geohash) {
    Ok(_) => Ok(()),
    Err(_) => Err(ValidationError::new("Invalid Geohash")),
  }
}

impl Model for FormState {
  fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    event.map(
      |new_location_form_event, _meta| match new_location_form_event {
        FormEvent::SetName(name) => {
          println!("NewLocationFormEvent::SetName({:#?})", name);
          self.name = name.to_string();
          // self.valid = validate(self.clone());
          println!("New State: {:#?}", self);
        }

        FormEvent::SetGeohash(geohash) => {
          println!("NewLocationFormEvent::SetGeohash({:#?})", geohash);
          self.geohash = geohash.to_string();
          self.validation_errors = self
            .validate()
            .err();
          println!("New State: {:#?}", self);
        }

        FormEvent::Submit => {
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
                cx.emit(FormEvent::FormError(e));
              }
            };
          }
          self.submitting = false;
        }

        FormEvent::FormError(e) => {
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
              ex.emit(FormEvent::SetGeohash(geohash));
            })
            .class("form_input");
          Textbox::new(cx, FormState::name)
            .on_edit(|ex, name| {
              ex.emit(FormEvent::SetName(name));
            })
            .class("form_input");
        })
        .class("row");

        let l = FormState::validation_errors.map(|x| {
          if let Some(y) = x {
            y.0
              .iter()
              .map(|x| format!("{:#?}", x.1))
              .reduce(|a, b| a + &b)
          } else {
            None
          }
        });

        Binding::new(cx, l, |cx, lens| {
          if let Some(errors) = lens.get(cx) {
            Label::new(cx, format!("Errors: {}", errors));
          };
        });

        Button::new(cx, |cx| Label::new(cx, "Save Location"))
          .on_press(|ex| {
            ex.emit(FormEvent::Submit);
          })
          .disabled(false);
      })
      .class("col");
    })
  }
}

impl View for NewLocationForm {}
