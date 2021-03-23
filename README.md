# Usermode API library for a NI Maschine Mikro MK2

This is a learning/experimentation project for Rust.

Much of the protocol implementation details have been gleaned from the 
[cabl](https://github.com/shaduzlabs/cabl) project.

The ultimate end goal is to move this project onto a microcontroller (likely the 
Raspberry Pi Pico) to use the controller to either output MIDI or act as a CV source 
or sequencer.

> **Note** this is a first Rust project so will contain approaches that are non 
> standard. And includes no tests (not quite got there yet!).

## Current status

Completed:

- Controlling all LEDS
- All buttons/encoders working
- All pads working

ToDo:

- Display
- Translating button ID's into enums
- Constants for magic numbers
- Better error handling
- Convert into a crate taht can be easily integrated into other applications (not 
  there is a caveat regarding hidapi that only a single hidapi::Context can exist).

## Implementation details

Uses the hidapi crate to identify and integrate with USB HID interfaces.

## Development

This code was developed on a Debian Linux based OS, instructions below may work on
other distributions.

There are two approaches to getting a debugger running you can either run an 
*lldb-server* as root (which is risky) or the better option of configuring *udev* 
to give read/write access to the USB device.

* As root (or sudo) create the file `/etc/udev/rules.d/99-native-instruments.rules`
  with the following entry for all Native Instruments devices:
  ```bash
  SUBSYSTEM=="usb", ATTRS{idVendor}=="17cc", MODE="0666"
  ```
  or to limit to just a Maschine Mikro Mk2 use: 
  ```bash
  SUBSYSTEM=="usb", ATTRS{idVendor}=="17cc", ATTTRS{idProduct}=="1200", MODE="0666"
  ```

* Reload the UDev config: 
  ```bash
  sudo udevadm control --reload-rules
  ```

* Reconnect the USB device.

You should now be able to access the device (and run/debug) as a normal user.
