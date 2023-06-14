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
    let dfx_network = std::env::var("DFX_NETWORK").unwrap();

    upload_app_canister(canister_name, max_chunk_size, &dfx_network);
    upload_python_stdlib(canister_name, max_chunk_size, &dfx_network);
    set_permissions(canister_name, &dfx_network);
    install_app_canister(canister_name, &dfx_network);
}

fn upload_app_canister(canister_name: &str, max_chunk_size: usize, dfx_network: &str) {
    let wasm = std::fs::read(format!(
        ".kybra/{canister_name}/{canister_name}_app.wasm.gz"
    ))
    .unwrap();

    let wasm_chunks = split_into_chunks(wasm, max_chunk_size);

    for (index, wasm_chunk) in wasm_chunks.iter().enumerate() {
        upload_chunk(
            &format!("{canister_name} wasm"),
            canister_name,
            wasm_chunk,
            "upload_wasm_chunk",
            dfx_network,
            index,
            wasm_chunks.len(),
        );
    }
}

fn upload_python_stdlib(canister_name: &str, max_chunk_size: usize, dfx_network: &str) {
    let python_stdlib_bytecode = get_python_stdlib_bytecode();

    let mut hasher = Sha256::new();
    hasher.update(&python_stdlib_bytecode);
    let result = hasher.finalize();
    let local_python_stdlib_bytecode_hash = hex::encode(result);

    let remote_python_stdlib_hash_output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg("--network")
        .arg(dfx_network)
        .arg(canister_name)
        .arg("python_stdlib_hash")
        .output()
        .expect("Failed to execute the dfx canister id command");

    if !remote_python_stdlib_hash_output.status.success() {
        panic!(
            "Error: {:?}",
            String::from_utf8_lossy(&remote_python_stdlib_hash_output.stderr)
        );
    }

    let remote_python_stdlib_bytecode_hash =
        String::from_utf8_lossy(&remote_python_stdlib_hash_output.stdout)
            .trim()
            .to_string();

    let hashes_do_not_match =
        format!("(\"{local_python_stdlib_bytecode_hash}\")") != remote_python_stdlib_bytecode_hash;

    if hashes_do_not_match {
        let python_stdlib_bytecode_chunks =
            split_into_chunks(python_stdlib_bytecode, max_chunk_size);

        // Upload Python stdlib bytecode
        for (index, python_stdlib_bytecode_chunk) in
            python_stdlib_bytecode_chunks.iter().enumerate()
        {
            upload_chunk(
                "Python stdlib",
                canister_name,
                python_stdlib_bytecode_chunk,
                "upload_python_stdlib_chunk",
                dfx_network,
                index,
                python_stdlib_bytecode_chunks.len(),
            );
        }
    }
}

fn get_python_stdlib_bytecode() -> Vec<u8> {
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

    python_stdlib_bytecode
}

fn set_permissions(canister_name: &str, dfx_network: &str) {
    let canister_id_output = Command::new("dfx")
        .arg("canister")
        .arg("id")
        .arg("--network")
        .arg(dfx_network)
        .arg(canister_name)
        .output()
        .expect("Failed to execute the dfx canister id command");

    let canister_id = String::from_utf8_lossy(&canister_id_output.stdout)
        .trim()
        .to_string();

    let add_controller_output = Command::new("dfx")
        .arg("canister")
        .arg("update-settings")
        .arg("--network")
        .arg(dfx_network)
        .arg("--add-controller")
        .arg(canister_id)
        .arg(canister_name)
        .output()
        .expect("Failed to execute the dfx command");

    if !add_controller_output.status.success() {
        panic!(
            "Error: {:?}",
            String::from_utf8_lossy(&add_controller_output.stderr)
        );
    }
}

fn install_app_canister(canister_name: &str, dfx_network: &str) {
    let install_output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg("--network")
        .arg(dfx_network)
        .arg(canister_name)
        .arg("install_wasm")
        .output()
        .expect("Failed to execute the dfx command");

    if !install_output.status.success() {
        let error_message = String::from_utf8_lossy(&install_output.stderr);

        if !error_message.contains("did not reply to the call")
            && !error_message.contains("function invocation does not match its signature")
        {
            panic!(
                "Error: {:?}",
                String::from_utf8_lossy(&install_output.stderr)
            );
        }
    }
}

fn upload_chunk(
    name: &str,
    canister_name: &str,
    bytecode_chunk: &Vec<u8>,
    canister_method_name: &str,
    dfx_network: &str,
    chunk_number: usize,
    chunk_total: usize,
) {
    let blob_string = vec_u8_to_blob_string(bytecode_chunk);

    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    temp_file
        .write_all(blob_string.as_bytes())
        .expect("Failed to write data to temporary file");

    let output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg("--network")
        .arg(dfx_network)
        .arg(canister_name)
        .arg(canister_method_name)
        .arg("--argument-file")
        .arg(temp_file.path())
        .output()
        .expect("Failed to execute the dfx command");

    let chunk_number = chunk_number + 1;

    if output.status.success() {
        println!(
            "{}",
            format!("Uploading {name} chunk {chunk_number}/{chunk_total}")
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
