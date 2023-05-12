use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let canister_name = &args[1];

    let max_chunk_size = 2 * 1_000 * 1_000; // 2 MB

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
        upload_python_bytecode_chunk(
            canister_name,
            python_source_bytecode_chunk,
            "upload_python_source_bytecode_chunk",
        );
    }

    // Upload Python stdlib bytecode
    for python_stdlib_bytecode_chunk in python_stdlib_bytecode_chunks {
        upload_python_bytecode_chunk(
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

fn upload_python_bytecode_chunk(
    canister_name: &str,
    bytecode_chunk: Vec<u8>,
    canister_method_name: &str,
) {
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
