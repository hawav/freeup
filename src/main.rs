use std::{env::current_dir, fs, path::PathBuf, process::Command};

fn main() {
    let current_dir = current_dir().unwrap();
    println!("Cleaning project artifacts in {:?}. This will remove all node_modules directories and run 'cargo clean' for Rust projects.", current_dir);

    // Remove all node_modules directories
    remove_node_modules(&current_dir);

    // Run 'cargo clean' for Rust projects
    run_cargo_clean(&current_dir);
}

fn remove_node_modules(directory: &PathBuf) {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.is_dir() {
            if path.ends_with("node_modules") {
                println!("Removing {}", path.display());

                fs::remove_dir_all(&path).unwrap();
            }

            remove_node_modules(&path);
        }
    }
}

fn run_cargo_clean(directory: &PathBuf) {
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.is_dir() {
            if path.join("Cargo.toml").exists() {
                println!("Running 'cargo clean' in {}", path.display());

                Command::new("cargo")
                    .current_dir(&path)
                    .arg("clean")
                    .status()
                    .unwrap();
            }

            run_cargo_clean(&path);
        }
    }
}
