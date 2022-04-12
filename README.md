# RustLumibaer

This is a small rewrite of JB-LED library without the multiprocessing part in Rust.
It's supposed to be used for:

  1. Writing and testing LED-Animations on your desktop using the `use_window` parameter
  2. Executing it on a RaspberryPi to control a LED-Strip and control it via MQTT

## How to write animations

Just create a struct that implements the `Animation` trait and use the methods `update`, `initialize` and `on_message` to create functionality.
For help either have a look at the exisiting animations or open an Issue.
