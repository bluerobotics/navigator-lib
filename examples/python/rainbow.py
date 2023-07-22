#!/usr/bin/env python

import navigator
import time
import math


def color_from_sine(percentage):
    pi = math.pi
    red = (math.sin(percentage * 2.0 * pi) * 0.5) + 0.5
    green = (math.sin((percentage + 0.33) * 2.0 * pi) * 0.5) + 0.5
    blue = (math.sin((percentage + 0.67) * 2.0 * pi) * 0.5) + 0.5
    return [
        int(red * 255),
        int(green * 255),
        int(blue * 255),
    ]


def main():
    navigator.init()

    print("Creating rainbow effect!")
    while True:
        steps = 1000
        for i in range(steps):
            ratio = i / steps
            navigator.set_neopixel([color_from_sine(ratio)])
            time.sleep(0.01)


if __name__ == "__main__":
    main()
