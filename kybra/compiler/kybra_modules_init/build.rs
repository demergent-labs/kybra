fn main() {
    let kybra_version = std::env::var("KYBRA_VERSION").unwrap();

    let python_stdlib_path = dirs::home_dir()
        .unwrap()
        .join(format!(".config/kybra/bin/{kybra_version}/python_stdlib"));

    if python_stdlib_path.exists() {
        println!("cargo:rustc-cfg=python_stdlib_exists");
    }
}
