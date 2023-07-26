#[cfg(feature = "python")]
use pyo3::{prelude::*, wrap_pyfunction};

use cpy_binder::export_cpy;
use lazy_static::lazy_static;
use std::sync::Mutex;

struct NavigationManager {
    navigator: navigator_rs::Navigator,
}

lazy_static! {
    static ref NAVIGATOR: Mutex<Option<NavigationManager>> = Mutex::new(None);
}

impl NavigationManager {
    fn get_instance() -> &'static Mutex<Option<Self>> {
        if NAVIGATOR.lock().unwrap().is_none() {
            *NAVIGATOR.lock().unwrap() = Some(NavigationManager {
                navigator: navigator_rs::Navigator::new(),
            });
        }
        &NAVIGATOR
    }
}

impl From<AdcChannel> for navigator_rs::AdcChannel {
    fn from(channel: AdcChannel) -> Self {
        match channel {
            AdcChannel::Ch0 => navigator_rs::AdcChannel::Ch0,
            AdcChannel::Ch1 => navigator_rs::AdcChannel::Ch1,
            AdcChannel::Ch2 => navigator_rs::AdcChannel::Ch2,
            AdcChannel::Ch3 => navigator_rs::AdcChannel::Ch3,
        }
    }
}

impl From<PwmChannel> for navigator_rs::PwmChannel {
    fn from(channel: PwmChannel) -> Self {
        match channel {
            PwmChannel::Ch1 => navigator_rs::PwmChannel::Ch1,
            PwmChannel::Ch2 => navigator_rs::PwmChannel::Ch2,
            PwmChannel::Ch3 => navigator_rs::PwmChannel::Ch3,
            PwmChannel::Ch4 => navigator_rs::PwmChannel::Ch4,
            PwmChannel::Ch5 => navigator_rs::PwmChannel::Ch5,
            PwmChannel::Ch6 => navigator_rs::PwmChannel::Ch6,
            PwmChannel::Ch7 => navigator_rs::PwmChannel::Ch7,
            PwmChannel::Ch8 => navigator_rs::PwmChannel::Ch8,
            PwmChannel::Ch9 => navigator_rs::PwmChannel::Ch9,
            PwmChannel::Ch10 => navigator_rs::PwmChannel::Ch10,
            PwmChannel::Ch11 => navigator_rs::PwmChannel::Ch11,
            PwmChannel::Ch12 => navigator_rs::PwmChannel::Ch12,
            PwmChannel::Ch13 => navigator_rs::PwmChannel::Ch13,
            PwmChannel::Ch14 => navigator_rs::PwmChannel::Ch14,
            PwmChannel::Ch15 => navigator_rs::PwmChannel::Ch15,
            PwmChannel::Ch16 => navigator_rs::PwmChannel::Ch16,
            PwmChannel::All => navigator_rs::PwmChannel::All,
        }
    }
}

impl From<UserLed> for navigator_rs::UserLed {
    fn from(led: UserLed) -> Self {
        match led {
            UserLed::Led1 => navigator_rs::UserLed::Led1,
            UserLed::Led2 => navigator_rs::UserLed::Led2,
            UserLed::Led3 => navigator_rs::UserLed::Led3,
        }
    }
}

impl From<navigator_rs::AxisData> for AxisData {
    fn from(read_axis: navigator_rs::AxisData) -> Self {
        Self {
            x: read_axis.x,
            y: read_axis.y,
            z: read_axis.z,
        }
    }
}

impl From<navigator_rs::ADCData> for ADCData {
    fn from(read_adc: navigator_rs::ADCData) -> Self {
        Self {
            channel: [
                read_adc.channel[0],
                read_adc.channel[1],
                read_adc.channel[2],
                read_adc.channel[3],
            ],
        }
    }
}

export_cpy!(
    mod navigator {
        enum AdcChannel {
            Ch0,
            Ch1,
            Ch2,
            Ch3,
        }

        enum UserLed {
            Led1,
            Led2,
            Led3,
        }

        enum PwmChannel {
            Ch1,
            Ch2,
            Ch3,
            Ch4,
            Ch5,
            Ch6,
            Ch7,
            Ch8,
            Ch9,
            Ch10,
            Ch11,
            Ch12,
            Ch13,
            Ch14,
            Ch15,
            Ch16,
            All,
        }

        struct AxisData {
            x: f32,
            y: f32,
            z: f32,
        }

        struct ADCData {
            channel: [f32; 4],
        }

        fn init() -> () {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .init();
        }

        fn self_test() -> bool {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .self_test()
        }

        fn set_led(select: UserLed, state: bool) -> () {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_led(select.into(), state)
        }

        fn get_led(select: UserLed) -> bool {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .get_led(select.into())
        }

        fn set_led_toggle(select: UserLed) {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_led_toggle(select.into())
        }

        fn set_led_all(state: bool) -> () {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_led_all(state)
        }

        fn_c set_neopixel(rgb_array: *const [u8; 3], length: usize) {
            let array = unsafe {
                assert!(!rgb_array.is_null());
                std::slice::from_raw_parts(rgb_array, length)
            };
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_neopixel(array);
        }

        fn_py set_neopixel(rgb_array: Vec<[u8;3]>) {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_neopixel(&rgb_array)
        }

        fn read_adc_all() -> ADCData {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .read_adc_all()
                .into()
        }

        fn read_adc(channel: AdcChannel) -> f32 {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .read_adc(channel.into())
        }

        fn read_pressure() -> f32 {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .read_pressure()
        }

        fn read_temp() -> f32 {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .read_temperature()
        }

        fn read_mag() -> AxisData {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .read_mag()
                .into()
        }

        fn read_accel() -> AxisData {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .read_accel()
                .into()
        }

        fn read_gyro() -> AxisData {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .read_gyro()
                .into()
        }

        fn pwm_enable(state: bool) {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .pwm_enable(state)
        }

        fn set_pwm_freq_prescale(value: u8) {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_pwm_freq_prescale(value)
        }

        fn set_pwm_freq_hz(freq: f32) {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_pwm_freq_hz(freq)
        }

        fn set_pwm_channel_value(channel: PwmChannel, value: u16) {
            NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_pwm_channel_value(channel.into(), value)
        }

        fn_c set_pwm_channels_value(channels: *const PwmChannel, value:u16, length: usize) {
            let array_channels = unsafe {
                assert!(!channels.is_null());
                std::slice::from_raw_parts(channels, length)
            };
            for channel in array_channels.iter().take(length) {
                NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_pwm_channel_value(channel.clone().into(), value);
            }
        }

        fn_py set_pwm_channels_value(channels: Vec<PwmChannel>, value:u16) {
            for i in 0..channels.len() {
                NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_pwm_channel_value(channels[i].clone().into(), value);
            }
        }

        fn_c set_pwm_channels_values(channels: *const PwmChannel, values : *const u16, length: usize) {
            let array_channels = unsafe {
                assert!(!channels.is_null());
                std::slice::from_raw_parts(channels, length)
            };
            let array_values = unsafe {
                assert!(!values.is_null());
                std::slice::from_raw_parts(values, length)
            };
            for i in 0..length {
                NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_pwm_channel_value(array_channels[i].clone().into(), array_values[i]);
            }
        }

        fn_py set_pwm_channels_values(channels: Vec<PwmChannel>, values: Vec<u16>) {
            if (channels.len() != values.len()) {
                println!("The number of values is different from the number of PWM channels.");
                return
            }

            for i in 0..channels.len() {
                NavigationManager::get_instance()
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .navigator
                .set_pwm_channel_value(channels[i].clone().into(), values[i]);
            }
        }
    }
);
