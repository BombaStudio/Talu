use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use wolflang::WolfEngine;
use wolflang::tokens::Token;
use raylib::prelude::*;
use raylib::color::Color;

mod engine;
mod setup;
use engine::utils::utils::*;

use engine::draw::{draw_register, Shapes};
use engine::input::input as input_register;
use crate::engine::asset_pipeline::load_assets;

use setup::cli;

fn main() {
    let (config_path, run_path) = cli().expect("Setup failed. Check package.talu and directory.");
    let draw_list: Arc<Mutex<Vec<Shapes>>> = Arc::new(Mutex::new(vec![]));
    let mut engine : WolfEngine = WolfEngine::new();

    let game_config = std::fs::read_to_string(&config_path).map_err(|_| "Config read failed").expect("Config read failed");
    engine.run(&game_config).expect("Config script failed");

    let screen_size_x = opt_to_i32(engine.get_int("screen_size_x"));
    let screen_size_y = opt_to_i32(engine.get_int("screen_size_y"));

    let t : Option<String> = engine.get_str("title");
    let title : &str = get_str_slice(&t);

    let (mut rl, thread) = raylib::init()
        .size(screen_size_x, screen_size_y)
        .title(title)
        .build();

    // Load assets (textures)
    let texture_registry = load_assets(&mut rl, &thread);

    let code = std::fs::read_to_string(&run_path).map_err(|_| "Run script read failed").expect("Run script read failed");

    // Register functions BEFORE running the script
    draw_register(&mut engine, draw_list.clone());
    input_register(&mut engine);
    crate::engine::physics::physics_register(&mut engine);

    engine.push_fn("floatToInt", |args| {
        if let Some(Token::Float(f)) = args.get(0) {
            return Token::Integer(*f as i64);
        }
        Token::Integer(0)
    });

    engine.run(&code).expect("Main script failed");
    let mut last_frame_time = std::time::Instant::now();
    let _ = engine.run("start()");

    // 1. Create variables to hold the crash state
    let mut script_crashed = false;
    let mut crash_message = String::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let now = std::time::Instant::now();
        let delta_time: f64 = now.duration_since(last_frame_time).as_secs_f64();
        last_frame_time = now;

        engine.push_float("deltaTime", delta_time);

        // 2. Only run the script if it hasn't crashed yet
        if !script_crashed {
            // Clear draw list for the new frame
            draw_list.lock().unwrap().clear();

            // 3. Wrap the engine call in catch_unwind
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Using run("update()") because it's more reliable in some versions of WolfLang
                engine.run("update()")
            }));

            // 4. Check if a panic was caught
            if let Err(panic_err) = result {
                script_crashed = true;

                // Try to extract the panic message
                if let Some(s) = panic_err.downcast_ref::<&'static str>() {
                    crash_message = s.to_string();
                } else if let Some(s) = panic_err.downcast_ref::<String>() {
                    crash_message = s.clone();
                } else {
                    crash_message = "Unknown script panic".to_string();
                }
            }

            // Draw the shapes collected during the update
            let shapes = draw_list.lock().unwrap();
            for shape in shapes.iter() {
                match shape {
                    Shapes::Circle { pos, rad, col } => d.draw_circle_v(pos, *rad, col),
                    Shapes::Rectangle { pos, size, col } => d.draw_rectangle_v(pos, size, col),
                    Shapes::Line { start, end, col } => d.draw_line_v(start, end, col),
                    Shapes::Texture { pos, tex_id, col } => {
                        if let Some(texture) = texture_registry.get(tex_id) {
                            d.draw_texture_v(texture, *pos, *col);
                        }
                    },
                }
            }
        }

        // 5. Draw the error screen if a crash happened
        if script_crashed {
            d.draw_rectangle(0, 0, screen_size_x, 50, Color::new(255, 0, 0, 200));
            d.draw_text(&format!("SCRIPT CRASHED: {}", crash_message), 10, 15, 20, Color::WHITE);

            // Hint to the user
            d.draw_text("Fix the script and restart the engine.", 10, 60, 20, Color::DARKGRAY);
        }
    }
}
