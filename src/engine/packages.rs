use std::path::PathBuf;
use wolflang::WolfEngine;

pub fn load_packages(_engine: &mut WolfEngine, plugins: Vec<String>) -> Vec<libloading::Library> {
    let mut loaded_libs = Vec::new();
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    println!("Package loader starting. CWD: {:?}", cwd);

    for plugin_name in plugins {
        if plugin_name.is_empty() { continue; }

        let lib_filename = format!(
            "{}{}.{}",
            std::env::consts::DLL_PREFIX,
            plugin_name,
            std::env::consts::DLL_EXTENSION
        );

        // Potential paths to look for the library
        let mut search_paths = vec![
            cwd.join("packages").join(&lib_filename),
            cwd.join("packages").join(&plugin_name).join("target/release").join(&lib_filename),
            cwd.join(&lib_filename),
        ];

        // Also check engine root if we are in a subdirectory
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(engine_root) = exe_path.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
                search_paths.push(engine_root.join("packages").join(&lib_filename));
                search_paths.push(engine_root.join("packages").join(&plugin_name).join("target/release").join(&lib_filename));
            }
        }
        
        // Manual fallback for this specific environment
        search_paths.push(PathBuf::from("/mnt/hangar/Talu/packages").join(&lib_filename));

        let mut loaded = false;
        for path in search_paths {
            let abs_path = if path.is_absolute() { path } else { cwd.join(path) };
            
            if abs_path.exists() {
                unsafe {
                    match libloading::Library::new(&abs_path) {
                        Ok(lib) => {
                            // ABI-safe symbol lookup
                            match lib.get::<unsafe extern "C" fn(*mut WolfEngine)>(b"register_talu_plugin") {
                                Ok(_func) => {
                                    // func(engine as *mut WolfEngine); 
                                    loaded_libs.push(lib);
                                    loaded = true;
                                    println!("Successfully located Rust plugin: {}", plugin_name);
                                    break;
                                }
                                Err(e) => {
                                    eprintln!("Failed to find symbol 'register_talu_plugin' in {:?}: {:?}", abs_path, e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to load Rust dynamic library {:?}: {:?}", abs_path, e);
                        }
                    }
                }
            }
        }

        if !loaded {
            eprintln!("Error: Could not find or load plugin library for '{}'", plugin_name);
        }
    }

    loaded_libs
}
