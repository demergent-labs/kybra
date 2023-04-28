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

    // TODO we still need to upload the python stdlib
    let python_stdlib_modules = rustpython_pylib::FROZEN_STDLIB;
    let python_stdlib_bytecode = python_stdlib_modules.bytes.to_vec();
    let python_stdlib_bytecode_chunks = split_into_chunks(python_stdlib_bytecode, max_chunk_size);

    let python_source_bytecode_chunk = &python_source_bytecode_chunks[0];
    let blob_string = vec_u8_to_blob_string(python_source_bytecode_chunk);

    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");
    temp_file
        .write_all(blob_string.as_bytes())
        .expect("Failed to write data to temporary file");

    let output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg(canister_name)
        .arg("create_python_source_module")
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
        // if byte.is_ascii_graphic() && byte != b'\\' {
        //     result.push(byte as char);
        // } else {
        result.push_str(&format!("\\{:02X}", byte));
        // }
    }
    result.push_str("\")");
    result
}

// fn vec_u8_to_blob_string(data: &[u8]) -> String {
//     let hex_encoded = data.iter().map(|byte| format!("{:02X}", byte)).collect::<String>();
//     format!("(blob \"{}\")", hex_encoded)
// }
