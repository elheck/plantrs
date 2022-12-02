# plantrs

## Scope
- Raspi pi pico 
- inspired by https://github.com/rp-rs/rp2040-project-template

## General
Flashing Picoprobe Firmware

In order to use one Pico to flash and debug another one, you must first flash firmware created by the Raspberry Pi Foundation onto one Pico of your choice.

- To do that, first download this [UF2](https://datasheets.raspberrypi.org/soft/picoprobe.uf2) binary onto your development machine
- Then, holding down the BOOTSEL (short for boot select) button on the Pico that will become the Picoprobe, plug in the USB cable into your development machine
- A window of the flash storage of the Pico should pop up no matter what operating system you’re running on your development machine
- Drag and drop the picoprobe.uf2 onto this window
- The Pico should now be fully flashed and ready to be used as a Picoprobe