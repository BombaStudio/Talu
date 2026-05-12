use wolflang::WolfEngine;
use wolflang::tokens::Token;
use rand::Rng;

use crate::engine::utils::keys::KEYS;

pub fn input(engine: &mut WolfEngine) {
    engine.push_fn("is_key_down", |args| {
        if let Some(Token::String(key)) = args.get(0) {
            let pressed = KEYS.iter()
                .find(|(name, _)| name == key)
                .map(|(_, k)| unsafe { raylib::ffi::IsKeyDown(*k as i32) })
                .unwrap_or(false);
            return Token::Boolean(pressed);
        }
        Token::Boolean(false)
    });

    engine.push_fn("is_key_up", |args| {
        if let Some(Token::String(key)) = args.get(0) {
            let pressed = KEYS.iter()
                .find(|(name, _)| name == key)
                .map(|(_, k)| unsafe { raylib::ffi::IsKeyUp(*k as i32) })
                .unwrap_or(false);
            return Token::Boolean(pressed);
        }
        Token::Boolean(false)
    });

    engine.push_fn("is_key_pressed", |args| {
        if let Some(Token::String(key)) = args.get(0) {
            let pressed = KEYS.iter()
                .find(|(name, _)| name == key)
                .map(|(_, k)| unsafe { raylib::ffi::IsKeyPressed(*k as i32) })
                .unwrap_or(false);
            return Token::Boolean(pressed);
        }
        Token::Boolean(false)
    });

    engine.push_fn("is_key_pressed_repeat", |args| {
        if let Some(Token::String(key)) = args.get(0) {
            let pressed = KEYS.iter()
                .find(|(name, _)| name == key)
                .map(|(_, k)| unsafe { raylib::ffi::IsKeyPressedRepeat(*k as i32) })
                .unwrap_or(false);
            return Token::Boolean(pressed);
        }
        Token::Boolean(false)
    });

    engine.push_fn("is_mouse_button_pressed", |args| {
        if let Some(Token::Integer(button)) = args.get(0) {
            let pressed = unsafe { raylib::ffi::IsMouseButtonPressed(*button as i32) };
            return Token::Boolean(pressed);
        }
        Token::Boolean(false)
    });

    engine.push_fn("get_mouse_x", |_| {
        let x = unsafe { raylib::ffi::GetMouseX() };
        Token::Float(x as f64)
    });

    engine.push_fn("get_mouse_y", |_| {
        let y = unsafe { raylib::ffi::GetMouseY() };
        Token::Float(y as f64)
    });

    engine.push_fn("random_float", |args| {
        if let (Some(Token::Float(min)), Some(Token::Float(max))) = (args.get(0), args.get(1)) {
            let mut rng = rand::thread_rng();
            let val: f64 = rng.gen_range(*min..*max);
            return Token::Float(val);
        }
        Token::Float(0.0)
    });
}