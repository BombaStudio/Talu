use raylib::prelude::*;
use wolflang::WolfEngine;
use wolflang::tokens::Token;

pub fn draw_register(engine: &mut WolfEngine) {
    engine.push_fn("drawRect", |args| {
        if let (
            Some(Token::Integer(x)),
            Some(Token::Integer(y)),
            Some(Token::Integer(w)),
            Some(Token::Integer(h)),
            Some(Token::Float(r_c)),
            Some(Token::Float(g_c)),
            Some(Token::Float(b_c)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4), args.get(5), args.get(6)) {
            unsafe {
                raylib::ffi::DrawRectangle(
                    *x as i32, 
                    *y as i32, 
                    *w as i32, 
                    *h as i32, 
                    Color::new(*r_c as u8, *g_c as u8, *b_c as u8, 255).into()
                );
            }
        }
        Token::Unknown
    });

    engine.push_fn("drawCircle", |args| {
        if let (
            Some(Token::Integer(x)),
            Some(Token::Integer(y)),
            Some(Token::Float(r)),
            Some(Token::Float(r_c)),
            Some(Token::Float(g_c)),
            Some(Token::Float(b_c)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4), args.get(5)) {
            unsafe {
                raylib::ffi::DrawCircle(
                    *x as i32, 
                    *y as i32, 
                    *r as f32,
                    Color::new(*r_c as u8, *g_c as u8, *b_c as u8, 255).into()
                );
            }
        }
        Token::Unknown
    });

    engine.push_fn("drawLine", |args| {
        if let (
            Some(Token::Integer(s_x)),
            Some(Token::Integer(s_y)),
            Some(Token::Integer(e_x)),
            Some(Token::Integer(e_y)),
            Some(Token::Float(r_c)),
            Some(Token::Float(g_c)),
            Some(Token::Float(b_c)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4), args.get(5), args.get(6)) {
            unsafe {
                raylib::ffi::DrawLine(
                    *s_x as i32, 
                    *s_y as i32, 
                    *e_x as i32,
                    *e_y as i32,
                    Color::new(*r_c as u8, *g_c as u8, *b_c as u8, 255).into()
                    
                );
            }
        }
        Token::Unknown
    });
}