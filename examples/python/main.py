#!/usr/bin/env python

import os
import platform
import bluerobotics_navigator as navigator
from bluerobotics_navigator import AdcChannel, UserLed


def navigator_check():
    print(f"Functions available: {navigator.__all__}")

    if os.environ.get("CI") == "true":
        print("Running in CI")
        print("Not possible to test navigator sensors yet.")
        return

    print("Initializing navigator module.")

    if platform.machine() == "aarch64":
        # It's possible to set the configuration before initializing the navigator, check this example
        from bluerobotics_navigator import NavigatorVersion, Raspberry
        print("Setting up for Navigator V2 on Raspberry Pi 5")
        navigator.set_rgb_led_strip_size(1)
        navigator.set_navigator_version(NavigatorVersion.Version2)
        navigator.set_raspberry_pi_version(Raspberry.Pi5)

    navigator.init()

    print("Setting led on!")
    navigator.set_led(UserLed.Led1, True)

    print(f"Temperature: {navigator.read_temp()}")

    print(f"Pressure: {navigator.read_pressure()}")

    print(f"Leak sensor: {navigator.read_leak()}")

    Data = navigator.read_adc_all()
    print(
        f"Data ADC Channels: 1 = {Data[0]}, 2 = {Data[1]}, 3 = {Data[2]}, 4 = {Data[3]}"
    )

    print(f"Data ADC Channel: 1 = {navigator.read_adc(AdcChannel.Ch1)}")

    Data = navigator.read_mag()
    print(f"Magnetic field: X = {Data.x}, Y = {Data.y}, Z = {Data.z}")

    Data = navigator.read_accel()
    print(f"Acceleration: X = {Data.x}, Y = {Data.y}, Z = {Data.z}")

    Data = navigator.read_gyro()
    print(f"Gyroscope: X = {Data.x}, Y = {Data.y}, Z = {Data.z}")

    print("Setting led off!")
    navigator.set_led(UserLed.Led1, False)


if __name__ == "__main__":
    navigator_check()
