# Navigator Library

[![Actions](https://github.com/bluerobotics/navigator-lib/actions/workflows/action.yml/badge.svg)](https://github.com/bluerobotics/navigator-lib/actions/workflows/action.yml)
[![PyPI](https://img.shields.io/pypi/v/bluerobotics_navigator)](https://pypi.org/project/bluerobotics-navigator/)

This library serves as the entry point for applications that want to use [Navigator](https://bluerobotics.com/store/comm-control-power/control/navigator/) with Python or C++.

> 1 . How-to setup the Raspberry Pi computer, please read [Instructions](#ocean-instructions-for-blueoshttpsdiscussblueroboticscomtblueos-official-release12024-recommended).
  2 . For **Rust** 🦀, please check the [navigator-rs library](https://github.com/bluerobotics/navigator-rs).


## Features
- **LEDs (User and RGB) access**
- **PWM (Pulse Width Modulation) control**
- **ADC (Analog Digital Converter) reading**
- **Magnetometer / Accelerometer / Gyroscope sampling**
- **Temperature reading**
- **Pressure estimation**

# 📖 Documentation:
* [Python](https://docs.bluerobotics.com/navigator-lib/python)
* [C++ (WIP)](https://gist.github.com/patrickelectric/133bc706a7397479bfae6f57665bddeb)

Check the examples folder for further [information and guide](https://github.com/bluerobotics/navigator-lib/tree/master/examples).

## 🐍 Python:

Install the library.

```shell
pip install bluerobotics_navigator
```

With that, you'll bee able to run the examples, or creating your own:

```python
#!/usr/bin/env python

import bluerobotics_navigator as navigator

print("Initializing navigator module.")
navigator.init()

print("Setting led on!")
navigator.set_led(navigator.UserLed.Led1, True)

print(f"Temperature: {navigator.read_temp()}")
print(f"Pressure: {navigator.read_pressure()}")

print(
    f"Data ADC Channels: {navigator.read_adc_all().channel}"
)

print(f"Data ADC Channel: 1 = {navigator.read_adc(navigator.AdcChannel.Ch1)}")

data = navigator.read_mag()
print(f"Magnetic field: X = {data.x}, Y = {data.y}, Z = {data.z}")
```

## 🛠️ C++:

Follow our example folder as a template to create your own project. To compile and run the examples, you can run:

```shell
cd examples/cpp
cmake -B build -DCMAKE_BUILD_TYPE=Debug && cmake --build build --config Debug --parallel
# Run one of the examples
./build/simple
./build/rainbow
```

For an example of C++ code, you can check the following code:

```cpp
#include "bindings.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

int main() {
  printf("Initiating navigator module.\n");
  init();

  printf("Setting led on!\n");
  set_led(UserLed::Led1, true);

  printf("Temperature: %f\n", read_temp());
  printf("Pressure: %f\n", read_pressure());

  ADCData adc = read_adc_all();
  printf("Reading ADC Channels: 1 = %f, 2 = %f, 3 = %f, 4 = %f\n",
         adc.channel[0], adc.channel[1], adc.channel[2], adc.channel[3]);
  printf("Data ADC Channels: 1 = %f\n", read_adc(AdcChannel::Ch1));

  AxisData mag = read_mag();
  printf("Magnetic field: X = %f, Y = %f, Z = %f\n", mag.x, mag.y, mag.z);

  return 0;
}

```

> Note: The CMakeLists_Standalone.txt is a self-contained CMake project file example. Users can use it as a template to create their standalone projects based on the navigator-lib.

## 🏗️ Supported Architectures

Currently, the library supports **armv7** and **aarch64** architectures, which are the official defaults for [BlueOS](https://docs.bluerobotics.com/ardusub-zola/software/onboard/BlueOS-1.1/). The library also provides C++ `.so` files for both `gnu` and `musl`.

For more detailed information, including installation instructions, schematics, and hardware specifications, please refer to the [navigator hardware setup guide](https://bluerobotics.com/learn/navigator-hardware-setup/#introduction).

##  :ocean: Instructions for [BlueOS](https://discuss.bluerobotics.com/t/blueos-official-release/12024) (Recommended)

1. Open the [Autopilot Firmware](https://blueos.cloud/docs/blueos/latest/advanced-usage/#autopilot-firmware) page, enable [Pirate Mode](https://blueos.cloud/docs/blueos/latest/advanced-usage/#pirate-mode), and "change board" to SITL
    - This stops the autopilot firmware from trying to operate while the WebAssistant is in use
1. [Reboot](https://blueos.cloud/docs/blueos/latest/advanced-usage/#power) the vehicle computer, and wait for the interface to re-connect
> Note: Since this library access the Navigator hardware, it **can´t** run in parallel with ArduPilot.
If you are running ArduPilot, be sure to disable it or set the board as **SITL** (Software Simulation) before running Navigator WebAssistant

##  :cherries: Instructions for [Raspberry Pi OS](https://www.raspberrypi.com/software/)

1. Download the Raspberry OS image (32 or 64 bits)
> Note: To use a kernel that matches with the BlueOS and avoid incompatilities, please use:
**32** Bits: [raspios_lite_armhf-2023-02-22](https://downloads.raspberrypi.com/raspios_lite_armhf/images/raspios_lite_armhf-2023-02-22/)
**64** Bits: [raspios_lite_arm64-2023-02-22](https://downloads.raspberrypi.com/raspios_lite_arm64/images/raspios_lite_arm64-2023-02-22/)

2. Install the [Raspberry Pi Imager](https://www.raspberrypi.com/software/) and flash the image
> Note: You can also automatically setup the ssh, user, hostname and wi-fi settings.

3. Execute the following command to setup the overlay required for Navigator ( I2C, SPI, GPIOs & others)


```shell
sudo su -c 'curl -fsSL https://raw.githubusercontent.com/bluerobotics/blueos-docker/master/install/boards/configure_board.sh | bash'
sudo reboot
```
