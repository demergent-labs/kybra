// TODO share with DFINITY how I'm doing this, maybe they can build that into dfx to overcome some of the Wasm binary limitations temporarily
// TODO a future version of dfx should take care of all of this chunk uploading for us

use sha2::{Digest, Sha256};
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let canister_name = &args[1];

    let max_chunk_size = 2 * 1_000 * 1_000; // 2 MB

    let wasm = std::fs::read(format!(
        ".kybra/{canister_name}/{canister_name}_app.wasm.gz"
    ))
    .unwrap();
    let wasm_chunks = split_into_chunks(wasm, max_chunk_size);

    for wasm_chunk in wasm_chunks {
        upload_chunk(canister_name, wasm_chunk, "upload_wasm_chunk");
    }

    let kybra_version = std::env::var("KYBRA_VERSION").unwrap();

    let python_stdlib_path = dirs::home_dir()
        .unwrap()
        .join(format!(".config/kybra/bin/{kybra_version}/python_stdlib"));

    #[cfg(not(python_stdlib_exists))]
    let python_stdlib_modules = rustpython_vm::py_freeze!(dir = "src/Lib");

    #[cfg(not(python_stdlib_exists))]
    let python_stdlib_bytecode = python_stdlib_modules.bytes.to_vec();

    #[cfg(not(python_stdlib_exists))]
    std::fs::write(python_stdlib_path, &python_stdlib_bytecode).unwrap();

    #[cfg(python_stdlib_exists)]
    let python_stdlib_bytecode = std::fs::read(python_stdlib_path).unwrap();

    let mut hasher = Sha256::new();

    hasher.update(&python_stdlib_bytecode);

    let result = hasher.finalize();

    let hash_hex = hex::encode(result);

    let python_stdlib_hash_output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg(canister_name)
        .arg("python_stdlib_hash")
        .output()
        .expect("Failed to execute the dfx canister id command");

    if python_stdlib_hash_output.status.success() {
        println!(
            "{}",
            format!(
                "Output from {canister_name}/python_stdlib_hash: {:?}",
                String::from_utf8_lossy(&python_stdlib_hash_output.stdout)
            )
        );
    } else {
        panic!(
            "Error: {:?}",
            String::from_utf8_lossy(&python_stdlib_hash_output.stderr)
        );
    }

    if format!("(\"{hash_hex}\")")
        != String::from_utf8_lossy(&python_stdlib_hash_output.stdout).trim()
    {
        let python_stdlib_bytecode_chunks =
            split_into_chunks(python_stdlib_bytecode, max_chunk_size);

        // Upload Python stdlib bytecode
        for python_stdlib_bytecode_chunk in python_stdlib_bytecode_chunks {
            upload_chunk(
                canister_name,
                python_stdlib_bytecode_chunk,
                "upload_python_stdlib_chunk",
            );
        }
    }

    let canister_id_output = Command::new("dfx")
        .arg("canister")
        .arg("id")
        .arg(canister_name)
        .output()
        .expect("Failed to execute the dfx canister id command");

    let canister_id = String::from_utf8_lossy(&canister_id_output.stdout)
        .trim()
        .to_string();

    // TODO we shouldn't just leave this...
    // TODO we should check if it is already the controller
    // TODO if it is, don't remove it. If it isn't add it and then remove it
    let add_controller_output = Command::new("dfx")
        .arg("canister")
        .arg("update-settings")
        .arg("--add-controller")
        .arg(canister_id)
        .arg(canister_name)
        .output()
        .expect("Failed to execute the dfx command");

    if add_controller_output.status.success() {
        println!(
            "Output from canister update-settings: {:?}",
            String::from_utf8_lossy(&add_controller_output.stdout)
        );
    } else {
        panic!(
            "Error: {:?}",
            String::from_utf8_lossy(&add_controller_output.stderr)
        );
    }

    let install_output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg(canister_name)
        .arg("install_wasm")
        .output()
        .expect("Failed to execute the dfx command");

    // TODO this will error out until we use notify
    if install_output.status.success() {
        println!(
            "{}",
            format!(
                "Output from {canister_name}/install_wasm: {:?}",
                String::from_utf8_lossy(&install_output.stdout)
            )
        );
    } else {
        panic!(
            "Error: {:?}",
            String::from_utf8_lossy(&install_output.stderr)
        );
    }
}

fn upload_chunk(canister_name: &str, bytecode_chunk: Vec<u8>, canister_method_name: &str) {
    let blob_string = vec_u8_to_blob_string(&bytecode_chunk);

    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    temp_file
        .write_all(blob_string.as_bytes())
        .expect("Failed to write data to temporary file");

    let output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg(canister_name)
        .arg(canister_method_name)
        .arg("--argument-file")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute the dfx command");

    if output.status.success() {
        println!(
            "{}",
            format!(
                "Output from {canister_name}/{canister_method_name}: {:?}",
                String::from_utf8_lossy(&output.stdout)
            )
        );
    } else {
        panic!("Error: {:?}", String::from_utf8_lossy(&output.stderr));
    }
}

fn split_into_chunks(data: Vec<u8>, chunk_size: usize) -> Vec<Vec<u8>> {
    let mut chunks = Vec::new();
    let mut start = 0;
    let data_len = data.len();

    while start < data_len {
        let end = usize::min(start + chunk_size, data_len);
        chunks.push(data[start..end].to_vec());
        start = end;
    }

    chunks
}

fn vec_u8_to_blob_string(data: &[u8]) -> String {
    let mut result = String::from("(blob \"");
    for &byte in data {
        result.push_str(&format!("\\{:02X}", byte));
    }
    result.push_str("\")");
    result
}
