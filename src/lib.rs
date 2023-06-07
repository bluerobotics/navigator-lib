#[cfg(feature = "python")]
use pyo3::{prelude::*, wrap_pyfunction};

use cpy_binder::export_cpy;

struct NavigationManager {
    navigator: navigator_rs::Navigator,
}
impl NavigationManager {
    // private constructor to prevent direct instantiation
    fn new() -> NavigationManager {
        NavigationManager {
            navigator: navigator_rs::Navigator::new(),
        }
    }

    fn get_instance() -> &'static mut NavigationManager {
        static mut SINGLETON_INSTANCE: Option<NavigationManager> = None;

        // use unsafe block to ensure thread safety
        unsafe { SINGLETON_INSTANCE.get_or_insert_with(NavigationManager::new) }
    }
}

impl From<adc_Channel> for navigator_rs::adc_Channel {
    fn from(channel: adc_Channel) -> Self {
        match channel {
            adc_Channel::Ch0 => navigator_rs::adc_Channel::SingleA0,
            adc_Channel::Ch1 => navigator_rs::adc_Channel::SingleA1,
            adc_Channel::Ch2 => navigator_rs::adc_Channel::SingleA2,
            adc_Channel::Ch3 => navigator_rs::adc_Channel::SingleA3,
            adc_Channel::DiffCh0Ch1 => navigator_rs::adc_Channel::DifferentialA0A1,
            adc_Channel::DiffCh0C3h => navigator_rs::adc_Channel::DifferentialA0A3,
            adc_Channel::DiffCh1Ch3 => navigator_rs::adc_Channel::DifferentialA1A3,
            adc_Channel::DiffCh2Ch3 => navigator_rs::adc_Channel::DifferentialA2A3,
        }
    }
}

impl From<pwm_Channel> for navigator_rs::pwm_Channel {
    fn from(channel: pwm_Channel) -> Self {
        match channel {
            pwm_Channel::Ch0 => navigator_rs::pwm_Channel::C0,
            pwm_Channel::Ch1 => navigator_rs::pwm_Channel::C1,
            pwm_Channel::Ch2 => navigator_rs::pwm_Channel::C2,
            pwm_Channel::Ch3 => navigator_rs::pwm_Channel::C3,
            pwm_Channel::Ch4 => navigator_rs::pwm_Channel::C4,
            pwm_Channel::Ch5 => navigator_rs::pwm_Channel::C5,
            pwm_Channel::Ch6 => navigator_rs::pwm_Channel::C6,
            pwm_Channel::Ch7 => navigator_rs::pwm_Channel::C7,
            pwm_Channel::Ch8 => navigator_rs::pwm_Channel::C8,
            pwm_Channel::Ch9 => navigator_rs::pwm_Channel::C9,
            pwm_Channel::Ch10 => navigator_rs::pwm_Channel::C10,
            pwm_Channel::Ch11 => navigator_rs::pwm_Channel::C11,
            pwm_Channel::Ch12 => navigator_rs::pwm_Channel::C12,
            pwm_Channel::Ch13 => navigator_rs::pwm_Channel::C13,
            pwm_Channel::Ch14 => navigator_rs::pwm_Channel::C14,
            pwm_Channel::Ch15 => navigator_rs::pwm_Channel::C15,
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
        enum adc_Channel {
            Ch0,
            Ch1,
            Ch2,
            Ch3,
            DiffCh0Ch1,
            DiffCh0C3h,
            DiffCh1Ch3,
            DiffCh2Ch3,
        }

        enum pwm_Channel {
            Ch0,
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
        }

        struct AxisData {
            x: f32,
            y: f32,
            z: f32,
        }

        struct ADCData {
            channel: [i16; 4],
        }

        fn init() -> () {
            NavigationManager::get_instance().navigator.init()
        }

        fn set_led_on() -> () {
            NavigationManager::get_instance().navigator.set_led_on()
        }

        fn set_led_off() -> () {
            NavigationManager::get_instance().navigator.set_led_off()
        }

        fn read_adc_all() -> ADCData {
            NavigationManager::get_instance()
                .navigator
                .read_adc_all()
                .into()
        }

        fn read_adc(channel: adc_Channel) -> i16 {
            NavigationManager::get_instance()
                .navigator
                .read_adc(channel.into())
        }

        fn read_pressure() -> f32 {
            NavigationManager::get_instance().navigator.read_pressure()
        }

        fn read_temp() -> f32 {
            NavigationManager::get_instance()
                .navigator
                .read_temperature()
        }

        fn read_mag() -> AxisData {
            NavigationManager::get_instance()
                .navigator
                .read_mag()
                .into()
        }

        fn read_accel() -> AxisData {
            NavigationManager::get_instance()
                .navigator
                .read_accel()
                .into()
        }

        fn read_gyro() -> AxisData {
            NavigationManager::get_instance()
                .navigator
                .read_gyro()
                .into()
        }

        fn set_pwm_channel_value(channel: pwm_Channel, value: u16) -> () {
            NavigationManager::get_instance()
                .navigator
                .set_pwm_channel_value(channel.into(), value)
        }
    }
);
