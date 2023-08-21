use cpy_binder::{cpy_enum, cpy_fn, cpy_fn_c, cpy_fn_py, cpy_module, cpy_struct};

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

#[cpy_enum]
#[comment = "Set of available options to select ADC's channel."]
enum AdcChannel {
    Ch0,
    Ch1,
    Ch2,
    Ch3,
}

#[cpy_enum]
#[comment = "Set of options to control navigator's LEDs."]
enum UserLed {
    Led1,
    Led2,
    Led3,
}

#[cpy_enum]
#[comment = "Set of available options to select PWM's channel."]
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

#[cpy_struct]
#[comment = "The AxisData encapsulate values for the x, y, and z axes."]
struct AxisData {
    x: f32,
    y: f32,
    z: f32,
}

#[cpy_struct]
#[comment = "Encapsulates the value of ADC's four channels."]
struct ADCData {
    channel: [f32; 4],
}

#[cpy_fn]
#[comment_c = "Initialize the navigator module with default settings."]
#[comment_py = "Initialize the navigator module with default settings.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.init()"]
fn init() {
    with_navigator!().init();
}

#[cpy_fn]
#[comment_c = "Runs some tests on available sensors, then returns the result."]
#[comment_py = "Runs some tests on available sensors, then returns the result.\n
    Returns:\n
        bool: The true state means the sensors are OK.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.self_test()"]
fn self_test() -> bool {
    with_navigator!().self_test()
}

#[cpy_fn]
#[comment_c = "Sets the selected navigator LED output."]
#[comment_py = "Sets the selected navigator LED output.\n
    Args:\n
        select (:py:class:`UserLed`):  A pin to be selected.\n
        state (bool): The value of output, LED is on with a true logic value.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import UserLed\n
        >>> navigator.set_led(UserLed.Led1, True)"]
fn set_led(select: UserLed, state: bool) {
    with_navigator!().set_led(select.into(), state)
}

#[cpy_fn]
#[comment_c = "Gets the selected navigator LED output state."]
#[comment_py = "Gets the selected navigator LED output state.\n\n
    Args:\n
        select (:py:class:`UserLed`):  A pin to be selected.\n
    Returns:\n
        bool: The true state means the LED is on.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import UserLed\n
        >>> navigator.get_led(UserLed.Led1)"]
fn get_led(select: UserLed) -> bool {
    with_navigator!().get_led(select.into())
}

#[cpy_fn]
#[comment_c = "Toggle the output of selected LED."]
#[comment_py = "Toggle the output of selected LED.\n\n
    Args:\n
        select (:py:class:`UserLed`):  A pin to be selected.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import UserLed\n
        >>> navigator.set_led_toggle(UserLed.Led1)"]
fn set_led_toggle(select: UserLed) {
    with_navigator!().set_led_toggle(select.into())
}

#[cpy_fn]
#[comment_c = "Set all LEDs on desired state ( Blue, Green and Red )."]
#[comment_py = "Set all LEDs on desired state ( Blue, Green and Red ).\n
    Args:\n
        state (bool): The value of output, LED is on with a true logic value.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import UserLed\n
        >>> navigator.set_led_all(True)"]
fn set_led_all(state: bool) {
    with_navigator!().set_led_all(state)
}

#[cpy_fn_c]
#[comment = "Set the values of the neopixel LED array."]
fn set_neopixel_c(rgb_array: *const [u8; 3], length: usize) {
    let array = unsafe {
        assert!(!rgb_array.is_null());
        std::slice::from_raw_parts(rgb_array, length)
    };
    with_navigator!().set_neopixel(array);
}

#[cpy_fn_py]
#[comment = "Set the values of the neopixel LED array for Python.\n
    Args:\n
        state ([[uint8, uint8, uint8]]): A 2D array containing RGB values for each LED.\n
            Each inner value representing the Red, Green and Blue from a LED.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.set_neopixel([[100,0,0]])"]
fn set_neopixel_py(rgb_array: Vec<[u8; 3]>) {
    with_navigator!().set_neopixel(&rgb_array)
}

#[cpy_fn]
#[comment_c = "Reads the ADC based on ADS1115 of Navigator."]
#[comment_py = "Reads the ADC based on ADS1115 of Navigator.\n
    Same as :py:func:`read_adc`, but it returns an array with all channel readings\n
    Returns:\n
        :py:class:`ADCData`: Measurements in [V].\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.read_adc_all().channel"]
fn read_adc_all() -> ADCData {
    with_navigator!().read_adc_all().into()
}

#[cpy_fn]
#[comment_c = "Reads the ADC based on ADS1115 of Navigator."]
#[comment_py = "Reads the ADC based on ADS1115 of Navigator.\n\n
    Args:\n
        select (:py:class:`AdcChannel`):  A pin to be selected.\n
    Returns:\n
        float32: Measurements in [V].\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import AdcChannel\n
        >>> navigator.read_adc(AdcChannel.Ch1)"]
fn read_adc(channel: AdcChannel) -> f32 {
    with_navigator!().read_adc(channel.into())
}

#[cpy_fn]
#[comment_c = "Reads the pressure based on BMP280 of Navigator."]
#[comment_py = "Reads the pressure based on BMP280 of Navigator.\n
    Returns:\n
        float32: Measurements in [kPa]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.read_pressure()"]
fn read_pressure() -> f32 {
    with_navigator!().read_pressure()
}

#[cpy_fn]
#[comment_c = "Reads the temperature using BMP280 of Navigator."]
#[comment_py = "Reads the temperature using BMP280 of Navigator.\n
    Returns:\n
        float32: Measurements in [˚C]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.read_temperature()"]
fn read_temp() -> f32 {
    with_navigator!().read_temperature()
}

#[cpy_fn]
#[comment_c = "Reads the magnetometer Ak09915 of Navigator."]
#[comment_py = "Reads the magnetometer Ak09915 of Navigator.\n
    Returns:\n
        :py:class:`AxisData`: Measurements in [µT]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> AxisData = navigator.read_mag()\n
        >>> AxisData.x\n
        >>> AxisData.y\n
        >>> AxisData.z"]
fn read_mag() -> AxisData {
    with_navigator!().read_mag().into()
}

#[cpy_fn]
#[comment_c = "Reads acceleration based on ICM20689 of Navigator."]
#[comment_py = "Reads acceleration based on ICM20689 of Navigator.\n
    Returns:\n
        :py:class:`AxisData`: Measurements in [m/s²]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> AxisData = navigator.read_accel()\n
        >>> AxisData.x\n
        >>> AxisData.y\n
        >>> AxisData.z"]
fn read_accel() -> AxisData {
    with_navigator!().read_accel().into()
}

#[cpy_fn]
#[comment_c = "Reads angular velocity based on ICM20689 of Navigator."]
#[comment_py = "Reads angular velocity based on ICM20689 of Navigator.\n
    Returns:\n
        :py:class:`AxisData`: Measurements in [rad/s]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> AxisData = navigator.read_gyro()\n
        >>> AxisData.x\n
        >>> AxisData.y\n
        >>> AxisData.z"]
fn read_gyro() -> AxisData {
    with_navigator!().read_gyro().into()
}

#[cpy_fn]
#[comment_c = "Sets the PWM IC to be enabled through firmware and OE_pin."]
#[comment_py = "Sets the PWM IC to be enabled through firmware and OE_pin.\n
    Args:\n
        state (bool): The state of PWM output, it’s enabled with a true logic value.\n
    Examples:\n
        Please check :py:func:`set_pwm_channel_value`\n
        >>> navigator.pwm_enable(True)"]
fn pwm_enable(state: bool) {
    with_navigator!().pwm_enable(state)
}

#[cpy_fn]
#[comment_c = "Sets the PWM frequency of Navigator."]
#[comment_py = "Sets the PWM frequency of Navigator.\n\n
    It changes the PRE_SCALE value on PCA9685.\n
    The prescaler value can be calculated for an update rate using the formula:\n
    prescale_value = round(clock_freq / (4096 * desired_freq)) - 1.\n
    If you want to control a servo, set a prescaler value of 100.\n
    This will correspond to a frequency of about 60 Hz, which is the frequency at which servos work.\n
    Notes:\n
        Re-run the set_pwm_channel_value() is required.\n
        The minimum prescaler value is 3, which corresponds to 1526 Hz.\n
        The maximum prescaler value is 255, which corresponds to 24 Hz.\n
        Internally, this function stops the oscillator and restarts it after setting the prescaler value\n
        if it was running.\n
    Args:\n
        value (uint8): The prescaler value (3..255).\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.set_pwm_freq_prescale(100)\n
        >>> navigator.set_pwm_channel_value(PwmChannel.Ch1, 2000)\n
        >>> navigator.pwm_enable(True)"]
fn set_pwm_freq_prescale(value: u8) {
    with_navigator!().set_pwm_freq_prescale(value)
}

#[cpy_fn]
#[comment_c = "Sets the pwm frequency in Hertz of Navigator."]
#[comment_py = "Sets the pwm frequency in Hertz of Navigator. Similar to :py:func:`set_pwm_freq_prescale`.\n
    Notes:\n
        The navigator module uses a crystal with a 24.5760 MHz clock.\n
    Args:\n
        freq (float32) : The prescaler value (24..1526)[Hz].\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.set_pwm_freq_hz(60)\n
        >>> navigator.set_pwm_channel_value(PwmChannel.Ch1, 2000)\n
        >>> navigator.pwm_enable(True)"]
fn set_pwm_freq_hz(freq: f32) {
    with_navigator!().set_pwm_freq_hz(freq)
}

#[cpy_fn]
#[comment_c = "Sets the Duty Cycle (high value time) of selected channel."]
#[comment_py = "Sets the Duty Cycle (high value time) of selected channel.\n
    On PCA9685, this function sets the OFF counter and uses ON value as 0.\n
    Args:\n
        channel (:py:class:`PwmChannel`): The channel to be selected for PWM.\n
        value (u16) : Duty cycle value.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import PwmChannel\n
        >>> navigator.init()\n
        >>> navigator.set_pwm_freq_hz(1000)\n
        >>> navigator.set_pwm_channel_value(PwmChannel.Ch1, 2000)\n
        >>> navigator.pwm_enable(True)"]
fn set_pwm_channel_value(channel: PwmChannel, value: u16) {
    with_navigator!().set_pwm_channel_value(channel.into(), value)
}

#[cpy_fn_c]
#[comment = "Sets the Duty Cycle for a list of multiple channels."]
fn set_pwm_channels_value_c(channels: *const PwmChannel, value: u16, length: usize) {
    let array_channels = unsafe {
        assert!(!channels.is_null());
        std::slice::from_raw_parts(channels, length)
    };
    for channel in array_channels.iter().take(length) {
        with_navigator!().set_pwm_channel_value(channel.clone().into(), value);
    }
}

#[cpy_fn_py]
#[comment = "Like :py:func:`set_pwm_channel_value`. This function sets the Duty Cycle for a list of multiple channels.\n
    Args:\n
        channels ([:py:class:`PwmChannel`]): An array of channels to be selected for PWM.\n
        value (u16) : The duty cycle value.\n
    Examples:\n
        You can use this method like :py:func:`set_pwm_channel_value`.\n
        >>> navigator.set_pwm_channels_value([PwmChannel.Ch1, PwmChannel.Ch16], 1000)"]
fn set_pwm_channels_value_py(channels: Vec<PwmChannel>, value: u16) {
    for i in 0..channels.len() {
        with_navigator!().set_pwm_channel_value(channels[i].clone().into(), value);
    }
}

#[cpy_fn_c]
#[comment = "Sets the Duty Cycle for a list of multiple channels with multiple values."]
fn set_pwm_channels_values_c(channels: *const PwmChannel, values: *const u16, length: usize) {
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

#[cpy_fn_py]
#[comment_py = "Like :py:func:`set_pwm_channel_value`. This function sets the Duty Cycle for a list of\n
    multiple channels with multiple values.\n
Args:\n
    channels ([:py:class:`PwmChannel`]): An array of channels to be selected for PWM.\n
    values ([u16]) : An array Duty cycle value.\n
Examples:\n
    You can use this method like :py:func:`set_pwm_channel_value`.\n
    >>> navigator.set_pwm_channels_values([PwmChannel.Ch1, PwmChannel.Ch5], [1000, 500])"]
fn set_pwm_channels_values_py(channels: Vec<PwmChannel>, values: Vec<u16>) {
    if channels.len() != values.len() {
        println!("The number of values is different from the number of PWM channels.");
        return;
    }

    for i in 0..channels.len() {
        with_navigator!().set_pwm_channel_value(channels[i].clone().into(), values[i]);
    }
}

cpy_module!(
    name = bluerobotics_navigator,
    types = [AdcChannel, UserLed, PwmChannel, AxisData, ADCData],
    functions = [
        init,
        self_test,
        set_led,
        get_led,
        set_led_toggle,
        set_led_all,
        set_neopixel,
        read_adc_all,
        read_adc,
        read_pressure,
        read_temp,
        read_mag,
        read_accel,
        read_gyro,
        pwm_enable,
        set_pwm_freq_prescale,
        set_pwm_freq_hz,
        set_pwm_channel_value,
        set_pwm_channels_value,
        set_pwm_channels_values
    ]
);
