use wolflang::tokens::Token;

pub fn opt_to_i32(i : Option<i64>) -> i32{
    i.and_then(|big| i32::try_from(big).ok()).unwrap_or(0)
}

pub fn get_str_slice(input: &Option<String>) -> &str {
    input.as_deref().unwrap_or("")
}

pub fn get_float(token: Option<&Token>) -> Option<f32> {
    match token {
        Some(Token::Float(f)) => Some(*f as f32),
        Some(Token::Integer(i)) => Some(*i as f32),
        _ => None,
    }
}
