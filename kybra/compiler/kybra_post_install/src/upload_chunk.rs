use std::io::Write;
use tempfile::NamedTempFile;

use crate::{dfx::dfx, error::create_error_string};

pub fn upload_chunk(
    name: &str,
    canister_name: &str,
    bytecode_chunk: &Vec<u8>,
    canister_method_name: &str,
    chunk_number: usize,
    chunk_total: usize,
) -> Result<(), String> {
    let blob_string = vec_u8_to_blob_string(bytecode_chunk);

    let mut temp_file =
        NamedTempFile::new().map_err(|_| create_error_string("Failed to create temporary file"))?;
    temp_file
        .write_all(blob_string.as_bytes())
        .map_err(|_| create_error_string("Failed to write data to temporary file"))?;

    let output = dfx(
        "canister",
        "call",
        &vec![
            canister_name,
            canister_method_name,
            "--argument-file",
            temp_file.path().to_str().ok_or(create_error_string(
                "temp_file path could not be converted to &str",
            ))?,
        ],
    )?;

    let chunk_number = chunk_number + 1;

    if output.status.success() {
        println!(
            "{}",
            format!("Uploading {name} chunk {chunk_number}/{chunk_total}")
        );
    } else {
        return Err(create_error_string(&String::from_utf8_lossy(
            &output.stderr,
        )));
    }

    Ok(())
}

pub fn split_into_chunks(data: Vec<u8>, chunk_size: usize) -> Vec<Vec<u8>> {
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
