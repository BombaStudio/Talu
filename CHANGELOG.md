# Changelog

All notable changes to the Talu Engine will be documented in this file.


## [Unreleased]

### Added
- **Visual Error Console**: Intercepts WolfLang script panics using `std::panic::catch_unwind` and renders the error message directly to the Raylib window to prevent abrupt application crashes.
- **Input Handling API**: Exposed keyboard input functions to the WolfLang scripting environment.
  - Supported functions: `is_key_down`, `is_key_up`, `is_key_pressed`, `is_key_pressed_repeat`.
  - Supported keys: `W`, `S`, `UP`, `DOWN`, `RIGHT`, `LEFT`.
- **Primitive Drawing API**: Exposed core Raylib 2D drawing functions to the engine.
  - `drawRect`, `drawCircle`, and `drawLine` are now callable from `test.wolf` scripts.
- **Engine Initialization**: Basic window configuration parameters (`screen_size_x`, `screen_size_y`, `title`) can now be set via `config.wolf`.