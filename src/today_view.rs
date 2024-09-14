use vizia::prelude::*;

use super::api_models::Meteo;
use super::views::lookup_weather_text;

pub struct ForecastScreen {
  forecast: Meteo,
}

impl ForecastScreen {
  pub fn new(cx: &mut Context, forecast: Meteo) -> Handle<Self> {
    Self {
      forecast: forecast.clone(),
    }
    .build(cx, |cx| {
      Label::new(cx, "Right now:").class("title");
      Label::new(
        cx,
        lookup_weather_text(
          &forecast
            .current
            .weather_code,
        )
        .unwrap_or("unknown weather 😱"),
      );
    })
  }
}

impl View for ForecastScreen {}
