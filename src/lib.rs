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
        enum "
        Set of available options to select ADC's channel.
        "
        AdcChannel {
            Ch0,
            Ch1,
            Ch2,
            Ch3,
        }

        enum "
        Set of options to control navigator's LEDs.
        "
        UserLed {
            Led1,
            Led2,
            Led3,
        }

        enum "
        Set of available options to select PWM's channel.
        "
        PwmChannel {
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

        struct "
        The AxisData encapsulate values for the x, y, and z axes.
        "
        AxisData {
            x: f32,
            y: f32,
            z: f32,
        }

        struct "
        Encapsulates the value of ADC's four channels.
        "
        ADCData {
            channel: [f32; 4],
        }

        fn "
        Initialize the navigator module with default settings.

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> navigator.init()
        "
        init() -> () {
            with_navigator!()
                .init();
        }

        fn "
        Runs some tests on available sensors, then returns the result.

        Returns:
            bool: The true state means the sensors are OK.

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> navigator.self_test()
        "
        self_test() -> bool {
            with_navigator!()
                .self_test()
        }

        fn "
        Sets the selected navigator LED output.

        Args:
            select (:py:class:`UserLed`):  A pin to be selected.
            state (bool): The value of output, LED is on with a true logic value.

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> from bluerobotics_navigator import UserLed
            >>> navigator.set_led(UserLed.Led1, True)
        "
        set_led(select: UserLed, state: bool) -> () {
            with_navigator!().set_led(select.into(), state)
        }

        fn "
        Gets the selected navigator LED output state.

        Args:
            select (:py:class:`UserLed`):  A pin to be selected.

        Returns:
            bool: The true state means the LED is on.

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> from bluerobotics_navigator import UserLed
            >>> navigator.get_led(UserLed.Led1)
        "
        get_led(select: UserLed) -> bool {
            with_navigator!().get_led(select.into())
        }

        fn "
        Toggle the output of selected LED.

        Args:
            select (:py:class:`UserLed`):  A pin to be selected.

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> from bluerobotics_navigator import UserLed
            >>> navigator.set_led_toggle(UserLed.Led1)
        "
        set_led_toggle(select: UserLed) {
            with_navigator!().set_led_toggle(select.into())
        }

        fn "
        Set all LEDs on desired state ( Blue, Green and Red ).

        Args:
            state (bool): The value of output, LED is on with a true logic value.

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> from bluerobotics_navigator import UserLed
            >>> navigator.set_led_all(True)
        "
        set_led_all(state: bool) -> () {
            with_navigator!().set_led_all(state)
        }

        fn_c set_neopixel(rgb_array: *const [u8; 3], length: usize) {
            let array = unsafe {
                assert!(!rgb_array.is_null());
                std::slice::from_raw_parts(rgb_array, length)
            };
            with_navigator!().set_neopixel(array);
        }

        fn_py "
        Set the values of the neopixel LED array.

        Args:
            state ([[uint8, uint8, uint8]]): A 2D array containing RGB values for each LED. Each inner value representing the Red, Green and Blue from a LED.

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> navigator.set_neopixel([[100,0,0]])
        "
        set_neopixel(rgb_array: Vec<[u8;3]>) {
            with_navigator!().set_neopixel(&rgb_array)
        }

        fn "
        Reads the ADC based on ADS1115 of Navigator.

        Same as :py:func:`read_adc`, but it returns an array with all channel readings

        Returns:
            :py:class:`ADCData`: Measurements in [V].

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> navigator.read_adc_all().channel
        "
        read_adc_all() -> ADCData {
            with_navigator!().read_adc_all()
                .into()
        }

        fn "
        Reads the ADC based on ADS1115 of Navigator.

        Args:
            select (:py:class:`AdcChannel`):  A pin to be selected.

        Returns:
            float32: Measurements in [V].

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> from bluerobotics_navigator import AdcChannel
            >>> navigator.read_adc(AdcChannel.Ch1)
        "
        read_adc(channel: AdcChannel) -> f32 {
            with_navigator!().read_adc(channel.into())
        }

        fn "
        Reads the pressure based on BMP280 of Navigator.

        Returns:
            float32: Measurements in [kPa]

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> navigator.read_pressure()
        "
        read_pressure() -> f32 {
            with_navigator!().read_pressure()
        }

        fn "
        Reads the temperature using BMP280 of Navigator.

        Returns:
            float32: Measurements in [˚C]

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> navigator.read_temperature()
        "
        read_temp() -> f32 {
            with_navigator!().read_temperature()
        }

        fn "
        Reads the magnetometer Ak09915 of Navigator.

        Returns:
            :py:class:`AxisData`: Measurements in [µT]

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> AxisData = navigator.read_mag()
            >>> AxisData.x
            >>> AxisData.y
            >>> AxisData.z
        "
        read_mag() -> AxisData {
            with_navigator!().read_mag()
                .into()
        }

        fn "
        Reads acceleration based on ICM20689 of Navigator.

        Returns:
            :py:class:`AxisData`: Measurements in [m/s²]

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> AxisData = navigator.read_accel().x
            >>> AxisData.x
            >>> AxisData.y
            >>> AxisData.z
        "
        read_accel() -> AxisData {
            with_navigator!().read_accel()
                .into()
        }

        fn "
        Reads angular velocity based on ICM20689 of Navigator.

        Returns:
            :py:class:`AxisData`: Measurements in [rad/s]

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> AxisData = navigator.read_gyro().x
            >>> AxisData.x
            >>> AxisData.y
            >>> AxisData.z
        "
        read_gyro() -> AxisData {
            with_navigator!().read_gyro()
                .into()
        }

        fn "
        Sets the PWM IC to be enabled through firmware and OE_pin.

        Args:
            state (bool): The state of PWM output, it’s enabled with a true logic value.

        Examples:
            Please check :py:func:`set_pwm_channel_value`

            >>> navigator.pwm_enable(True)
        "
        pwm_enable(state: bool) {
            with_navigator!().pwm_enable(state)
        }

        fn "
        Sets the PWM frequency of Navigator.

        It changes the PRE_SCALE value on PCA9685.

        The prescaler value can be calculated for an update rate using the formula:

        prescale_value = round(clock_freq / (4096 * desired_freq)) - 1.

        If you want to control a servo, set a prescaler value of 100. This will correspond to a frequency of about 60 Hz, which is the frequency at which servos work.

        Notes:
            Re-run the set_pwm_channel_value() is required.

            The minimum prescaler value is 3, which corresponds to 1526 Hz. The maximum prescaler value is 255, which corresponds to 24 Hz.

            Internally, this function stops the oscillator and restarts it after setting the prescaler value if it was running.

        Args:
            value (uint8): The prescaler value (3..255).

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> navigator.set_pwm_freq_prescale(100)
            >>> navigator.set_pwm_channel_value(PwmChannel.Ch1, 2000)
            >>> navigator.pwm_enable(True)
        "
        set_pwm_freq_prescale(value: u8) {
            with_navigator!().set_pwm_freq_prescale(value)
        }

        fn "
            Sets the pwm frequency in Hertz of Navigator. Similar to :py:func:`set_pwm_freq_prescale`.

        Notes:
            The navigator module uses a crystal with a 24.5760 MHz clock.

        Args:
            freq (float32) : The prescaler value (24..1526)[Hz].

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> navigator.set_pwm_freq_hz(60)
            >>> navigator.set_pwm_channel_value(PwmChannel.Ch1, 2000)
            >>> navigator.pwm_enable(True)
        "
        set_pwm_freq_hz(freq: f32) {
            with_navigator!().set_pwm_freq_hz(freq)
        }

        fn "
        Sets the Duty Cycle (high value time) of selected channel.

        On PCA9685, this function sets the OFF counter and uses ON value as 0.

        Args:
            channel (:py:class:`PwmChannel`): The channel to be selected for PWM.
            value (u16) : Duty cycle value.

        Examples:
            >>> import bluerobotics_navigator as navigator
            >>> from bluerobotics_navigator import PwmChannel
            >>> navigator.init()
            >>> navigator.set_pwm_freq_hz(1000)
            >>> navigator.set_pwm_channel_value(PwmChannel.Ch1, 2000)
            >>> navigator.pwm_enable(True)
        "
        set_pwm_channel_value(channel: PwmChannel, value: u16) {
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

        fn_py "
        Like :py:func:`set_pwm_channel_value`. This function sets the Duty Cycle for a list of multiple channels.

        Args:
            channels ([:py:class:`PwmChannel`]): An array of channels to be selected for PWM.
            value (u16) : The duty cycle value.

        Examples:
            You can use this method like :py:func:`set_pwm_channel_value`.

            >>> navigator.set_pwm_channels_value([PwmChannel.Ch1, PwmChannel.Ch16], 1000)
        "
        set_pwm_channels_value(channels: Vec<PwmChannel>, value:u16) {
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

        fn_py "
        Like :py:func:`set_pwm_channel_value`. This function sets the Duty Cycle for a list of multiple channels with multiple values.

        Args:
            channels ([:py:class:`PwmChannel`]): An array of channels to be selected for PWM.
            values ([u16]) : An array Duty cycle value.

        Examples:
            You can use this method like :py:func:`set_pwm_channel_value`.

            >>> navigator.set_pwm_channels_values([PwmChannel.Ch1, PwmChannel.Ch5], [1000, 500])
        "
        set_pwm_channels_values(channels: Vec<PwmChannel>, values: Vec<u16>) {
            if channels.len() != values.len() {
                println!("The number of values is different from the number of PWM channels.");
                return
            }

            for i in 0..channels.len() {
                with_navigator!().set_pwm_channel_value(channels[i].clone().into(), values[i]);
            }
        }
    }
);
