use std::collections::HashMap;
use std::fs;
use std::path::Path;
use raylib::prelude::*;

pub fn load_assets(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<i32, Texture2D> {
    let mut texture_registry = HashMap::new();
    let mut id_counter = 1;

    if let Ok(entries) = fs::read_dir("assets") {
        let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        paths.sort_by_key(|e| e.path());

        for entry in paths {
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("png") {
                if let Ok(tex) = rl.load_texture(thread, path.to_str().unwrap()) {
                    println!("Loaded asset ID {}: {:?}", id_counter, path.file_name().unwrap());
                    texture_registry.insert(id_counter, tex);
                    id_counter += 1;
                }
            }
        }
    }
    texture_registry
}