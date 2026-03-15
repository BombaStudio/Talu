

pub fn opt_to_i32(i : Option<i64>) -> i32{
    return i.and_then(|big| i32::try_from(big).ok()).unwrap_or(0);;
}
pub fn get_str_slice(input: &Option<String>) -> &str {
    input.as_deref().unwrap_or("")
}

