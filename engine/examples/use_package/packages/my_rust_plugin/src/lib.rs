use wolflang::{WolfEngine, tokens::Token};

#[unsafe(no_mangle)]
pub extern "C" fn register_talu_plugin(engine_ptr: *mut WolfEngine) {
    let engine = unsafe { 
        if engine_ptr.is_null() { return; }
        &mut *engine_ptr 
    };
    
    println!(">>> My Rust Plugin is being registered!");
    
    engine.push_fn("hello_from_rust", |_args| {
        println!(">>> Hello! This message comes from a Rust dynamic plugin!");
        Token::Unknown
    });
}
