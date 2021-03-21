# Usermode API library for a NI Maschine Mikro MK2

This is a learning/experimentation project for Rust.

Much of the protocl implementation details have been obtained from the cabl project.

The ultimate end goal is to move this project onto amicrocontroller (likely the Raspberry Pi Pico) to use the controller to either output MIDI or act as a CV source or sequencer.

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
- Convert into a crate taht can be easily integrated into other applications (not there is a caveat regarding hidapi that only a single hidapi::Context can exist).

## Implementation details

Uses the hidapi crate to identify and integrate with USB HID interfaces.

!Note! this is a first Rust project so will contain approaches that are non standard. And includes no tests (not quite got there yet!).
- 
