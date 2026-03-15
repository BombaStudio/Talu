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


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        
        engine.get_fn("update", vec![]);
    }
}
