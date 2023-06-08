#!/usr/bin/env python

import os
import navigator
from navigator import adc_Channel


def navigator_check():
    print(f"Fucntions available: {navigator.__all__}")

    if os.environ.get("CI") == "true":
        print("Running in CI")
        print("Not possible to test navigator sensors yet.")
        return

    print("Initializing navigator module.")
    navigator.init()

    print("Setting leds on!")
    navigator.set_led_on()

    print(f"Temperature: {navigator.read_temp()}")

    print(f"Pressure: {navigator.read_pressure()}")

    Data = navigator.read_adc_all()
    print(
        f"Data ADC Channels: 1 = {Data.channel[0]}, 2 = {Data.channel[1]}, 3 = {Data.channel[2]}, 4 = {Data.channel[3]}"
    )

    print(
        f"Data ADC Channel: 1 = {navigator.read_adc(navigator.adc_Channel.Ch1)}"
    )

    Data = navigator.read_mag()
    print(f"Magnetic field: X = {Data.x}, Y = {Data.y}, Z = {Data.z}")

    Data = navigator.read_accel()
    print(f"Acceleration: X = {Data.x}, Y = {Data.y}, Z = {Data.z}")

    Data = navigator.read_gyro()
    print(f"Gyroscope: X = {Data.x}, Y = {Data.y}, Z = {Data.z}")

    print("Setting leds off!")
    navigator.set_led_off()


if __name__ == "__main__":
    navigator_check()
