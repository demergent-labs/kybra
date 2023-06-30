use sha2::{Digest, Sha256};

use crate::{
    dfx::dfx,
    error::create_error_string,
    upload_chunk::{split_into_chunks, upload_chunk},
};

#[cfg(ic_build)]
use crate::error::error_to_string;

pub fn upload_python_stdlib(canister_name: &str, max_chunk_size: usize) -> Result<(), String> {
    let python_stdlib_bytecode = get_python_stdlib_bytecode()?;

    let local_python_stdlib_bytecode_hash =
        get_local_python_stdlib_bytecode_hash(&python_stdlib_bytecode);

    let remote_python_stdlib_bytecode_hash = get_remote_python_stdlib_bytecode_hash(canister_name)?;

    let hashes_do_not_match =
        format!("(\"{local_python_stdlib_bytecode_hash}\")") != remote_python_stdlib_bytecode_hash;

    if hashes_do_not_match {
        handle_python_stdlib_hashes_do_not_match(
            canister_name,
            python_stdlib_bytecode,
            max_chunk_size,
        )?;
    }

    Ok(())
}

fn get_local_python_stdlib_bytecode_hash(python_stdlib_bytecode: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(python_stdlib_bytecode);
    let result = hasher.finalize();
    hex::encode(result)
}

fn get_remote_python_stdlib_bytecode_hash(canister_name: &str) -> Result<String, String> {
    let remote_python_stdlib_hash_output = dfx(
        "canister",
        "call",
        &vec![canister_name, "get_python_stdlib_hash"],
    )?;

    if !remote_python_stdlib_hash_output.status.success() {
        return Err(create_error_string(&String::from_utf8_lossy(
            &remote_python_stdlib_hash_output.stderr,
        )));
    }

    let remote_python_stdlib_bytecode_hash =
        String::from_utf8_lossy(&remote_python_stdlib_hash_output.stdout)
            .trim()
            .to_string();

    Ok(remote_python_stdlib_bytecode_hash)
}

fn handle_python_stdlib_hashes_do_not_match(
    canister_name: &str,
    python_stdlib_bytecode: Vec<u8>,
    max_chunk_size: usize,
) -> Result<(), String> {
    let python_stdlib_bytecode_chunks = split_into_chunks(python_stdlib_bytecode, max_chunk_size);

    for (index, python_stdlib_bytecode_chunk) in python_stdlib_bytecode_chunks.iter().enumerate() {
        upload_chunk(
            "Python stdlib",
            canister_name,
            python_stdlib_bytecode_chunk,
            "upload_python_stdlib_chunk",
            index,
            python_stdlib_bytecode_chunks.len(),
        )?;
    }

    Ok(())
}

fn get_python_stdlib_bytecode() -> Result<Vec<u8>, String> {
    #[cfg(ic_build)]
    let kybra_version = std::env::var("KYBRA_VERSION")
        .map_err(|_| create_error_string("KYBRA_VERSION environment variable not present"))?;

    #[cfg(ic_build)]
    let python_stdlib_path = dirs::home_dir()
        .ok_or(create_error_string("Home directory not found"))?
        .join(format!(".config/kybra/{kybra_version}/bin/python_stdlib"));

    #[cfg(all(not(python_stdlib_exists), ic_build))]
    let python_stdlib_modules = rustpython_vm::py_freeze!(dir = "src/Lib");

    #[cfg(all(not(python_stdlib_exists), ic_build))]
    let python_stdlib_bytecode = python_stdlib_modules.bytes.to_vec();

    #[cfg(all(not(python_stdlib_exists), ic_build))]
    std::fs::write(python_stdlib_path, &python_stdlib_bytecode).map_err(|e| error_to_string(&e))?;

    #[cfg(all(python_stdlib_exists, ic_build))]
    let python_stdlib_bytecode =
        std::fs::read(python_stdlib_path).map_err(|e| error_to_string(&e))?;

    #[cfg(not(ic_build))]
    let python_stdlib_bytecode = vec![];

    Ok(python_stdlib_bytecode)
}
