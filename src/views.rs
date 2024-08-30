use vizia::{
  context::Context,
  icons,
  view::{Handle, View},
  views::{Icon, Label, VStack},
};

pub struct DataCell {
  label: String,
  info: String,
  unit: String,
}

// TODO: i think i'm doing something 'wrong' here...
impl DataCell {
  pub fn new(cx: &mut Context, label: String, info: String, unit: String) -> Handle<Self> {
    Self {
      label: label.clone(),
      info: info.clone(),
      unit: unit.clone(),
    }
    .build(cx, |cx| {
      VStack::new(cx, |cx| {
        Label::new(cx, &label);
        Label::new(cx, format!("{}{}", &info, &unit));
      });
    })
  }
}

impl View for DataCell {}

pub struct WeatherCode {
  weather_code: i64,
}

impl WeatherCode {
  pub fn new(cx: &mut Context, weather_code: i64) -> Handle<Self> {
    Self { weather_code }.build(cx, |cx| {
      VStack::new(cx, |cx| {
        if let Some(icon_name) = lookup_weather_icon(&weather_code) {
          Icon::new(cx, icon_name);
        }
        let weather_text =
          lookup_weather_text(&weather_code).unwrap_or("weather description not found");
        Label::new(cx, format!("{}: {}", weather_code, weather_text));
      });
    })
  }
}

impl View for WeatherCode {}

fn lookup_weather_text(weather_code: &i64) -> Option<&str> {
  match weather_code {
    0 => Some("Sunny & Clear Skies"),
    _ => None,
  }
}

fn lookup_weather_icon(weather_code: &i64) -> Option<&str> {
  match weather_code {
    0 => Some(icons::ICON_SUN),
    _ => None,
  }
}
