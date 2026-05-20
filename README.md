# 🐺 Talu Engine

<div align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/Raylib-000000?style=for-the-badge&logo=raylib&logoColor=white" alt="Raylib" />
  <img src="https://img.shields.io/badge/WolfLang-5E35B1?style=for-the-badge&logo=wolf&logoColor=white" alt="WolfLang" />
  <br>
  <strong>A high-performance 2D game engine powered by Rust and scripted with WolfLang.</strong>
</div>

---

Talu is a professional-grade 2D game framework that combines the safety and performance of **Rust** with the simplicity of **WolfLang**. Designed for developers who want to build games quickly without sacrificing power.

## ✨ Features

- **🚀 High Performance**: Built on top of Rust and Raylib 5.5 for blazing-fast rendering.
- **📜 Scripting Mastery**: Express your game logic easily with **WolfLang**.
- **⚖️ Built-in Physics**: Integrated physics module with collision detection.
- **🎮 Input System**: Comprehensive keyboard and mouse input handling.
- **🎨 Render Primitives**: Draw rectangles, circles, and lines with ease.
- **🛠️ Developer-First**: Real-time panic-catching UI that prevents crashes and shows debug info.
- **📦 Modular Packages**: Organize your projects with independent `package.talu` and `config.wolf` setups.
- **🔧 `talu` CLI**: Full project lifecycle management — scaffold, run, build for distribution, and manage Rust plugins without touching Cargo directly.
- **🔌 Live Plugin System**: Write engine extensions in Rust (`cdylib`) and load them into any project via `package.talu`. Plugins register custom WolfLang functions at runtime.

## 🚀 Getting Started

### 1. Prerequisites
Ensure you have the [Rust toolchain](https://rustup.rs/) installed on your system.

### 2. Installation
Clone the repository and build both the engine and CLI:

```bash
git clone https://github.com/BombaStudio/Talu.git
cd Talu
cargo build --release
```

This produces two binaries in `target/release/`: `talu-engine` (the runtime) and `talu` (the CLI). Add them to your `PATH` or use them directly from `target/release/`.

### 3. Creating a New Project

```bash
talu new mygame
cd mygame
talu run
```

`talu new` scaffolds a ready-to-run project with `package.talu`, `config.wolf`, `main.wolf`, and empty `assets/` and `packages/` directories.

### 4. Running Examples
Talu comes with several pre-built examples:

```bash
talu run engine/examples/platformer
talu run engine/examples/rigidbody_boxes
talu run engine/examples/clicker
```

### 5. Building for Distribution

```bash
talu build ./mygame
```

Produces a self-contained `mygame/dist/` folder with the engine binary, all scripts, assets, and plugins — ready to zip and share.

## 📖 Usage

A Talu project typically consists of:
1. `package.talu`: Defines the configuration, entry script, and Rust plugins.
2. `config.wolf`: Window and engine initialization parameters.
3. `your_script.wolf`: Your game logic (must define `start()` and `update()`).

### The Manifest (`package.talu`)
```talu
config = config.wolf
run = main.wolf
plugins = my_rust_plugin, another_plugin
```
The `plugins` key automatically resolves and loads platform-specific Rust dynamic libraries (`.so`, `.dll`, `.dylib`) from your `packages/` directory or the engine root.

### Importing Wolf Packages
Talu fully supports WolfLang's native module system.
```wolf
import "packages/my_math.wolf" as math

fn update()
    let res : int = math::add(5, 10)
end
```

Check out the [User Manual](docs/manual.md) for a deep dive into the engine's capabilities.

### Rust Plugins

Plugins let you extend WolfLang with custom functions written in Rust. Scaffold and build one with the CLI:

```bash
talu new-plugin my_plugin
cd my_plugin
# edit src/lib.rs to register your functions
talu build-plugin .
```

Then reference it in your project's `package.talu`:

```talu
config = config.wolf
run = main.wolf
plugins = my_plugin
```

## 🖥️ CLI Reference

| Command | Description |
| :--- | :--- |
| `talu new <name>` | Scaffold a new project |
| `talu run [path]` | Run a project (default: current directory) |
| `talu build [path]` | Package a project into a distributable `dist/` folder |
| `talu new-plugin <name>` | Scaffold a new Rust plugin crate |
| `talu build-plugin [path]` | Compile and install a plugin into `packages/` |

## 📚 Documentation & History

- 📘 **[User Manual](docs/manual.md)**: Detailed API references, project structure, and scripting guide.
- 📘 **[Tutorials](docs/tutorials.md)**: Step-by-step guides to build your first game with Talu.
- 📜 **[Changelog](CHANGELOG.md)**: Track all the latest updates, features, and bug fixes.
- 📂 **[Examples](examples/)**: Explore pre-built projects like the Platformer and Physics simulations.

## 🛠️ Built-in API

| Category | Function | Description |
| :--- | :--- | :--- |
| **Rendering** | `drawRect(x, y, w, h, r, g, b)` | Draws a colored rectangle. |
| | `drawCircle(x, y, rad, r, g, b)` | Draws a colored circle. |
| | `drawLine(sx, sy, ex, ey, r, g, b)` | Draws a colored line. |
| **Physics** | `check_collision(x1, y1, w1, h1, x2, y2, w2, h2)` | Returns `true` if two rects overlap. |
| **Input** | `is_key_down(key)` | Checks if a key is currently held. |
| | `is_key_pressed(key)` | Checks if a key was pressed this frame. |
| **Utility** | `print(msg)` | Prints a message to the console. |
| | `random_float(min, max)` | Generates a random float. |

---

<div align="center">
  <sub>Built with ❤️ by the BombaStudio Team</sub>
</div>

## 📈 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=BombaStudio/Talu&type=Date)](https://star-history.com/#BombaStudio/Talu&Date)
