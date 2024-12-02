use cpy_binder::{cpy_enum, cpy_fn, cpy_fn_c, cpy_fn_py, cpy_module, cpy_struct};

use lazy_static::lazy_static;
use std::sync::Mutex;

struct NavigatorBuilderManager {
    rgb_led_strip_size: usize,
}

lazy_static! {
    static ref NAVIGATORBUILDER: Mutex<NavigatorBuilderManager> =
        Mutex::new(NavigatorBuilderManager {
            rgb_led_strip_size: 1,
        });
}

macro_rules! with_navigator_builder {
    () => {
        NAVIGATORBUILDER.lock().unwrap()
    };
}

#[cpy_fn]
#[comment_c = "Sets the size of the navigator led strip (1 is the default), should be called before `init`."]
#[comment_py = "Sets the size of the navigator led strip (1 is the default), should be called before `init`.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.set_rgb_led_strip_size(1)\n
        >>> navigator.init()"]
fn set_rgb_led_strip_size(size: usize) {
    with_navigator_builder!().rgb_led_strip_size = size;
}

struct NavigatorManager {
    navigator: navigator_rs::Navigator,
}

lazy_static! {
    static ref NAVIGATOR: Mutex<Option<NavigatorManager>> = Mutex::new(None);
}

impl NavigatorManager {
    fn get_instance() -> &'static Mutex<Option<Self>> {
        if NAVIGATOR.lock().unwrap().is_none() {
            *NAVIGATOR.lock().unwrap() = Some(NavigatorManager {
                navigator: navigator_rs::Navigator::create()
                    .with_rgb_led_strip_size(with_navigator_builder!().rgb_led_strip_size)
                    .build_navigator_v1_pi4(),
            });
        }
        &NAVIGATOR
    }
}

macro_rules! with_navigator {
    () => {
        NavigatorManager::get_instance()
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

#[cpy_enum]
#[comment = "Available ADC channels to read from."]
enum AdcChannel {
    Ch0,
    Ch1,
    Ch2,
    Ch3,
}

#[cpy_enum]
#[comment = "Onboard user-controllable LEDs."]
enum UserLed {
    Led1,
    Led2,
    Led3,
}

#[cpy_struct]
#[comment = "Board-oriented direction axes (x is forwards, y is right, z is down)."]
struct AxisData {
    x: f32,
    y: f32,
    z: f32,
}

#[cpy_fn]
#[comment_c = "Initializes the Navigator module with default settings."]
#[comment_py = "Initializes the Navigator module with default settings.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.init()"]
fn init() {
    // Keep to avoid API break
}

#[cpy_fn]
#[comment_c = "Runs some tests on available sensors, then returns the result."]
#[comment_py = "Runs some tests on available sensors, then returns the result.\n
    Returns:\n
        bool: `True` if the sensors are responding as expected.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> sensors_ok = navigator.self_test()"]
fn self_test() -> bool {
    // Keep to avoid API break
    true
}

#[cpy_fn]
#[comment_c = "Sets the state of the selected onboard LED."]
#[comment_py = "Sets the state of the selected onboard LED.\n
    Args:\n
        select (:py:class:`UserLed`):  A pin to be selected.\n
        state (bool): The desired output state. `True` -> ON, `False` -> OFF.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import UserLed\n
        >>> navigator.set_led(UserLed.Led1, True)"]
fn set_led(select: UserLed, state: bool) {
    with_navigator!().set_led(select.into(), state)
}

#[cpy_fn]
#[comment_c = "Gets the selected onboard LED output state."]
#[comment_py = "Gets the selected onboard LED output state.\n\n
    Args:\n
        select (:py:class:`UserLed`):  A pin to be selected.\n
    Returns:\n
        bool: The current state. `True` -> ON, `False` -> OFF.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import UserLed\n
        >>> led1_on = navigator.get_led(UserLed.Led1)"]
fn get_led(select: UserLed) -> bool {
    with_navigator!().get_led(select.into())
}

#[cpy_fn]
#[comment_c = "Toggle the output of the selected LED."]
#[comment_py = "Toggle the output of the selected LED.\n\n
    Args:\n
        select (:py:class:`UserLed`):  A pin to be selected.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import UserLed\n
        >>> navigator.set_led_toggle(UserLed.Led1)"]
fn set_led_toggle(select: UserLed) {
    with_navigator!().set_led_toggle(select.into())
}

#[cpy_fn_c]
#[comment = "Set the color brightnesses of a connected NeoPixel LED array."]
fn set_neopixel_c(rgb_array: *const [u8; 3], length: usize) {
    let array = unsafe {
        assert!(!rgb_array.is_null());
        std::slice::from_raw_parts(rgb_array, length)
    };
    with_navigator!().set_neopixel(array);
}

#[cpy_fn_py]
#[comment = "Set the color brightnesses of a connected NeoPixel LED array.\n
    Args:\n
        state ([[uint8, uint8, uint8], ...]): A 2D array containing RGB values for each LED.\n
            Set the Red, Green, and Blue components independently, with values from 0-255.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.set_neopixel([[100,0,0]])"]
fn set_neopixel_py(rgb_array: Vec<[u8; 3]>) {
    with_navigator!().set_neopixel(&rgb_array)
}

#[cpy_fn_c]
#[comment = "Set the color brightnesses of a connected NeoPixel LED array."]
fn set_neopixel_rgbw_c(rgb_array: *const [u8; 4], length: usize) {
    let array = unsafe {
        assert!(!rgb_array.is_null());
        std::slice::from_raw_parts(rgb_array, length)
    };
    with_navigator!().set_neopixel_rgbw(array);
}

#[cpy_fn_py]
#[comment = "Set the color brightnesses of a connected NeoPixel LED array.\n
    Args:\n
        state ([[uint8, uint8, uint8, uint8], ...]): A 2D array containing RGB values for each LED.\n
            Set the Red, Green, Blue and White components independently, with values from 0-255.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.set_neopixel([[100,0,0,128]])"]
fn set_neopixel_rgbw_py(rgb_array: Vec<[u8; 4]>) {
    with_navigator!().set_neopixel_rgbw(&rgb_array)
}

#[cpy_fn_py]
#[comment_py = "Reads the ADC channel values (from the ADS1115 chip).\n
    Same as :py:func:`read_adc`, but it returns an array with all channel readings.\n
    Returns:\n
        :py:class:`ADCData`: Measurements in [V].\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> adc_measurements = navigator.read_adc_all().channel"]
fn read_adc_all_py() -> Vec<f32> {
    with_navigator!().read_adc_all()
}

#[cpy_fn_c]
#[comment_c = "Reads the ADC channel values (from the ADS1115 chip)."]
fn read_adc_all_c(mut adc_array: *mut f32, length: usize) {
    let mut array = unsafe {
        assert!(!adc_array.is_null());
        std::slice::from_raw_parts_mut::<f32>(adc_array, length)
    };

    let values = with_navigator!().read_adc_all();
    for i in 0..length {
        array[i] = values[i];
    }
}

#[cpy_fn]
#[comment_c = "Reads a specific ADC channel (from the ADS1115 chip)."]
#[comment_py = "Reads a specific ADC channel (from the ADS1115 chip).\n\n
    Args:\n
        select (:py:class:`AdcChannel`):  An ADC channel to read from.\n
    Returns:\n
        float32: Measurement in [V].\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import AdcChannel\n
        >>> adc1_measurement = navigator.read_adc(AdcChannel.Ch1)"]
fn read_adc(channel: AdcChannel) -> f32 {
    with_navigator!().read_adc(channel.into())
}

#[cpy_fn]
#[comment_c = "Reads the current pressure (from the onboard BMP280 chip)."]
#[comment_py = "Reads the current pressure (from the onboard BMP280 chip).\n
    Returns:\n
        float32: Measurement in [kPa]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> air_pressure = navigator.read_pressure()"]
fn read_pressure() -> f32 {
    with_navigator!().read_pressure()
}

#[cpy_fn]
#[comment_c = "Reads the current temperature (from the onboard BMP280 chip)."]
#[comment_py = "Reads the current temperature (from the onboard BMP280 chip).\n
    Returns:\n
        float32: Measurement in [˚C]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> air_temperature = navigator.read_temperature()"]
fn read_temp() -> f32 {
    with_navigator!().read_temperature()
}

#[cpy_fn]
#[comment_c = "Reads the local magnetic field strengths (from the onboard Ak09915 magnetometer)."]
#[comment_py = "Reads the local magnetic field strengths (from the onboard Ak09915 magnetometer).\n
    Returns:\n
        :py:class:`AxisData`: Measurements in [µT]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> mag_field = navigator.read_mag()"]
fn read_mag() -> AxisData {
    with_navigator!().read_mag().into()
}

#[cpy_fn]
#[comment_c = "Reads the current acceleration values (from the ICM20689 chip's accelerometer)."]
#[comment_py = "Reads the current acceleration values (from the ICM20689 chip's accelerometer).\n
    Returns:\n
        :py:class:`AxisData`: Measurements in [m/s²]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> acceleration = navigator.read_accel()\n
        >>> forward_acc = acceleration.x"]
fn read_accel() -> AxisData {
    with_navigator!().read_accel().into()
}

#[cpy_fn]
#[comment_c = "Reads the current angular velocity (from the ICM20689 chip's gyroscope)."]
#[comment_py = "Reads the current angular velocity (from the ICM20689 chip's gyroscope).\n
    Returns:\n
        :py:class:`AxisData`: Measurements in [rad/s]\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> angular_velocity = navigator.read_gyro()\n
        >>> roll_rate = angular_velocity.x\n
        >>> pitch_rate = angular_velocity.y\n
        >>> yaw_rate = angular_velocity.z"]
fn read_gyro() -> AxisData {
    with_navigator!().read_gyro().into()
}

#[cpy_fn]
#[comment_c = "Reads the state of leak detector pin from Navigator."]
#[comment_py = "Reads the state of leak detector pin from Navigator.\n\n
    Returns:\n
        bool: The current state. `True` -> Leak detection, `False` -> No leak.\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> leak_detector = navigator.read_leak()"]
fn read_leak() -> bool {
    with_navigator!().read_leak()
}

#[cpy_fn]
#[comment_c = "Enables or disables the PWM chip (PCA9685), using the firmware and OE_pin."]
#[comment_py = "Enables or disables the PWM chip (PCA9685), using the firmware and OE_pin.\n
    Args:\n
        state (bool): The desired PWM chip state. `True` -> ON, `False` -> OFF.\n
    Examples:\n
        Please check :py:func:`set_pwm_channel_value`\n
        >>> navigator.set_pwm_enable(True)"]
fn set_pwm_enable(state: bool) {
    with_navigator!().set_pwm_enable(state)
}

#[cpy_fn]
#[comment_c = "Sets the PWM frequency of the PCA9685 chip. All channels use the same frequency."]
#[comment_py = "Sets the PWM frequency of the PCA9685 chip. All channels use the same frequency.\n
    This is a convenience wrapper around :py:func:`set_pwm_freq_prescale`, which chooses the closest
    possible pre-scaler to achieve the desired frequency.\n
    Notes:\n
        Servo motors generally work best with PWM frequencies between 50-200 Hz.\n
        LEDs flicker less in video streams when driven at a frequency multiple of the camera's
        framerate (e.g. a 30fps camera stream should have LEDs at 30/60/90/120/... Hz).\n
    Args:\n
        freq (float32) : The desired PWM frequency (24..1526) [Hz].\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> navigator.set_pwm_freq_hz(60)\n
        >>> navigator.set_pwm_channel_value(1, 2000)\n
        >>> navigator.set_pwm_enable(True)"]
fn set_pwm_freq_hz(freq: f32) {
    with_navigator!().set_pwm_frequency(freq)
}

#[cpy_fn]
#[comment_c = "Sets the duty cycle (the proportion of ON time) for the selected PWM channel."]
#[comment_py = "Sets the duty cycle (the proportion of ON time) for the selected PWM channel.\n
    This sets the PWM channel's OFF counter, with the ON counter hard-coded to 0.\n
    The output turns ON at the start of each cycle, then turns OFF after the specified count
    (value), where each full cycle (defined by :py:func:`set_pwm_freq_hz`) is split into 4096
    segments.\n
    Notes:\n
        A duty cycle of 20% is achieved using a count of 819.\n
        To achieve a specific pulse-duration you need to consider the cycle frequency:\n
        `value = 4095 * pulse_duration / cycle_period`\n
        As an example, if the frequency is set to 50 Hz (20 ms period) then a 1100 µs
        pulse-duration can be achieved with a 5.5% duty cycle, which requires a count of 225.
        Similarly, 1900 µs pulses would be achieved with a count of 389.
    Args:\n
        channel (:py:class:`PwmChannel`): The channel to be selected for PWM.\n
        value (u16) : Duty cycle count value (0..4095).\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import PwmChannel\n
        >>> navigator.init()\n
        >>> navigator.set_pwm_freq_hz(1000)\n
        >>> navigator.set_pwm_channel_value(PwmChannel.Ch1, 2000)\n
        >>> navigator.set_pwm_enable(True)"]
fn set_pwm_channel_value(channel: usize, value: f32) {
    with_navigator!().set_pwm_duty_cycle(channel, value)
}

#[cpy_fn]
#[comment_c = "Sets the duty cycle (the proportion of ON time) for the selected PWM channel."]
#[comment_py = "Sets the duty cycle (the proportion of ON time) for the selected PWM channel.\n
    Similar to :py:func:`set_pwm_channel_value`, this function calculate the OFF counter
    value to match desired PWM channel's duty_cyle.\n
    Notes:\n
        A duty cycle of 1.0 or 0.0 acts like a relay.\n
        Details of counters on IC, check :py:func:`set_pwm_channel_value`.
    Args:\n
        channel (:py:class:`PwmChannel`): The channel to be selected for PWM.\n
        duty_cycle (f32) : Duty cycle count value (0.0 : 1.0).\n
    Examples:\n
        >>> import bluerobotics_navigator as navigator\n
        >>> from bluerobotics_navigator import PwmChannel\n
        >>> navigator.init()\n
        >>> navigator.set_pwm_freq_hz(1000)\n
        >>> navigator.set_pwm_channel_duty_cycle(PwmChannel.Ch1, 0.5)\n
        >>> navigator.set_pwm_enable(True)"]
fn set_pwm_channel_duty_cycle(channel: usize, duty_cycle: f32) {
    with_navigator!().set_pwm_duty_cycle(channel, duty_cycle)
}

#[cpy_fn_c]
#[comment = "Sets the duty cycle (based on OFF counter from 0 to 1) for a list of multiple PWM channels."]
fn set_pwm_channels_value_c(channels: *const usize, value: f32, length: usize) {
    let array_channels = unsafe {
        assert!(!channels.is_null());
        std::slice::from_raw_parts(channels, length)
    };
    for channel in array_channels.iter().take(length) {
        with_navigator!().set_pwm_duty_cycle(channel.clone(), value);
    }
}

#[cpy_fn_c]
#[comment = "Sets the duty cycle (from 0.0 to 1.0) for a list of multiple PWM channels."]
fn set_pwm_channels_duty_cycle_c(channels: *const usize, duty_cycle: f32, length: usize) {
    let array_channels = unsafe {
        assert!(!channels.is_null());
        std::slice::from_raw_parts(channels, length)
    };
    for channel in array_channels.iter().take(length) {
        with_navigator!().set_pwm_duty_cycle(channel.clone().into(), duty_cycle);
    }
}

#[cpy_fn_py]
#[comment = "Like :py:func:`set_pwm_channel_value`. This function sets the duty cycle for a list of multiple PWM channels.\n
    Args:\n
        channels ([:py:class:`PwmChannel`]): A list of PWM channels to configure.\n
        value (u16) : The desired duty cycle value (0..4095).\n
    Examples:\n
        You can use this method like :py:func:`set_pwm_channel_value`.\n
        >>> navigator.set_pwm_channels_value([PwmChannel.Ch1, PwmChannel.Ch16], 1000)"]
fn set_pwm_channels_value_py(channels: Vec<PwmChannel>, value: u16) {
    for i in 0..channels.len() {
        with_navigator!().set_pwm_channel_value(channels[i].clone().into(), value);
    }
}

#[cpy_fn_py]
#[comment = "Like :py:func:`set_pwm_channel_duty_cycle`. This function sets the duty cycle for a list of multiple PWM channels.\n
    Args:\n
        channels ([:py:class:`PwmChannel`]): A list of PWM channels to configure.\n
        duty_cycle (f32) : Duty cycle count value (0.0 : 1.0).\n
    Examples:\n
        You can use this method like :py:func:`set_pwm_channel_duty_cycle`.\n
        >>> navigator.set_pwm_channels_value([PwmChannel.Ch1, PwmChannel.Ch16], 0.5)"]
fn set_pwm_channels_duty_cycle_py(channels: Vec<PwmChannel>, duty_cycle: f32) {
    for channel in channels {
        with_navigator!().set_pwm_duty_cycle(channel.into(), duty_cycle);
    }
}

#[cpy_fn_c]
#[comment = "Sets the duty cycle (from 0 to 4096) for a list of multiple channels with multiple values."]
fn set_pwm_channels_values_c(channels: *const usize, values: *const f32, length: usize) {
    let array_channels = unsafe {
        assert!(!channels.is_null());
        std::slice::from_raw_parts(channels, length)
    };
    let array_values = unsafe {
        assert!(!values.is_null());
        std::slice::from_raw_parts(values, length)
    };
    for i in 0..length {
        with_navigator!().set_pwm_duty_cycle(array_channels[i].clone(), array_values[i]);
    }
}

#[cpy_fn_c]
#[comment = "Sets the duty cycle (from 0.0 to 1.0) for a list of multiple channels with multiple values."]
fn set_pwm_channels_duty_cycle_values_c(
    channels: *const usize,
    duty_cycle: *const f32,
    length: usize,
) {
    let array_channels = unsafe {
        assert!(!channels.is_null());
        std::slice::from_raw_parts(channels, length)
    };
    let array_values = unsafe {
        assert!(!duty_cycle.is_null());
        std::slice::from_raw_parts(duty_cycle, length)
    };
    for i in 0..length {
        with_navigator!()
            .set_pwm_duty_cycle(array_channels[i].clone().into(), array_values[i]);
    }
}

#[cpy_fn_py]
#[comment = "Like :py:func:`set_pwm_channel_value`. This function sets the duty cycle for a list of
    multiple channels with multiple values.\n
    Args:\n
        channels ([:py:class:`PwmChannel`]): A list of PWM channels to configure.\n
        values ([u16]) : A corresponding list of duty cycle values.\n
    Examples:\n
        You can use this method like :py:func:`set_pwm_channel_value`.\n
        >>> navigator.set_pwm_channels_values([PwmChannel.Ch1, PwmChannel.Ch5], [1000, 500])"]
fn set_pwm_channels_values_py(channels: Vec<PwmChannel>, values: Vec<u16>) {
    if channels.len() != values.len() {
        println!("The number of values is different from the number of PWM channels.");
        return;
    }

    for i in 0..channels.len() {
        with_navigator!().set_pwm_duty_cycle(channels[i].clone().into(), values[i]);
    }
}

#[cpy_fn_py]
#[comment = "Like :py:func:`set_pwm_channel_duty_cycle`. This function sets the duty cycle for a list of
    multiple channels with multiple values.\n
    Args:\n
        channels ([:py:class:`PwmChannel`]): A list of PWM channels to configure.\n
        duty_cycle_values (f32) : Duty cycle count value (0.0 : 1.0).\n
    Examples:\n
        You can use this method like :py:func:`set_pwm_channel_duty_cycle`.\n
        >>> navigator.set_pwm_channels_duty_cycle_values([PwmChannel.Ch1, PwmChannel.Ch5], [0.25, 0.75])"]
fn set_pwm_channels_duty_cycle_values_py(channels: Vec<PwmChannel>, duty_cycle_values: Vec<f32>) {
    if channels.len() != duty_cycle_values.len() {
        println!("The number of values is different from the number of PWM channels.");
        return;
    }

    for i in 0..channels.len() {
        with_navigator!()
            .set_pwm_duty_cycle(channels[i].clone().into(), duty_cycle_values[i]);
    }
}
cpy_module!(
    name = bluerobotics_navigator,
    types = [AdcChannel, UserLed, PwmChannel, AxisData],
    functions = [
        init,
        self_test,
        set_led,
        get_led,
        set_led_toggle,
        set_led_all,
        set_neopixel,
        set_neopixel_rgbw,
        read_adc_all,
        read_adc,
        read_pressure,
        read_temp,
        read_leak,
        read_mag,
        read_accel,
        read_gyro,
        set_pwm_enable,
        get_pwm_enable,
        set_pwm_freq_prescale,
        set_pwm_freq_hz,
        set_pwm_channel_value,
        set_pwm_channel_duty_cycle,
        set_pwm_channels_value,
        set_pwm_channels_duty_cycle,
        set_pwm_channels_values,
        set_pwm_channels_duty_cycle_values
    ]
);
