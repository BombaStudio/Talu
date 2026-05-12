# Changelog

All notable changes to the Talu Engine will be documented in this file.

## [v0.1.0] - 2026-05-12

### Added
- **Physics Engine Module**: New native physics system with AABB collision detection.
- **`check_collision` function**: Exposed collision detection to WolfLang scripts for complex interaction logic.
- **Modular Example Structure**: Examples are now organized into subdirectories (`clicker`, `platformer`, `rigidbody_boxes`) each with its own `package.talu` and `config.wolf`.
- **Advanced Arg Parsing**: Added `get_float` utility to handle both `Float` and `Integer` tokens in drawing and physics functions, making scripts more resilient.

### Changed
- **New Drawing Architecture**: Refactored `main.rs` and `draw.rs` to use a `draw_list` (Arc<Mutex<Vec<Shapes>>>) to decouple logic from rendering, allowing for future optimizations and thread-safe drawing.
- **Improved Script Error Handling**: Replaced silent failures with explicit `expect` messages during script loading and improved the visual panic console to provide clearer error details.
- **Script Syntax Standardization**: Switched from `//` to `#` for comments to ensure compatibility with the WolfLang parser.

### Fixed
- **Platformer Jump Fix**: Fixed a case-sensitivity issue in key names ("Space" vs "SPACE") that prevented jumping in the platformer example.
- **Divergent Branch Conflicts**: Resolved significant merge conflicts in core engine files after a remote architecture update.

## [0.0.1] - Initial Release

### Added
- **Visual Error Console**: Intercepts WolfLang script panics and renders error messages directly to the window.
- **Input Handling API**: Support for `is_key_down`, `is_key_up`, `is_key_pressed`, etc.
- **Primitive Drawing API**: Support for `drawRect`, `drawCircle`, and `drawLine`.
- **Engine Initialization**: Basic window configuration via `config.wolf`.