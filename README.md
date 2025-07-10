# ESPeyboard: USB keyboard relay

Have you ever needed a USB keyboard and just didn't have one? ESPeyboard
allows you to use your laptop (or another device with a keyboard) as a USB
keyboard by sending keystrokes over Bluetooth LE to an ESP32-S3, which acts
as an HID device and relays the keystrokes over USB.

![Communication diagram](./assets/espeyboard.svg)

There are two components to ESPeyboard: the ESP32 code, and a Rust client that runs on the input device.
