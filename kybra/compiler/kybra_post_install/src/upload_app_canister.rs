use crate::{
    error::error_to_string,
    upload_chunk::{split_into_chunks, upload_chunk},
};

pub fn upload_app_canister(canister_name: &str, max_chunk_size: usize) -> Result<(), String> {
    let wasm = std::fs::read(format!(
        ".kybra/{canister_name}/{canister_name}_app.wasm.gz"
    ))
    .map_err(|e| error_to_string(&e))?;

    let wasm_chunks = split_into_chunks(wasm, max_chunk_size);

    for (index, wasm_chunk) in wasm_chunks.iter().enumerate() {
        upload_chunk(
            &format!("{canister_name} wasm"),
            canister_name,
            wasm_chunk,
            "upload_wasm_chunk",
            index,
            wasm_chunks.len(),
        )?;
    }

    Ok(())
}
