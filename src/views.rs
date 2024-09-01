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

// INFO: https://www.nodc.noaa.gov/archive/arc0021/0002199/1.1/data/0-data/HTML/WMO-CODE/WMO4677.HTM
fn lookup_weather_text(weather_code: &i64) -> Option<&str> {
  match weather_code {
    // 00-49 	No precipitation at the station at the time of observation

    // 00-19 	No precipitation, fog, ice fog (except for 11 and 12), duststorm, sandstorm, drifting or blowing snow at the station* at the time of observation or, except for 09 and 17, during the preceding hour
    0 => Some("Cloud development not observed or not observable"),
    1 => Some("Clouds generally dissolving or becoming less developed"),
    2 => Some("State of sky on the whole unchanged"),
    3 => Some("Clouds generally forming or developing"),
    4 => Some("Visibility reduced by smoke, e.g. veldt or forest fires, industrial smoke or volcanic ashes"),
    5 => Some("Haze"),
    6 => Some("Widespread dust in suspension in the air, not raised by wind at or near the station at the time of observation"),
    7 => Some("Dust or sand raised by wind at or near the station at the time of observation, but no well developed dust whirl(s) or sand whirl(s), and no duststorm or sandstorm seen"),
    8 => Some("Well developed dust whirl(s) or sand whirl(s) seen at or near the station during the preceding hour or at the time ot observation, but no duststorm or sandstorm"),
    9 => Some("Duststorm or sandstorm within sight at the time of observation, or at the station during the preceding hour"),
    10 => Some("Mist"),
    11 => Some("Patches"),
    12 => Some("More or less continuous"),
    13 => Some("Lightning visible, no thunder heard"),
    14 => Some("Precipitation within sight, not reaching the ground or the surface of the sea"),
    15 => Some("Precipitation within sight, reaching the ground or the surface of the sea, but distant, i.e. estimated to be more than 5 km from the station"),
    16 => Some("Precipitation within sight, reaching the ground or the surface of the sea, near to, but not at the station"),
    17 => Some("Thunderstorm, but no precipitation at the time of observation"),
    18 => Some("Squalls"),
    19 => Some("Funnel cloud(s) (Tornado cloud or water-spout)"),

    // 20-29 	Precipitation, fog, ice fog or thunderstorm at the station during the preceding hour but not at the time of observation
    20 => Some("Drizzle (not freezing) or snow grains"), // not falling as shower(s)
    21 => Some("Rain (not freezing)"), // not falling as shower(s)
    22 => Some("Snow"), // not falling as shower(s)
    23 => Some("Rain and snow or ice pellets"), // not falling as shower(s)
    24 => Some("Freezing drizzle or freezing rain"), // not falling as shower(s)
    25 => Some("Shower(s) of rain"),
    26 => Some("Shower(s) of snow, or of rain and snow"),
    27 => Some("Shower(s) of hail, or of rain and hail"),
    28 => Some("Fog or ice fog"),
    29 => Some("Thunderstorm (with or without precipitation)"),

    // 30-39 	Duststorm, sandstorm, drifting or blowing snow
    30 => Some("Slight or moderate duststorm or sandstorm"),
    31 => Some("Slight or moderate duststorm or sandstorm"),
    32 => Some("Slight or moderate duststorm or sandstorm"),
    33 => Some("Severe duststorm or sandstorm"),
    34 => Some("Severe duststorm or sandstorm"),
    35 => Some("Severe duststorm or sandstorm"),
    36 => Some("Slight or moderate blowing snow"),
    37 => Some("Heavy drifting snow"),
    38 => Some("Slight or moderate blowing snow"),
    39 => Some("Heavy drifting snow"),

    // 40-49 	Fog or ice fog at the time of observation
    40 => Some("Fog or ice fog at a distance at the time of observation, but not at the station during the preceding hour, the fog or ice fog extending to a level above that of the observer"),
    41 => Some("Fog or ice fog in patches"),
    42 => Some("Fog or ice fog, sky visible"),
    43 => Some("Fog or ice fog, sky invisible"),
    44 => Some("Fog or ice fog, sky visible"),
    45 => Some("Fog or ice fog, sky invisible"),
    46 => Some("Fog or ice fog, sky visible"),
    47 => Some("Fog or ice fog, sky invisible"),
    48 => Some("Fog, depositing rime, sky visible"),
    49 => Some("Fog, depositing rime, sky invisible"),

    // 50-59 	Drizzle
    50 => Some("Drizzle, not freezing, intermittent (slight)"),
    51 => Some("Drizzle, not freezing, continuous (slight)"),
    52 => Some("Drizzle, not freezing, intermittent (moderate)"),
    53 => Some("Drizzle, not freezing, continuous (moderate)"),
    54 => Some("Drizzle, not freezing, intermittent (heavy)"),
    55 => Some("Drizzle, not freezing, continuous (heavy)"),
    56 => Some("Drizzle, freezing, slight"),
    57 => Some("Drizzle, freezing, moderate or heavy (dense)"),
    58 => Some("Drizzle and rain, slight"),
    59 => Some("Drizzle and rain, moderate or heavy"),

    // 60-69    Rain
    60 => Some("Rain, not freezing, intermittent (slight)"),
    61 => Some("Rain, not freezing, continuous (slight)"),
    62 => Some("Rain, not freezing, intermittent (moderate)"),
    63 => Some("Rain, not freezing, continuous (moderate)"),
    64 => Some("Rain, not freezing, intermittent (heavy)"),
    65 => Some("Rain, not freezing, continuous (heavy)"),
    66 => Some("Rain, freezing, slight"),
    67 => Some("Rain, freezing, moderate or heavy (dense)"),
    68 => Some("Rain or drizzle and snow, slight"),
    69 => Some("Rain or drizzle and snow, moderate or heavy"),

    // 70-79 Solid precipitation not in showers
    70 => Some("Intermittent fall of snowflakes (slight)"),
    71 => Some("Continuous fall of snowflakes (slight)"),
    72 => Some("Intermittent fall of snowflakes (moderate)"),
    73 => Some("Continuous fall of snowflakes (moderate)"),
    74 => Some("Intermittent fall of snowflakes (heavy)"),
    75 => Some("Continuous fall of snowflakes (heavy)"),
    76 => Some("Diamond dust (with or without fog)"),
    77 => Some("Snow grains (with or without fog)"),
    78 => Some("Isolated star-like snow crystals (with or without fog)"),
    79 => Some("Ice pellets"),

    // 80-99 Showery precipitation, or precipitation with current or recent thunderstorm 
    80 => Some("Rain shower(s), slight"),
    81 => Some("Rain shower(s), moderate or heavy"),
    82 => Some("Rain shower(s), violent"),
    83 => Some("Shower(s) of rain and snow mixed, slight"),
    84 => Some("Shower(s) of rain and snow mixed, moderate or heavy"),
    85 => Some("Snow shower(s), slight"),
    86 => Some("Snow shower(s), moderate or heavy"),
    87 => Some("Shower(s) of snow pellets or small hail, with or without rain or rain and snow mixed (slight)"),
    88 => Some("Shower(s) of snow pellets or small hail, with or without rain or rain and snow mixed (moderate or heavy)"),
    89 => Some("Shower(s) of hail, with or without rain or rain and snow mixed, not associated with thunder (slight)"),
    90 => Some("Shower(s) of hail, with or without rain or rain and snow mixed, not associated with thunder (heavy)"),
    91 => Some("Slight rain at time of observation"),
    92 => Some("Moderate or heavy rain at time of observation"),
    93 => Some("Slight snow, or rain and snow mixed or hail at time of observation"),
    94 => Some("Moderate or heavy snow, or rain and snow mixed or hail at time of observation"),
    95 => Some("Thunderstorm, slight or moderate, without hail but with rain and/or snow at time of observation"),
    96 => Some("Thunderstorm, slight or moderate, with hail at time of observation"),
    97 => Some("Thunderstorm, heavy, without hail but with rain and/or snow at time of observation"),
    98 => Some("Thunderstorm combined with duststorm or sandstorm at time of observation"),
    99 => Some("Thunderstorm, heavy, with hail at time of observation"),

    _ => None,
  }
}

fn lookup_weather_icon(weather_code: &i64) -> Option<&str> {
  match weather_code {
    0 => Some(icons::ICON_SUN),
    _ => None,
  }
}
