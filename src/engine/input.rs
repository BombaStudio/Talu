use wolflang::WolfEngine;
use wolflang::tokens::Token;

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
}