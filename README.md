# trainbuttons

Two boards providing buttons for controlling a train (simulator).

The buttons are provided by keyboard key switches (Cherry MX or compatible). The two boards are to be interconnected via a repurposed HDMI cable; the left board is then connected via USB-C to the computer.

The button layout is inspired by the results of the EUDDplus (European Driver's Desk plus) project and UIC 612. The icons are reconstructed from those in EN 16186-2. The buttons chosen are those supported, or expected to be supported, by the Zusi simulator.

The design does not include throttle or brake levers; potentiometer-based solutions are available from [Stillertec](https://fahrpult.com/).

The default firmware lets the device appear as a HID gamepad device with zero axes and 21 buttons. A debug pin header is provided on the board to allow installation of other firmware.
