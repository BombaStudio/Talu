use std::env;
use std::path::Path;

pub fn get_files(package_content: &str) -> [(&str, &str); 2] {
    let mut config_file = "";
    let mut run_file = "";

    for line in package_content.lines() {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        
        if parts.len() == 2 {
            match parts[0] {
                "config" => config_file = parts[1],
                "run" => run_file = parts[1],
                _ => {} // Ignore unknown keys
            }
        }
    }

    // Return the array of string slices
    [
        ("config", config_file),
        ("run", run_file)
    ]
}

pub fn cli() -> Option<(String, String)> {
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

    let files = get_files(&file);
    let mut config = "";
    let mut run = "";
    for (key, val) in files {
        if key == "config" { config = val; }
        if key == "run" { run = val; }
    }
    Some((config.to_string(), run.to_string()))
}