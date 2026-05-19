use std::collections::HashMap;
use std::env;
use std::path::Path;

pub fn get_manifest(content: &str) -> HashMap<String, String> {
    let mut manifest = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            manifest.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    manifest
}

pub fn cli() -> Option<(String, String, Vec<String>)> {
    let args: Vec<String> = env::args().collect();

    let target_dir = if args.len() > 1 {
        &args[1]
    } else {
        "." // Default to current directory if no argument is passed
    };

    let target_path = Path::new(target_dir);
    if let Err(e) = env::set_current_dir(&target_path) {
        eprintln!("CRITICAL ERROR: Could not find or open game folder '{}'.", target_dir);
        eprintln!("Details: {}", e);
        std::process::exit(1); // Kill the engine if the folder doesn't exist
    }

    println!("Talu Engine Booting...");
    println!("Working Directory: {:?}", env::current_dir().unwrap());

    let file = std::fs::read_to_string("package.talu").expect("Could not find package.talu");

    let manifest = get_manifest(&file);
    let config = manifest.get("config").map(|s| s.to_string()).unwrap_or_default();
    let run = manifest.get("run").map(|s| s.to_string()).unwrap_or_default();
    let plugins = manifest.get("plugins")
        .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
        .unwrap_or_default();

    Some((config, run, plugins))
}