use wolflang::WolfEngine;
use wolflang::tokens::Token;
use raylib::prelude::*;
use raylib::color::Color;

mod engine;
use engine::utils::utils::*;

use engine::draw::draw_register;
use engine::input::input as input_register;

fn main() {
    let mut engine : WolfEngine = WolfEngine::new();

    let mut game_config = std::fs::read_to_string("config.wolf").unwrap();
    engine.run(&game_config);

    let mut screen_size_x = opt_to_i32(engine.get_int("screen_size_x"));
    let mut screen_size_y = opt_to_i32(engine.get_int("screen_size_y"));

    let t : Option<String> = engine.get_str("title");

    let mut title : &str = get_str_slice(&t);

    let (mut rl, thread) = raylib::init()
        .size(screen_size_x, screen_size_y)
        .title(title)
        .build();


    let code = std::fs::read_to_string("test.wolf").unwrap();
    engine.run(&code);
    engine.get_fn("start", vec![]);
    draw_register(&mut engine);
    input_register(&mut engine);

    // Add this at the top of your file if it's not there:
    // use std::panic;

    // 1. Create variables to hold the crash state
    let mut script_crashed = false;
    let mut crash_message = String::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        
        // 2. Only run the script if it hasn't crashed yet
        if !script_crashed {
            // 3. Wrap the engine call in catch_unwind
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                engine.get_fn("update", vec![])
            }));

            // 4. Check if a panic was caught
            if let Err(panic_err) = result {
                script_crashed = true;
                
                // Try to extract the panic message
                crash_message = if let Some(s) = panic_err.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown script panic".to_string()
                };
            }
        }

        // 5. Draw the error screen if a crash happened
        if script_crashed {
            d.draw_rectangle(0, 0, screen_size_x, 50, Color::new(255, 0, 0, 200));
            d.draw_text(&format!("SCRIPT CRASHED: {}", crash_message), 10, 15, 20, Color::WHITE);
            
            // Optional: Hint to the user
            d.draw_text("Fix the script and restart the engine.", 10, 60, 20, Color::DARKGRAY);
        }
    }
}