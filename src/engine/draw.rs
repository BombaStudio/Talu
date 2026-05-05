use raylib::prelude::*;
use wolflang::WolfEngine;
use wolflang::tokens::Token;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum Shapes {
    Circle { pos: Vector2, rad: f32, col: Color },
    Rectangle { pos: Vector2, size: Vector2, col: Color },
    Line { start: Vector2, end: Vector2, col: Color },
}

pub fn draw_register(engine: &mut WolfEngine, draw_list: Arc<Mutex<Vec<Shapes>>>) {

    let dl = draw_list.clone();
    engine.push_fn("drawRect", move |args| {
        if let (
            Some(Token::Float(x)),
            Some(Token::Float(y)),
            Some(Token::Float(w)),
            Some(Token::Float(h)),
            Some(Token::Float(r)),
            Some(Token::Float(g)),
            Some(Token::Float(b)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4), args.get(5), args.get(6)) {
            dl.lock().unwrap().push(Shapes::Rectangle {
                pos: Vector2 { x: *x as f32, y: *y as f32 },
                size: Vector2 { x: *w as f32, y: *h as f32 },
                col: Color::new(*r as u8, *g as u8, *b as u8, 255),
            });
        }
        Token::Unknown
    });

    let dl = draw_list.clone();
    engine.push_fn("drawCircle", move |args| {
        if let (
            Some(Token::Float(x)),
            Some(Token::Float(y)),
            Some(Token::Float(r)),
            Some(Token::Float(rc)),
            Some(Token::Float(gc)),
            Some(Token::Float(bc)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4), args.get(5)) {
            dl.lock().unwrap().push(Shapes::Circle {
                pos: Vector2 { x: *x as f32, y: *y as f32 },
                rad: *r as f32,
                col: Color::new(*rc as u8, *gc as u8, *bc as u8, 255),
            });
        }
        Token::Unknown
    });

    let dl = draw_list.clone();
    engine.push_fn("drawLine", move |args| {
        if let (
            Some(Token::Float(sx)),
            Some(Token::Float(sy)),
            Some(Token::Float(ex)),
            Some(Token::Float(ey)),
            Some(Token::Float(r)),
            Some(Token::Float(g)),
            Some(Token::Float(b)),
        ) = (args.get(0), args.get(1), args.get(2), args.get(3), args.get(4), args.get(5), args.get(6)) {
            dl.lock().unwrap().push(Shapes::Line {
                start: Vector2 { x: *sx as f32, y: *sy as f32 },
                end: Vector2 { x: *ex as f32, y: *ey as f32 },
                col: Color::new(*r as u8, *g as u8, *b as u8, 255),
            });
        }
        Token::Unknown
    });
}