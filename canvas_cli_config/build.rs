use std::path::PathBuf;

fn main() {
    let mut path = dirs::config_dir().unwrap_or(PathBuf::from("."));
    path.push("canvas_cli");
    path.push("config.toml");

    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut example_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    example_path.push("config.toml.example");

    if !path.exists() {
        std::fs::copy(&example_path, &path).expect("Unable to copy example config file!");
    }

    println!("cargo:rerun-if-changed=build.rs");
}
