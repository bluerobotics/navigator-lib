# Navigator Library

[![Actions](https://github.com/bluerobotics/navigator-lib/actions/workflows/action.yml/badge.svg)](https://github.com/bluerobotics/navigator-lib/actions/workflows/action.yml)
![PyPI](https://img.shields.io/pypi/v/bluerobotics_navigator)

This library serves as the entry point for applications that want to use [Navigator](https://bluerobotics.com/store/comm-control-power/control/navigator/) with Python or C++.

> For Rust 🦀, please check the [navigator-rs library](https://github.com/bluerobotics/navigator-rs).


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
