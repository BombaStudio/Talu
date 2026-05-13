use raylib::prelude::*;
use wolflang::WolfEngine;
use wolflang::tokens::Token;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum Shapes {
    Circle { pos: Vector2, rad: f32, col: Color },
    Rectangle { pos: Vector2, size: Vector2, col: Color },
    Line { start: Vector2, end: Vector2, col: Color },
    Texture { pos: Vector2, tex_id: i32, col: Color },
}

fn get_float(token: Option<&Token>) -> Option<f32> {
    match token {
        Some(Token::Float(f)) => Some(*f as f32),
        Some(Token::Integer(i)) => Some(*i as f32),
        _ => None,
    }
}

pub fn draw_register(engine: &mut WolfEngine, draw_list: Arc<Mutex<Vec<Shapes>>>) {
    let dl = draw_list.clone();
    engine.push_fn("drawRect", move |args| {
        let x = get_float(args.get(0));
        let y = get_float(args.get(1));
        let w = get_float(args.get(2));
        let h = get_float(args.get(3));
        let r = get_float(args.get(4));
        let g = get_float(args.get(5));
        let b = get_float(args.get(6));

        if let (Some(x), Some(y), Some(w), Some(h), Some(r), Some(g), Some(b)) = (x, y, w, h, r, g, b) {
            dl.lock().unwrap().push(Shapes::Rectangle {
                pos: Vector2 { x, y },
                size: Vector2 { x: w, y: h },
                col: Color::new(r as u8, g as u8, b as u8, 255),
            });
        }
        Token::Unknown
    });

    let dl = draw_list.clone();
    engine.push_fn("drawCircle", move |args| {
        let x = get_float(args.get(0));
        let y = get_float(args.get(1));
        let r = get_float(args.get(2));
        let rc = get_float(args.get(3));
        let gc = get_float(args.get(4));
        let bc = get_float(args.get(5));

        if let (Some(x), Some(y), Some(r), Some(rc), Some(gc), Some(bc)) = (x, y, r, rc, gc, bc) {
            dl.lock().unwrap().push(Shapes::Circle {
                pos: Vector2 { x, y },
                rad: r,
                col: Color::new(rc as u8, gc as u8, bc as u8, 255),
            });
        }
        Token::Unknown
    });

    let dl = draw_list.clone();
    engine.push_fn("drawLine", move |args| {
        let sx = get_float(args.get(0));
        let sy = get_float(args.get(1));
        let ex = get_float(args.get(2));
        let ey = get_float(args.get(3));
        let r = get_float(args.get(4));
        let g = get_float(args.get(5));
        let b = get_float(args.get(6));

        if let (Some(sx), Some(sy), Some(ex), Some(ey), Some(r), Some(g), Some(b)) = (sx, sy, ex, ey, r, g, b) {
            dl.lock().unwrap().push(Shapes::Line {
                start: Vector2 { x: sx, y: sy },
                end: Vector2 { x: ex, y: ey },
                col: Color::new(r as u8, g as u8, b as u8, 255),
            });
        }
        Token::Unknown
    });

    let dl = draw_list.clone();
    engine.push_fn("drawTexture", move |args| {
        let id = get_float(args.get(0));
        let x = get_float(args.get(1));
        let y = get_float(args.get(2));
        let r = get_float(args.get(3));
        let g = get_float(args.get(4));
        let b = get_float(args.get(5));

        if let (Some(id), Some(x), Some(y), Some(r), Some(g), Some(b)) = (id, x, y, r, g, b) {
            dl.lock().unwrap().push(Shapes::Texture {
                pos: Vector2 { x, y },
                tex_id: id as i32,
                col: Color::new(r as u8, g as u8, b as u8, 255),
            });
        }
        Token::Unknown
    });
}
