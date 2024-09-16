use anyhow::Error;
use pollster::FutureExt as _;
use validator::{Validate, ValidationError};
use validator_struct::ValidatorStruct;
use vizia::prelude::*;

use super::{app_data::AppEvent, queries::add_location_to_db};

enum FormEvent {
  SetName(String),
  SetGeohash(String),
  Submit,
  SubmitError(Error),
  Validate,
}

#[derive(Default, Debug, Clone, Lens, Validate, ValidatorStruct)]
#[validator_struct(derive(Clone, PartialEq, PartialOrd))]
struct FormState {
  #[validate(custom(function = "validate_geohash"))]
  pub geohash: String,
  #[validate(length(min = 1))]
  pub name: String,
  pub submitting: bool,
  pub validation_errors: Option<FormStateError>,
  pub error_message: Option<String>,
}

impl vizia::binding::Data for FormStateError {
  fn same(&self, other: &Self) -> bool {
    self.geohash == other.geohash && self.name == other.name
  }
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
          println!("FormEvent::SetName({:#?})", name);
          self.name = name.to_string();
          println!("New State: {:#?}", self);
          cx.emit(FormEvent::Validate);
        }

        FormEvent::SetGeohash(geohash) => {
          println!("FormEvent::SetGeohash({:#?})", geohash);
          self.geohash = geohash.to_string();
          println!("New State: {:#?}", self);
          cx.emit(FormEvent::Validate);
        }

        FormEvent::Submit => {
          println!("FormEvent::Submit");
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
                cx.emit(FormEvent::SubmitError(e));
              }
            };
          }
          self.submitting = false;
        }

        FormEvent::SubmitError(e) => {
          println!("FormEvent::DisplayError");
          self.error_message = Some(e.to_string());
          println!("New State: {:#?}", self);
        }

        FormEvent::Validate => {
          println!("FormEvent::Validate");
          self.validation_errors = self
            .validate_struct()
            .err();
          println!("New State: {:#?}", self);
        }
      },
    )
  }
}

pub struct NewLocationForm {}

impl View for NewLocationForm {}

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

        Binding::new(cx, FormState::validation_errors, |cx, lens| {
          let errors = lens.get(cx);
          Label::new(cx, format!("Errors: {:#?}", errors));

          let has_errors = errors
            .map(|x| {
              x.geohash
                .is_some()
                || x
                  .name
                  .is_some()
            })
            .unwrap_or(false);
          Binding::new(cx, FormState::submitting, move |cx, lens| {
            let submitting = lens.get(cx);
            Button::new(cx, |cx| Label::new(cx, "Save Location"))
              .on_press(|ex| {
                ex.emit(FormEvent::Submit);
              })
              .disabled(submitting || has_errors);
          });
        });
      })
      .class("col");
    })
  }
}
