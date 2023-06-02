fn main() {
    // The unwrap_or is so that this doesn't break RustAnalyzer
    let kybra_version = std::env::var("KYBRA_VERSION").unwrap_or("0.0.0".to_string());

    let python_stdlib_path = dirs::home_dir()
        .unwrap()
        .join(format!(".config/kybra/bin/{kybra_version}/python_stdlib"));

    if python_stdlib_path.exists() {
        println!("cargo:rustc-cfg=python_stdlib_exists");
    }
}
