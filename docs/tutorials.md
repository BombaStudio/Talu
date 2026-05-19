# 🧪 Talu Engine Tutorial

This document explains how to create a Talu Engine game project and understand example games step by step.

## 1. Getting Started: Creating a Project Folder

1. Create a new folder.
2. Add `package.talu`, `config.wolf`, and `main.wolf` files.
3. `package.talu` content:

```talu
config = config.wolf
run = main.wolf
# Optional: Load Rust dynamic libraries
# plugins = my_rust_plugin 
```

4. `config.wolf` content:

```wolf
let screen_size_x : int = 800
let screen_size_y : int = 600
let title : string = "Talu Game"
```

5. `main.wolf` content:

```wolf
let x : float = 100.0
let y : float = 100.0
let speed : float = 200.0

fn start()
    print("Welcome to the Talu game!")
end

fn update()
    if is_key_down("D")
        x = x + speed * deltaTime
    end
    if is_key_down("A")
        x = x - speed * deltaTime
    end

    drawRect(x, y, 80.0, 80.0, 0.0, 200.0, 255.0)
end
```

This is a playable starter game. Press `A` and `D` to move the square horizontally.

## 2. Clicker Game with `examples/clicker`

The `examples/clicker/clicker.wolf` example shows clicking with the left mouse button and scoring.

### How it works

- `drawCircle` draws the target.
- `is_mouse_button_pressed(0)` checks left mouse clicks.
- `get_mouse_x()` and `get_mouse_y()` get the cursor position.
- When the target is clicked, `score` increases and the circle moves to a random position.

### Main code sections:

```wolf
let score : int = 0
let circleX : float = 400.0
let circleY : float = 300.0
let circleRadius : float = 50.0

fn update()
    drawCircle(circleX, circleY, circleRadius, 0.0, 150.0, 255.0)

    if is_mouse_button_pressed(0)
        let mx : float = get_mouse_x()
        let my : float = get_mouse_y()

        let dx : float = mx - circleX
        let dy : float = my - circleY
        let distSq : float = dx * dx + dy * dy

        if distSq < circleRadius * circleRadius
            score = score + 1
            print("Score: " + score)
            circleX = random_float(100.0, 700.0)
            circleY = random_float(100.0, 500.0)
        end
    end
end
```

### What you learn

- How to detect mouse clicks.
- How to check if the cursor hits the target.
- Using `random_float` for dynamic positions.

## 3. Platformer Movement with `examples/platformer`

The platformer example demonstrates gravity, jumping, and horizontal movement.

### Core ideas

- Move left and right with `is_key_down("A")` and `is_key_down("D")`.
- Jump with `is_key_pressed("SPACE")`.
- Use `deltaTime` to make speed independent of frame rate.

### Example code segments:

```wolf
let playerX : float = 100.0
let playerY : float = 500.0
let velY : float = 0.0
let isGrounded : bool = false

let gravity : float = 800.0
let jumpForce : float = -400.0
let speed : float = 200.0

fn update()
    if is_key_down("A")
        playerX = playerX - speed * deltaTime
    end
    if is_key_down("D")
        playerX = playerX + speed * deltaTime
    end

    velY = velY + gravity * deltaTime
    playerY = playerY + velY * deltaTime

    if playerY > 500.0
        playerY = 500.0
        velY = 0.0
        isGrounded = true
    end

    if isGrounded
        if is_key_pressed("SPACE")
            velY = jumpForce
            isGrounded = false
        end
    end

    drawRect(playerX, playerY, 40.0, 40.0, 0.0, 255.0, 0.0)
    drawRect(0.0, 540.0, 800.0, 60.0, 100.0, 100.0, 100.0)
end
```

### What you learn

- How to respond only once on jump input.
- A simple ground collision check.
- Using `deltaTime` for smooth movement.

## 4. Simple Physics and Collision with `examples/rigidbody_boxes`

This example simulates two boxes falling and reacting when they collide.

### Key points

- Uses `check_collision(...)` for two rectangles.
- Reverses velocities on collision and separates boxes.
- Applies gravity with `gravity * deltaTime`.

### Main code sections:

```wolf
let b1_px : float = 400.0
let b1_py : float = 100.0
let b1_vy : float = 0.0
let b1_mass : float = 1.0

let b2_px : float = 410.0
let b2_py : float = 300.0
let b2_vy : float = 0.0
let b2_mass : float = 2.0

let gravity : float = 500.0

fn update()
    b1_vy = b1_vy + gravity * deltaTime
    b1_py = b1_py + b1_vy * deltaTime

    b2_vy = b2_vy + gravity * deltaTime
    b2_py = b2_py + b2_vy * deltaTime

    if b1_py > 500.0
        b1_py = 500.0
        b1_vy = b1_vy * -0.5
    end

    if b2_py > 500.0
        b2_py = 500.0
        b2_vy = b2_vy * -0.3
    end

    if check_collision(b1_px, b1_py, 50.0, 50.0, b2_px, b2_py, 80.0, 80.0)
        let temp : float = b1_vy
        b1_vy = b2_vy * 0.5
        b2_vy = temp * 0.5

        if b1_py < b2_py
            b1_py = b2_py - 50.1
        end
        if b1_py > b2_py
            b2_py = b1_py - 80.1
        end
    end

    drawRect(b1_px, b1_py, 50.0, 50.0, 255.0, 0.0, 0.0)
    drawRect(b2_px, b2_py, 80.0, 80.0, 0.0, 0.0, 255.0)
    drawRect(0.0, 550.0, 800.0, 50.0, 150.0, 150.0, 150.0)
end
```

### What you learn

- Basic collision detection and response.
- Velocity changes after collision.
- Updating physics-based objects.

## 5. Building Your Own Game

### Suggested development steps:
1. Use `import` to split complex logic into separate `.wolf` files inside a `packages/` directory.
2. Add more variables in `main.wolf`.
3. Initialize starting state in `start()`.
4. Separate input, physics, and drawing in `update()`.
5. Add visual elements with `drawRect` / `drawCircle`.
6. Use `print` for debugging.

### Example: Small game flow

```wolf
let playerX : float = 200.0
let playerY : float = 400.0
let score : int = 0

fn start()
    print("Game starting...")
end

fn update()
    if is_key_down("W")
        playerY = playerY - 150.0 * deltaTime
    end
    if is_key_down("S")
        playerY = playerY + 150.0 * deltaTime
    end

    drawRect(playerX, playerY, 40.0, 40.0, 0.0, 255.0, 0.0)
end
```

This tutorial covers the basic steps to develop a game with Talu Engine. Explore the ready-made examples in the `examples/` folder to expand your own project.
