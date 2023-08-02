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

macro_rules! with_navigator {
    () => {
        NavigationManager::get_instance()
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .navigator
    };
}

macro_rules! impl_from_enum {
    ($from:ty, $to:ty, $($variant:ident),+ $(,)?) => {
        impl From<$from> for $to {
            fn from(item: $from) -> Self {
                match item {
                    $(
                        <$from>::$variant => <$to>::$variant,
                    )+
                }
            }
        }
    };
}

// Help with conversion from navigator enum API to our stable API
impl_from_enum!(AdcChannel, navigator_rs::AdcChannel, Ch0, Ch1, Ch2, Ch3);
impl_from_enum!(
    PwmChannel,
    navigator_rs::PwmChannel,
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
    All
);
impl_from_enum!(UserLed, navigator_rs::UserLed, Led1, Led2, Led3);

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
    mod bluerobotics_navigator {
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
            with_navigator!()
                .init();
        }

        fn self_test() -> bool {
            with_navigator!()
                .self_test()
        }

        fn set_led(select: UserLed, state: bool) -> () {
            with_navigator!().set_led(select.into(), state)
        }

        fn get_led(select: UserLed) -> bool {
            with_navigator!().get_led(select.into())
        }

        fn set_led_toggle(select: UserLed) {
            with_navigator!().set_led_toggle(select.into())
        }

        fn set_led_all(state: bool) -> () {
            with_navigator!().set_led_all(state)
        }

        fn_c set_neopixel(rgb_array: *const [u8; 3], length: usize) {
            let array = unsafe {
                assert!(!rgb_array.is_null());
                std::slice::from_raw_parts(rgb_array, length)
            };
            with_navigator!().set_neopixel(array);
        }

        fn_py set_neopixel(rgb_array: Vec<[u8;3]>) {
            with_navigator!().set_neopixel(&rgb_array)
        }

        fn read_adc_all() -> ADCData {
            with_navigator!().read_adc_all()
                .into()
        }

        fn read_adc(channel: AdcChannel) -> f32 {
            with_navigator!().read_adc(channel.into())
        }

        fn read_pressure() -> f32 {
            with_navigator!().read_pressure()
        }

        fn read_temp() -> f32 {
            with_navigator!().read_temperature()
        }

        fn read_mag() -> AxisData {
            with_navigator!().read_mag()
                .into()
        }

        fn read_accel() -> AxisData {
            with_navigator!().read_accel()
                .into()
        }

        fn read_gyro() -> AxisData {
            with_navigator!().read_gyro()
                .into()
        }

        fn pwm_enable(state: bool) {
            with_navigator!().pwm_enable(state)
        }

        fn set_pwm_freq_prescale(value: u8) {
            with_navigator!().set_pwm_freq_prescale(value)
        }

        fn set_pwm_freq_hz(freq: f32) {
            with_navigator!().set_pwm_freq_hz(freq)
        }

        fn set_pwm_channel_value(channel: PwmChannel, value: u16) {
            with_navigator!().set_pwm_channel_value(channel.into(), value)
        }

        fn_c set_pwm_channels_value(channels: *const PwmChannel, value:u16, length: usize) {
            let array_channels = unsafe {
                assert!(!channels.is_null());
                std::slice::from_raw_parts(channels, length)
            };
            for channel in array_channels.iter().take(length) {
                with_navigator!().set_pwm_channel_value(channel.clone().into(), value);
            }
        }

        fn_py set_pwm_channels_value(channels: Vec<PwmChannel>, value:u16) {
            for i in 0..channels.len() {
                with_navigator!().set_pwm_channel_value(channels[i].clone().into(), value);
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
                with_navigator!().set_pwm_channel_value(array_channels[i].clone().into(), array_values[i]);
            }
        }

        fn_py set_pwm_channels_values(channels: Vec<PwmChannel>, values: Vec<u16>) {
            if (channels.len() != values.len()) {
                println!("The number of values is different from the number of PWM channels.");
                return
            }

            for i in 0..channels.len() {
                with_navigator!().set_pwm_channel_value(channels[i].clone().into(), values[i]);
            }
        }
    }
);
