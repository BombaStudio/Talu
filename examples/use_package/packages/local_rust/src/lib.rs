use wolflang::{WolfEngine, tokens::Token};

#[unsafe(no_mangle)]
pub extern "Rust" fn register_talu_plugin(engine: &mut WolfEngine) {
    println!("Project-level Rust package registered successfully!");
    engine.push_fn("local_rust_hello", |_args| {
        println!("Hello from local_rust (project-level Rust package)!");
        Token::Integer(42)
    });
}
