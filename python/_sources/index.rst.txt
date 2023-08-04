.. extension documentation master file, created by
   sphinx-quickstart on Mon Jun 19 15:02:05 2023.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

BlueRobotics's Navigator Library
=====================================

This library serves as the entry point for embedding applications using Python on Blue Robotics's Navigator_.

The Navigator board has the Raspberry Pi HAT form factor, which allows you to easily attach it to a Raspberry Pi 4 board. Then you can unleash the power of Navigator to develop new solutions or interact with your ROV hardware.

The board offers the following capabilities:

Control:

- LEDs

- PWM (Pulse Width Modulation) with 16 channels

Measurements:

- ADC (Analog Digital Converter) entries

- Magnetic field

- Acceleration

- Angular velocity

- Temperature

- Pressure

Currently, it supports **armv7** and **aarch64** architectures, which are the official defaults for BlueOS_.
However, despite using the embedded-hal concept, new ports can be added as long as the platform matches the hardware design and specifications.

For more information, including installation instructions, schematics, and hardware specifications, please check the navigator hardware setup guide_.

.. _Navigator: https://bluerobotics.com/store/comm-control-power/control/navigator/
.. _BlueOS: https://docs.bluerobotics.com/ardusub-zola/software/onboard/BlueOS-1.1/
.. _guide: https://bluerobotics.com/learn/navigator-hardware-setup/#introduction

.. automodule:: bluerobotics_navigator
   :members:
   :undoc-members:
