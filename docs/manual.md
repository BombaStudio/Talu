# 📘 Talu Engine User Guide

Talu Engine is a game engine designed to run small games written in the Wolf scripting language. This guide explains the project structure, basic Wolf syntax, and the engine API step by step.

## 📁 Project Structure

A Talu game folder must contain at least these three files:

### 1. `package.talu`
The game manifest. It specifies which files are used for config, run, and defines which Rust plugins to load.

```talu
config = config.wolf
run = main.wolf
plugins = local_rust, rust_test
```
*Note: The `plugins` key is optional and is used to load Rust dynamic libraries.*

### 2. `config.wolf`
Contains engine startup settings. This file is executed first in `main.rs`.

```wolf
let screen_size_x : int = 800
let screen_size_y : int = 600
let title : string = "My Awesome Game"
```

Supported config variables:
- `screen_size_x`: Screen width.
- `screen_size_y`: Screen height.
- `title`: Window title.

### 3. `main.wolf`
The main script containing game logic. It should include at least two functions:
- `fn start()`: Runs once when the game starts.
- `fn update()`: Runs once every frame.

## 📦 Package Management

Talu supports extending its capabilities through two types of packages:

### Wolf Packages
You can import other Wolf language scripts using the native `import` syntax. This helps keep your codebase modular.

```wolf
import "packages/math.wolf" as math

fn start()
    let result : int = math::add(5, 10)
    print("Result: " + result)
end
```
*(Note: As WolfLang is under active development, the syntax and stability of module member access may vary).*

### Rust Plugins
High-performance or system-level features can be written in Rust. Compile them as dynamic libraries (`.so`, `.dll`, `.dylib`) and place them in the `packages/` directory.

To load a Rust plugin, add its name to `package.talu` (without the `lib` prefix or extension):
```talu
plugins = my_rust_plugin
```
The engine will automatically resolve the correct filename for the current operating system, load it via `libloading`, and call its `register_talu_plugin` function.

## 🧩 Basic Wolf Syntax

### Variable Declaration

In Wolf, variables are declared with the `let` keyword. Type annotations are required.

```wolf
let x : float = 100.0
let y : int = 5
let isAlive : bool = true
let name : string = "Player"
```

### Function Declaration

Every function starts with `fn` and ends with `end`.

```wolf
fn start()
    print("Game started")
end

fn update()
    # Put per-frame code here
end
```

### Conditions

Using `if`:

```wolf
if is_key_down("A")
    x = x - 1.0
end
```

`else` is also supported:

```wolf
if is_key_down("A")
    x = x - 1.0
else
    x = x + 1.0
end
```

### Comments

Wolf uses `#` for comments.

```wolf
# This is a comment
```

### String Concatenation

Strings can be concatenated with the `+` operator.

```wolf
let score : int = 10
print("Score: " + score)
```

## 🚀 Startup and Execution Flow

In `main.rs`:
- `config.wolf` is executed first,
- then `main.wolf` is loaded,
- `start()` is called,
- `update()` is called every frame.

If a panic occurs inside the script, the engine shows a red error bar and stops calling `update()`.

## 🎨 Rendering API

Talu provides four drawing functions:

- `drawRect(x, y, width, height, r, g, b)`
- `drawCircle(x, y, radius, r, g, b)`
- `drawLine(x1, y1, x2, y2, r, g, b)`
- `drawTexture(texture_id, x, y, r, g, b)`

Color values range from 0.0 to 255.0.

### Example: Draw a Rectangle

```wolf
fn update()
    drawRect(100.0, 150.0, 120.0, 60.0, 255.0, 0.0, 0.0)
end
```

### Example: Draw a Circle

```wolf
fn update()
    drawCircle(400.0, 300.0, 50.0, 0.0, 150.0, 255.0)
end
```

### Example: Draw a Line

```wolf
fn update()
    drawLine(100.0, 100.0, 700.0, 100.0, 255.0, 255.0, 0.0)
end
```

### Example: Draw a Texture

PNG files in the `assets/` folder are automatically loaded and can be drawn with `drawTexture`.

```wolf
# texture_id corresponds to the order of files in the assets folder.
fn update()
    drawTexture(1.0, 200.0, 200.0, 255.0, 255.0, 255.0)
end
```

> Note: `texture_id` must be an integer, but in Wolf it is passed as a `float`.

## 🕹️ Input API

### Keyboard Controls

Key names are passed as uppercase strings.

- `is_key_down(key: string) -> bool`
- `is_key_pressed(key: string) -> bool`
- `is_key_pressed_repeat(key: string) -> bool`
- `is_key_up(key: string) -> bool`

```wolf
if is_key_down("A")
    playerX = playerX - speed * deltaTime
end

if is_key_pressed("SPACE")
    jump = true
end
```

`is_key_down` returns true while the key is held.
`is_key_pressed` returns true only once when the key is pressed.
`is_key_pressed_repeat` returns true on repeated key presses.
`is_key_up` checks if the key was released.

### Mouse Controls

- `is_mouse_button_pressed(button: int) -> bool`
  - `0`: left click
  - `1`: right click
- `get_mouse_x() -> float`
- `get_mouse_y() -> float`

```wolf
if is_mouse_button_pressed(0)
    let mx : float = get_mouse_x()
    let my : float = get_mouse_y()
end
```

## ⚖️ Physics and Collision

- `check_collision(x1, y1, w1, h1, x2, y2, w2, h2) -> bool`

This function checks whether two rectangles are colliding.

```wolf
if check_collision(x1, y1, w1, h1, x2, y2, w2, h2)
    print("Collision detected")
end
```

## 🛠️ Utility Functions

- `print(message)`: Prints a message to the console.
- `random_float(min, max) -> float`: Returns a random floating-point number in the given range.
- `deltaTime`: Time passed since the last frame, in seconds.

### Example: Random Position

```wolf
let x : float = random_float(100.0, 700.0)
let y : float = random_float(50.0, 550.0)
```

### Example: Move with `deltaTime`

```wolf
let speed : float = 200.0
let x : float = 100.0

fn update()
    if is_key_down("D")
        x = x + speed * deltaTime
    end
end
```

Using `deltaTime` reduces frame rate dependency in motion.

## 🧠 Debugging

If a script error occurs in `main.rs`, the engine shows a red error bar with the message. Once an error occurs, `update()` is not called again until the script is fixed.

## 📁 Asset Loading (`assets` folder)

The `src/engine/asset_pipeline.rs` file automatically loads `.png` files from the `assets/` folder. Files are numbered alphabetically:

- First PNG file is `texture_id = 1`
- Second PNG file is `texture_id = 2`

You can draw them with `drawTexture`.

## 🛠️ Running the Game

From the project root, use:

```bash
cargo run examples/your_folder
```

Examples:
- `cargo run examples/clicker`
- `cargo run examples/platformer`
- `cargo run examples/rigidbody_boxes`
