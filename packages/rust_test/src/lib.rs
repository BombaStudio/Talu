use wolflang::{WolfEngine, tokens::Token};

#[unsafe(no_mangle)]
pub extern "C" fn register_talu_plugin(engine_ptr: *mut WolfEngine) {
    let engine = unsafe { 
        if engine_ptr.is_null() { return; }
        &mut *engine_ptr 
    };
    
    println!("Rust package registered successfully!");
    engine.push_fn("rust_hello", |_args| {
        println!("Hello from Rust package function!");
        Token::Integer(42)
    });
}
