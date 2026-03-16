# Talu Engine

The first 2D game framework powered by **WolfLang** and **Rust** (via Raylib).

## Installation

### 1. Get the source on GitHub

```bash
git clone https://github.com/BombaStudio/Talu.git
cd Talu

```

### 2. Installing Dependencies

Because Talu is built with Rust and manages its dependencies via Cargo, you don't need to manually install external libraries. Just ensure you have the Rust toolchain installed.

```bash
cargo build

```

### 3. Defining Package File

Create a file that named `package.talu` for defining config and game scripts.

```
config = config.wolf
run = test.wolf
```

### 4. Setting Up Configs

Talu uses a `config.wolf` file in the root directory to initialize the window parameters before the engine boots. Create or edit it like so:

```text
let screen_size_x : int = 800
let screen_size_y : int = 600
let title : string = "WOLF"

```

### 5. Coding the Game

Your game logic is written in WolfLang inside the `test.wolf` file. The engine looks for specific lifecycle functions to execute:

* `start()`: Called once when the engine initializes.
* `update()`: Called every frame during the game loop.

Here is an example `test.wolf` script demonstrating rendering and input:

```text
fn start()
    print "Game Initialized"
end

fn update()
    // Render a static line
    drawLine(50, 200, 500, 200, 0.0, 0.0, 255.0)
    
    // Input handling and drawing shapes
    if is_key_down("W")
        drawRect(50, 50, 100, 100, 255.0, 0.0, 0.0)
    end

    if is_key_down("S")
        drawCircle(300, 100, 40.0, 0.0, 255.0, 0.0)
    end
end

```

### 6. Run It

To compile the engine and launch your game, simply run:

```bash
cargo run

```

---

## Built-in Engine API

### Rendering

Colors are passed as RGB floats `(0.0 to 255.0)`.

* `drawRect(x: int, y: int, width: int, height: int, r: float, g: float, b: float)`
* `drawCircle(x: int, y: int, radius: float, r: float, g: float, b: float)`
* `drawLine(start_x: int, start_y: int, end_x: int, end_y: int, r: float, g: float, b: float)`

### Input

Input functions take a string representation of the key. Supported keys include: `"W"`, `"S"`, `"UP"`, `"DOWN"`, `"RIGHT"`, `"LEFT"`.

* `is_key_down(key: string) -> bool`
* `is_key_up(key: string) -> bool`
* `is_key_pressed(key: string) -> bool`
* `is_key_pressed_repeat(key: string) -> bool`

### Error Handling

Talu features an active panic-catching UI. If your WolfLang script encounters a runtime error, the engine intercepts the crash and renders a red developer console directly on the game screen instead of abruptly closing the window.
