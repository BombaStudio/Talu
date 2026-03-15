use raylib::prelude::*;
use wolflang::WolfEngine;
use wolflang::tokens::Token;
use raylib::color::Color;
use raylib::prelude::*;

fn draw(d: &mut RaylibDrawHandle, engine: &mut WolfEngine) {
    engine.push_fn("draw_rect", |args| {
        if let (
            Some(Token::Integer(x)),
            Some(Token::Integer(y)),
            Some(Token::Integer(w)),
            Some(Token::Integer(h)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3)) {
            
            // Use the handle 'd' to draw safely
            d.draw_rectangle(
                *x as i32, 
                *y as i32, 
                *w as i32, 
                *h as i32, 
                Color::RED
            );
        }
        Token::Unknown
    });
}