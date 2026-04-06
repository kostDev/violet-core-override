use std::ffi::c_float;

#[repr(u8)]
#[derive(Debug)]
pub enum WeatherState
{
    Fine              = 0,
    Fog               = 1,
    LightRain = 3,
    MediumRain = 4,
    HeavyRain = 5,
    LightSnow = 6,
    MediumSnow = 7,
    HeavySnow = 8,
    LightSandStorm = 22,
    MediumSandStorm= 41,
    HeavySandStorm = 42,
    Thunders          = 86,
    BlackRain         = 90,
    BlackSnow         = 106
}


#[derive(Debug)]
pub enum WeatherType
{
    Fine       = 0,
    Rain       = 1,
    Snow       = 2,
    Storm      = 3,
    Thunders   = 86,
    BlackRain  = 90
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn violet_get_weather_state(m_type: WeatherType, m_grade: c_float) -> WeatherState {
    if m_grade < 0.27f32 {
        return WeatherState::Fine;
    }

    match m_type {
        WeatherType::Fine => WeatherState::Fine,
        WeatherType::Rain => {
            if m_grade < 0.4f32 {
                WeatherState::LightSnow
            } else if m_grade < 0.7f32 {
                WeatherState::MediumRain
            } else {
                WeatherState::HeavyRain
            }
        }
        WeatherType::Snow => {
            if m_grade < 0.4f32 {
                WeatherState::LightSnow
            } else if m_grade < 0.7f32 {
                WeatherState::MediumSnow
            } else {
                WeatherState::HeavySnow
            }
        }
        WeatherType::Storm => {
            if m_grade < 0.4f32 {
                WeatherState::LightSandStorm
            } else if m_grade < 0.7f32 {
                WeatherState::MediumSandStorm
            } else {
                WeatherState::HeavySandStorm
            }
        }
        WeatherType::Thunders => {
            WeatherState::Thunders
        }
        WeatherType::BlackRain => {
            WeatherState::BlackRain
        }
    }
    // println!("DEBUG [VioletCore]: Type: {} Grade: {} | Result: {:?}", m_type as u32, m_grade, result);
    // result
}
