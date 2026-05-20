use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "new" => {
            if args.len() < 3 {
                eprintln!("Usage: talu new <project-name>");
                std::process::exit(1);
            }
            cmd_new(&args[2]);
        }
        "run" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            cmd_run(path);
        }
        "build" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            cmd_build(path);
        }
        "new-plugin" => {
            if args.len() < 3 {
                eprintln!("Usage: talu new-plugin <name>");
                std::process::exit(1);
            }
            cmd_new_plugin(&args[2]);
        }
        "build-plugin" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            cmd_build_plugin(path);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}


fn print_help() {
    println!("Talu Engine CLI");
    println!("Usage:");
    println!("  talu new <name>          Create a new project");
    println!("  talu run [path]          Run a project");
    println!("  talu build [path]        Package project for distribution");
    println!("  talu new-plugin <name>   Scaffold a new Rust plugin");
    println!("  talu build-plugin [path] Compile and install a plugin");
}

fn cmd_new(name: &str) {
    if Path::new(name).exists() {
        eprintln!("Error: folder '{}' already exists", name);
        std::process::exit(1);
    }

    fs::create_dir_all(format!("{}/assets", name)).unwrap();
    fs::create_dir_all(format!("{}/packages", name)).unwrap();

    fs::write(format!("{}/package.talu", name),
        "config = config.wolf\nrun = main.wolf\n"
    ).unwrap();

    fs::write(format!("{}/config.wolf", name),
        "let screen_size_x : int = 800\nlet screen_size_y : int = 600\nlet title : string = \"My Talu Game\"\n"
    ).unwrap();

    fs::write(format!("{}/main.wolf", name),
        "fn start()\n    print \"Game started!\"\nend\n\nfn update()\nend\n"
    ).unwrap();

    println!("Created project '{}'", name);
    println!("  cd {}", name);
    println!("  talu run");
}

fn cmd_run(path: &str) {
    let package_file = Path::new(path).join("package.talu");
    if !package_file.exists() {
        eprintln!("Error: no package.talu found in '{}'", path);
        std::process::exit(1);
    }

    let engine_path = get_engine_path();
    if !engine_path.exists() {
        eprintln!("Error: talu-engine binary not found at {:?}", engine_path);
        std::process::exit(1);
    }

    let status = Command::new(&engine_path)
        .arg(path)
        .status()
        .expect("Failed to launch talu-engine");

    std::process::exit(status.code().unwrap_or(1));
}

fn cmd_build(path: &str) {
    let package_file = Path::new(path).join("package.talu");
    if !package_file.exists() {
        eprintln!("Error: no package.talu found in '{}'", path);
        std::process::exit(1);
    }

    let dist = Path::new(path).join("dist");
    fs::create_dir_all(&dist).unwrap();

    // Copy engine binary
    let engine_path = get_engine_path();
    let engine_name = if cfg!(windows) { "talu-engine.exe" } else { "talu-engine" };
    fs::copy(&engine_path, dist.join(engine_name)).unwrap();

    // Copy all project files
    for file in ["package.talu", "config.wolf", "main.wolf"] {
        let src = Path::new(path).join(file);
        if src.exists() {
            fs::copy(&src, dist.join(file)).unwrap();
        }
    }

    // Copy assets/ and packages/ folders
    copy_dir(&Path::new(path).join("assets"), &dist.join("assets"));
    copy_dir(&Path::new(path).join("packages"), &dist.join("packages"));

    println!("Build complete: {:?}", dist);
    println!("Ship the 'dist/' folder — it contains everything needed to run the game.");
}

fn cmd_new_plugin(name: &str) {
    if Path::new(name).exists() {
        eprintln!("Error: folder '{}' already exists", name);
        std::process::exit(1);
    }

    fs::create_dir_all(format!("{}/src", name)).unwrap();

    fs::write(format!("{}/Cargo.toml", name), format!(
r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wolflang = {{ git = "https://github.com/islamfazliyev/Wolf-Lang" }}
"#, name)).unwrap();

    fs::write(format!("{}/src/lib.rs", name),
r#"use wolflang::WolfEngine;
use wolflang::tokens::Token;

#[no_mangle]
pub extern "C" fn register_talu_plugin(engine: *mut WolfEngine) {
    let engine = unsafe { &mut *engine };

    engine.push_fn("hello_from_plugin", |_args| {
        println!("Hello from Rust plugin!");
        Token::Unknown
    });
}
"#).unwrap();

    println!("Created plugin '{}'", name);
    println!("  cd {}", name);
    println!("  talu build-plugin");
}

fn cmd_build_plugin(path: &str) {
    let cargo_toml = Path::new(path).join("Cargo.toml");
    if !cargo_toml.exists() {
        eprintln!("Error: no Cargo.toml found in '{}'", path);
        std::process::exit(1);
    }

    // Get the crate name from Cargo.toml
    let toml_content = fs::read_to_string(&cargo_toml).unwrap();
    let plugin_name = toml_content.lines()
        .find(|l| l.trim().starts_with("name"))
        .and_then(|l| l.split('=').nth(1))
        .map(|s| s.trim().trim_matches('"').to_string())
        .expect("Could not parse plugin name from Cargo.toml");

    println!("Building plugin '{}'...", plugin_name);

    // Run cargo build --release inside the plugin folder
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(path)
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        eprintln!("Plugin build failed.");
        std::process::exit(1);
    }

    // Figure out the compiled filename
    let lib_filename = format!(
        "{}{}.{}",
        std::env::consts::DLL_PREFIX,
        plugin_name,
        std::env::consts::DLL_EXTENSION
    );

    let src = Path::new(path).join("target/release").join(&lib_filename);

    // Install to ../packages/ relative to plugin folder, or ./packages/ if at root
    let packages_dir = Path::new(path).join("../packages");
    let packages_dir = if packages_dir.exists() {
        packages_dir
    } else {
        Path::new(path).join("packages")
    };
    fs::create_dir_all(&packages_dir).unwrap();

    let dst = packages_dir.join(&lib_filename);
    fs::copy(&src, &dst).unwrap();

    println!("Plugin installed: {:?}", dst);
    println!("Add '{}' to the plugins key in your package.talu", plugin_name);
}

// Recursively copy a directory
fn copy_dir(src: &Path, dst: &Path) {
    if !src.exists() { return; }
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let dst_path = dst.join(entry.file_name());
        if entry.path().is_dir() {
            copy_dir(&entry.path(), &dst_path);
        } else {
            fs::copy(entry.path(), dst_path).unwrap();
        }
    }
}

fn get_engine_path() -> std::path::PathBuf {
    // Look next to the current executable
    let mut path = env::current_exe().unwrap();
    path.pop(); // remove the 'talu' binary name

    let engine_name = if cfg!(windows) {
        "talu-engine.exe"
    } else {
        "talu-engine"
    };

    path.join(engine_name)
}