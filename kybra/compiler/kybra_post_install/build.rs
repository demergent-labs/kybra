use fs_extra::dir::{copy, CopyOptions};
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

static ERROR_PREFIX: &str = "Kybra Post Install Build Error";

fn main() -> Result<(), String> {
    let ic_build = get_and_set_ic_build();

    if ic_build == false {
        return Ok(());
    }

    let kybra_version = get_kybra_version()?;
    let home_dir = dirs::home_dir().ok_or(create_error_string("Home directory not found"))?;

    let python_stdlib_binary_path =
        home_dir.join(format!(".config/kybra/{kybra_version}/bin/python_stdlib"));
    handle_python_stdlib_exists(&python_stdlib_binary_path);

    let kybra_post_install_src_lib_path = Path::new("src/Lib");

    if kybra_post_install_src_lib_path.exists() {
        clean_kybra_post_install_src_lib(&kybra_post_install_src_lib_path)?;
    }

    let python_stdlib_src_path = home_dir.join(format!(".config/kybra/{kybra_version}/Lib"));

    if python_stdlib_src_path.exists() {
        copy_from_python_stdlib_src_to_kybra_post_install_src(
            &python_stdlib_src_path,
            &kybra_post_install_src_lib_path,
        )?;

        return Ok(());
    }

    let github_repo = "https://github.com/RustPython/RustPython.git";
    let commit_hash = "f12875027ce425297c07cbccb9be77514ed46157";

    let temp_dir = tempdir().map_err(|e| error_to_string(&e))?;
    git_clone(github_repo, &temp_dir)?;
    git_checkout(commit_hash, &temp_dir)?;

    let git_repo_lib_dir = temp_dir.path().join("Lib");
    copy_directory(&git_repo_lib_dir, &python_stdlib_src_path)?;
    copy_directory(&python_stdlib_src_path, &kybra_post_install_src_lib_path)?;

    temp_dir.close().map_err(|e| error_to_string(&e))?;

    Ok(())
}

fn get_kybra_version() -> Result<String, String> {
    std::env::var("KYBRA_VERSION")
        .map_err(|_| create_error_string("KYBRA_VERSION environment variable is not present"))
}

fn handle_python_stdlib_exists(python_stdlib_binary_path: &Path) {
    if python_stdlib_binary_path.exists() {
        println!("cargo:rustc-cfg=python_stdlib_exists");
    }
}

fn clean_kybra_post_install_src_lib(kybra_post_install_src_lib_path: &Path) -> Result<(), String> {
    fs_extra::dir::remove(kybra_post_install_src_lib_path).map_err(|e| error_to_string(&e))?;
    Ok(())
}

fn copy_from_python_stdlib_src_to_kybra_post_install_src(
    python_stdlib_src_path: &Path,
    kybra_post_install_src_lib_path: &Path,
) -> Result<u64, String> {
    copy(
        python_stdlib_src_path,
        kybra_post_install_src_lib_path,
        &CopyOptions {
            copy_inside: true,
            ..Default::default()
        },
    )
    .map_err(|e| error_to_string(&e))
}

fn git_clone(github_repo: &str, temp_dir: &tempfile::TempDir) -> Result<(), String> {
    let status = Command::new("git")
        .args(&[
            "clone",
            github_repo,
            temp_dir
                .path()
                .to_str()
                .ok_or(create_error_string("Could not convert path to &str"))?,
        ])
        .status()
        .map_err(|e| error_to_string(&e))?;

    if status.success() {
        Ok(())
    } else {
        Err(create_error_string("Failed to clone the repository"))
    }
}

fn git_checkout(commit_hash: &str, temp_dir: &tempfile::TempDir) -> Result<(), String> {
    let status = Command::new("git")
        .current_dir(temp_dir.path())
        .args(&["checkout", commit_hash])
        .status()
        .map_err(|e| error_to_string(&e))?;

    if status.success() {
        Ok(())
    } else {
        Err(create_error_string(
            "Could not checkout the specific commit",
        ))
    }
}

fn copy_directory(src: &Path, dst: &Path) -> Result<u64, String> {
    copy(
        src,
        dst,
        &CopyOptions {
            copy_inside: true,
            ..Default::default()
        },
    )
    .map_err(|e| error_to_string(&e))
}

fn error_to_string(e: &dyn std::error::Error) -> String {
    format!("{ERROR_PREFIX}: {}", e.to_string())
}

fn create_error_string(message: &str) -> String {
    format!("{ERROR_PREFIX}: {message}")
}

// This attempts to detect when the code is being compiled in a non-ic environment
// such as Rust Analyzer. The problem is that this build script and the main.rs
// are run by Rust Analyzer, and because the network requests, file IO, and compilation
// can be quite heavy, it makes local debugging very slow
// This is an attempt to allow Rust Analyzer to run quickly
fn get_and_set_ic_build() -> bool {
    match (std::env::var("KYBRA_VERSION"), std::env::var("DFX_NETWORK")) {
        (Ok(_), Ok(_)) => {
            println!("cargo:rustc-cfg=ic_build");
            true
        }
        _ => false,
    }
}
