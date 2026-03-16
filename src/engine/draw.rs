use raylib::prelude::*;
use wolflang::WolfEngine;
use wolflang::tokens::Token;

pub fn draw_register(engine: &mut WolfEngine) {
    engine.push_fn("drawRect", |args| {
        if let (
            Some(Token::Float(x)),
            Some(Token::Float(y)),
            Some(Token::Float(w)),
            Some(Token::Float(h)),
            Some(Token::Float(r_c)),
            Some(Token::Float(g_c)),
            Some(Token::Float(b_c)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4), args.get(5), args.get(6)) {
            unsafe {
                raylib::ffi::DrawRectangleV(
                    raylib::ffi::Vector2 { x: *x as f32, y: *y as f32 },
                    raylib::ffi::Vector2 { x: *w as f32, y: *h as f32 },
                    Color::new(*r_c as u8, *g_c as u8, *b_c as u8, 255).into()
                );
            }
        }
        Token::Unknown
    });

    engine.push_fn("drawCircle", |args| {
        
        if let (
            Some(Token::Float(x)),
            Some(Token::Float(y)),
            Some(Token::Float(r)),
            Some(Token::Float(r_c)),
            Some(Token::Float(g_c)),
            Some(Token::Float(b_c)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4), args.get(5)) {
            unsafe {
                raylib::ffi::DrawCircleV(
                    raylib::ffi::Vector2 {x: *x as f32, y: *y as f32},
                    *r as f32,
                    Color::new(*r_c as u8, *g_c as u8, *b_c as u8, 255).into()
                );
            }
        }
        Token::Unknown
    });

    engine.push_fn("drawLine", |args| {
        if let (
            Some(Token::Float(s_x)),
            Some(Token::Float(s_y)),
            Some(Token::Float(e_x)),
            Some(Token::Float(e_y)),
            Some(Token::Float(r_c)),
            Some(Token::Float(g_c)),
            Some(Token::Float(b_c)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4), args.get(5), args.get(6)) {
            unsafe {
                raylib::ffi::DrawLineV(
                    raylib::ffi::Vector2 {x: *s_x as f32, y: *s_y as f32},
                    raylib::ffi::Vector2 {x: *e_x as f32, y: *e_y as f32},
                    Color::new(*r_c as u8, *g_c as u8, *b_c as u8, 255).into()
                    
                );
            }
        }
        Token::Unknown
    });
}