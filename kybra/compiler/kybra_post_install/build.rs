use std::process::Command;
use tempfile::tempdir;

fn main() {
    // The unwrap_or is so that this doesn't break RustAnalyzer
    let kybra_version = std::env::var("KYBRA_VERSION").unwrap_or("0.0.0".to_string());

    // TODO we should store the src stdlib in the global area as well
    // TODO we should then conditionally compile the github code in the main.rs I think
    let python_stdlib_binary_path = dirs::home_dir()
        .unwrap()
        .join(format!(".config/kybra/{kybra_version}/bin/python_stdlib"));

    if python_stdlib_binary_path.exists() {
        println!("cargo:rustc-cfg=python_stdlib_exists");
    }

    let python_stdlib_src_path = dirs::home_dir()
        .unwrap()
        .join(format!(".config/kybra/{kybra_version}/Lib"));

    if python_stdlib_src_path.exists() {
        fs_extra::dir::copy(
            &python_stdlib_src_path,
            "src/Lib",
            &fs_extra::dir::CopyOptions {
                copy_inside: true,
                ..Default::default()
            },
        )
        .expect("Failed to copy directory.");

        return;
    }

    // Define the GitHub repository URL
    let github_repo = "https://github.com/RustPython/RustPython.git";

    // TODO eventually we will want this hash to be our exact RustPython dependency hash
    // TODO but right now our RustPython fork is behind the live RustPython
    let commit_hash = "f12875027ce425297c07cbccb9be77514ed46157";

    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temp dir");

    // Run the git clone command
    let status = Command::new("git")
        .args(&["clone", github_repo, temp_dir.path().to_str().unwrap()])
        .status()
        .expect("Failed to execute git clone.");

    if !status.success() {
        panic!("Could not clone the repository.");
    }

    // Checkout the specific commit
    let status = Command::new("git")
        .current_dir(temp_dir.path())
        .args(&["checkout", commit_hash])
        .status()
        .expect("Failed to checkout the specific commit.");

    if !status.success() {
        panic!("Could not checkout the specific commit.");
    }

    // Define the source directory path (the one to copy) and the destination path
    let src_dir = temp_dir.path().join("Lib");
    let dest_dir = python_stdlib_src_path;

    // Use a crate like fs_extra to copy the directory recursively
    fs_extra::dir::copy(
        &src_dir,
        &dest_dir,
        &fs_extra::dir::CopyOptions {
            copy_inside: true,
            ..Default::default()
        },
    )
    .expect("Failed to copy directory.");

    fs_extra::dir::copy(
        &dest_dir,
        "src/Lib",
        &fs_extra::dir::CopyOptions {
            copy_inside: true,
            ..Default::default()
        },
    )
    .expect("Failed to copy directory.");

    // Close (and delete) the temporary directory
    let _ = temp_dir.close();
}
