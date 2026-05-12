# 📘 Talu Engine User Manual

Welcome to the Talu Engine documentation. This guide will help you understand how to use the engine's scripting API and project structure.

## 📁 Project Structure

A Talu game directory must contain at least these three files:

### 1. `package.talu`
The manifest file for your game.
```talu
config = config.wolf
run = main.wolf
```

### 2. `config.wolf`
Initializes the engine state.
```wolf
let screen_size_x : int = 800
let screen_size_y : int = 600
let title : string = "My Awesome Game"
```

### 3. Entry Script (`main.wolf`)
Contains your game logic. It must define two functions:
- `fn start()`: Runs once at startup.
- `fn update()`: Runs every frame.

---

## 📜 WolfLang Scripting API

### 🎨 Rendering
Colors use RGB values from `0.0` to `255.0`.

- `drawRect(x, y, width, height, r, g, b)`
- `drawCircle(x, y, radius, r, g, b)`
- `drawLine(x1, y1, x2, y2, r, g, b)`

### 🕹️ Input Handling
Key names must be **uppercase** (e.g., `"SPACE"`, `"W"`, `"LEFT"`).

- `is_key_down(key: string) -> bool`: Returns true if key is held.
- `is_key_pressed(key: string) -> bool`: Returns true if key was just pressed.
- `is_mouse_button_pressed(button: int) -> bool`: `0` for left click, `1` for right click.
- `get_mouse_x() -> float`
- `get_mouse_y() -> float`

### ⚖️ Physics
- `check_collision(x1, y1, w1, h1, x2, y2, w2, h2) -> bool`: Checks collision between two rectangles.

### 🛠️ Utilities
- `print(message)`: Log to the terminal.
- `random_float(min, max) -> float`: Returns a random float in range.
- `deltaTime`: A global variable containing the time passed since the last frame (in seconds).

---

## 💡 Tips & Best Practices

1. **Comments**: Use `#` for comments. `//` is not supported.
2. **Types**: Be explicit with types when declaring variables (e.g., `let x : float = 0.0`).
3. **Debugging**: If your script crashes, look at the red console on the screen. It will tell you the exact line and error.
4. **Examples**: Check the `examples/` directory for full implementations:
   - `examples/platformer`: Basic movement and jumping.
   - `examples/rigidbody_boxes`: Physics and collision response.
   - `examples/clicker`: Mouse interaction and score management.

---

## 🛠️ Building & Running
From the engine root:
```bash
cargo run examples/your_folder
```
