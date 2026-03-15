use wolflang::WolfEngine;
use wolflang::tokens::Token;
use raylib::prelude::*;
use raylib::color::Color;

mod utils;
use utils::utils::*;

mod draw;
mod input;

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
    
    engine.push_fn("draw_rect", |args| {
        if let (
            Some(Token::Integer(x)),
            Some(Token::Integer(y)),
            Some(Token::Integer(w)),
            Some(Token::Integer(h)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3)) {
            unsafe {
                DrawRectangle(
                    *x as i32, 
                    *y as i32, 
                    *w as i32, 
                    *h as i32, 
                    Color::RED.into()
                );
            }
        }

        Token::Unknown
    });

    while !rl.window_should_close() {
        // Çizim aşamasını başlat
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);
        d.draw_text("Merhaba Rust ve Raylib!", 190, 200, 20, Color::DARKGRAY);

        engine.get_fn("update", vec![]);
    }
}
