use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

// TODO share with DFINITY how I'm doing this, maybe they can build that into dfx to overcome some of the Wasm binary limitations temporarily
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let canister_name = &args[1];

    let max_chunk_size = 2 * 1_000 * 1_000; // 2 MB

    match std::env::current_dir() {
        Ok(current_dir) => println!("Current working directory: {}", current_dir.display()),
        Err(e) => println!("Error getting current directory: {}", e),
    };

    let wasm = std::fs::read(format!(".kybra/{canister_name}/{canister_name}_app.wasm")).unwrap();
    let wasm_chunks = split_into_chunks(wasm, max_chunk_size);

    for wasm_chunk in wasm_chunks {
        upload_chunk(canister_name, wasm_chunk, "upload_wasm_chunk");
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
            "Output: {:?}",
            String::from_utf8_lossy(&add_controller_output.stdout)
        );
    } else {
        eprintln!(
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
            "Output: {:?}",
            String::from_utf8_lossy(&install_output.stdout)
        );
    } else {
        eprintln!(
            "Error: {:?}",
            String::from_utf8_lossy(&install_output.stderr)
        );
    }

    let python_source_modules = rustpython_vm::py_freeze!(dir = "../python_source");
    let python_source_bytecode = python_source_modules.bytes.to_vec();
    let python_source_bytecode_chunks = split_into_chunks(python_source_bytecode, max_chunk_size);

    let python_stdlib_modules = rustpython_vm::py_freeze!(dir = "src/Lib");
    let python_stdlib_bytecode = python_stdlib_modules.bytes.to_vec();
    let python_stdlib_bytecode_chunks = split_into_chunks(python_stdlib_bytecode, max_chunk_size);

    let clear_output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg(canister_name)
        .arg("clear_python_bytecode_chunks")
        .output()
        .expect("Failed to execute the dfx command");

    if clear_output.status.success() {
        println!(
            "Output: {:?}",
            String::from_utf8_lossy(&clear_output.stdout)
        );
    } else {
        eprintln!("Error: {:?}", String::from_utf8_lossy(&clear_output.stderr));
    }

    // Upload Python source bytecode
    for python_source_bytecode_chunk in python_source_bytecode_chunks {
        upload_chunk(
            canister_name,
            python_source_bytecode_chunk,
            "upload_python_source_bytecode_chunk",
        );
    }

    // Upload Python stdlib bytecode
    for python_stdlib_bytecode_chunk in python_stdlib_bytecode_chunks {
        upload_chunk(
            canister_name,
            python_stdlib_bytecode_chunk,
            "upload_python_stdlib_bytecode_chunk",
        );
    }

    let initialize_output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg(canister_name)
        .arg("initialize_python_modules")
        .output()
        .expect("Failed to execute the dfx command");

    if initialize_output.status.success() {
        println!(
            "Output: {:?}",
            String::from_utf8_lossy(&initialize_output.stdout)
        );
    } else {
        eprintln!(
            "Error: {:?}",
            String::from_utf8_lossy(&initialize_output.stderr)
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
        println!("Output: {:?}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Error: {:?}", String::from_utf8_lossy(&output.stderr));
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
