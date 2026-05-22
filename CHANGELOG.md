# Changelog

All notable changes to the Talu Engine will be documented in this file.

## [Unreleased]

### Added
- **Cargo Workspace**: Restructured the repository into a Cargo workspace with two crates — `talu-engine` (the runtime) and `talu` (the CLI) — sharing a single `target/` directory. Both binaries are built with a single `cargo build --release`.
- **Circular Math Helpers**: Exposed `shape_pos_x(radius, angle)` and `shape_pos_y(radius, angle)` utility functions to WolfLang to easily calculate positions on a circle (using degrees).
- **`talu` CLI**: New command-line tool for managing the full project lifecycle:
  - `talu new <name>` — Scaffolds a new project with `package.talu`, `config.wolf`, `main.wolf`, `assets/`, and `packages/` directories.
  - `talu run [path]` — Validates the project and launches the engine binary.
  - `talu build [path]` — Packages the project for distribution, producing a self-contained `dist/` folder with the engine binary, all scripts, assets, and plugins.
  - `talu new-plugin <name>` — Scaffolds a new Rust `cdylib` plugin crate with the correct `Cargo.toml` and a `register_talu_plugin` entry point template.
  - `talu build-plugin [path]` — Compiles a plugin crate in release mode and automatically installs the resulting `.so`/`.dll`/`.dylib` into the project's `packages/` directory.
- **Plugin Activation**: Uncommented and enabled the `func(engine as *mut WolfEngine)` call in `packages.rs`, so loaded plugins now actually register their functions with the WolfLang engine.
- **Manifest-Based Plugin System**: Added support for loading Rust dynamic libraries via the `plugins` key in `package.talu`. The engine now handles OS-specific prefixes and extensions automatically.
- **Native WolfLang Imports**: Integrated WolfLang's native `import "file.wolf" as alias` feature, deprecating the legacy auto-loading of `.wolf` files from the packages directory.
- **Documentation Improvements**: Updated the user guide and added tutorial documentation for WolfLang usage.

### Changed
- **Asset Loading Logic**: Refactored `load_assets` to remove redundant asset directory path handling and rely on explicit asset paths.
- **Utility Functions**: Consolidated token parsing logic into a central `get_float` helper, removing redundant and logically flawed utility functions like `float_to_int`.
- **Plugin ABI Handling**: Shifted dynamic library registration to use `extern "C"` and raw pointers (`*mut WolfEngine`) to mitigate segmentation faults caused by cross-ABI Rust object sharing.

### Fixed
- **Plugin Resolution**: Fixed a critical bug where plugin filenames were missing the dot before the extension (e.g., `libpluginso` instead of `libplugin.so`).
- **Path Searching**: Improved absolute path resolution so the engine can reliably find plugins both in the project directory and the engine root, regardless of the working directory.
- **Texture Lookup**: Improved texture registration so runtime asset names can be used directly by scripts.


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
