// TODO share with DFINITY how I'm doing this, maybe they can build that into dfx to overcome some of the Wasm binary limitations temporarily
// TODO a future version of dfx should take care of all of this chunk uploading for us

use sha2::{Digest, Sha256};
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tempfile::NamedTempFile;

static ERROR_PREFIX: &str = "Kybra Post Install Error";

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let canister_name = args
        .get(1)
        .ok_or(create_error_string("Canister name argument not present"))?;
    let max_chunk_size = 2 * 1_000 * 1_000; // 2 MB
    let dfx_network = std::env::var("DFX_NETWORK")
        .map_err(|_| create_error_string("DFX_NETWORK environment variable not present"))?;

    upload_app_canister(canister_name, max_chunk_size, &dfx_network)?;
    upload_python_stdlib(canister_name, max_chunk_size, &dfx_network)?;
    let (canister_id, canister_already_its_own_controller) =
        add_permissions(canister_name, &dfx_network)?;
    install_app_canister(canister_name, &dfx_network)?;
    remove_permissions(
        canister_name,
        &dfx_network,
        &canister_id,
        canister_already_its_own_controller,
    )?;

    // TODO this is here because of some complications with the install_code self-referential cross-canister call
    // TODO the call is a notify and thus won't wait for the canister's post_upgrade function to complete
    // TODO we wait here to make sure that the canister is most likely initialized before ending the post_install script
    thread::sleep(Duration::from_secs(5));

    Ok(())
}

fn upload_app_canister(
    canister_name: &str,
    max_chunk_size: usize,
    dfx_network: &str,
) -> Result<(), String> {
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
            dfx_network,
            index,
            wasm_chunks.len(),
        )?;
    }

    Ok(())
}

fn upload_python_stdlib(
    canister_name: &str,
    max_chunk_size: usize,
    dfx_network: &str,
) -> Result<(), String> {
    let python_stdlib_bytecode = get_python_stdlib_bytecode()?;

    let local_python_stdlib_bytecode_hash =
        get_local_python_stdlib_bytecode_hash(&python_stdlib_bytecode);

    let remote_python_stdlib_bytecode_hash =
        get_remote_python_stdlib_bytecode_hash(canister_name, dfx_network)?;

    let hashes_do_not_match =
        format!("(\"{local_python_stdlib_bytecode_hash}\")") != remote_python_stdlib_bytecode_hash;

    if hashes_do_not_match {
        handle_python_stdlib_hashes_do_not_match(
            canister_name,
            dfx_network,
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

fn get_remote_python_stdlib_bytecode_hash(
    canister_name: &str,
    dfx_network: &str,
) -> Result<String, String> {
    let remote_python_stdlib_hash_output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg("--network")
        .arg(dfx_network)
        .arg(canister_name)
        .arg("python_stdlib_hash")
        .output()
        .map_err(|_| create_error_string("Failed to execute the dfx canister call command"))?;

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
    dfx_network: &str,
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
            dfx_network,
            index,
            python_stdlib_bytecode_chunks.len(),
        )?;
    }

    Ok(())
}

fn get_python_stdlib_bytecode() -> Result<Vec<u8>, String> {
    let kybra_version = std::env::var("KYBRA_VERSION")
        .map_err(|_| create_error_string("KYBRA_VERSION environment variable not present"))?;

    let python_stdlib_path = dirs::home_dir()
        .ok_or(create_error_string("Home directory not found"))?
        .join(format!(".config/kybra/{kybra_version}/bin/python_stdlib"));

    #[cfg(not(python_stdlib_exists))]
    let python_stdlib_modules = rustpython_vm::py_freeze!(dir = "src/Lib");

    #[cfg(not(python_stdlib_exists))]
    let python_stdlib_bytecode = python_stdlib_modules.bytes.to_vec();

    #[cfg(not(python_stdlib_exists))]
    std::fs::write(python_stdlib_path, &python_stdlib_bytecode).map_err(|e| error_to_string(&e))?;

    #[cfg(python_stdlib_exists)]
    let python_stdlib_bytecode =
        std::fs::read(python_stdlib_path).map_err(|e| error_to_string(&e))?;

    Ok(python_stdlib_bytecode)
}

fn add_permissions(canister_name: &str, dfx_network: &str) -> Result<(String, bool), String> {
    let canister_id = get_canister_id(canister_name, dfx_network)?;
    let canister_already_its_own_controller =
        get_canister_already_its_own_controller(canister_name, dfx_network, &canister_id)?;

    if !canister_already_its_own_controller {
        add_controller(canister_name, dfx_network, &canister_id)?;
    }

    Ok((canister_id, canister_already_its_own_controller))
}

fn get_canister_id(canister_name: &str, dfx_network: &str) -> Result<String, String> {
    let canister_id_output = Command::new("dfx")
        .arg("canister")
        .arg("id")
        .arg("--network")
        .arg(dfx_network)
        .arg(canister_name)
        .output()
        .map_err(|_| create_error_string("Failed to execute the dfx canister id command"))?;

    let canister_id = String::from_utf8_lossy(&canister_id_output.stdout)
        .trim()
        .to_string();

    Ok(canister_id)
}

fn get_canister_already_its_own_controller(
    canister_name: &str,
    dfx_network: &str,
    canister_id: &str,
) -> Result<bool, String> {
    let current_controllers_output = Command::new("dfx")
        .arg("canister")
        .arg("info")
        .arg("--network")
        .arg(dfx_network)
        .arg(canister_name)
        .output()
        .map_err(|_| create_error_string("Failed to execute the dfx canister info command"))?;

    if !current_controllers_output.status.success() {
        return Err(create_error_string(&String::from_utf8_lossy(
            &current_controllers_output.stderr,
        )));
    }

    let canister_already_its_own_controller =
        String::from_utf8_lossy(&current_controllers_output.stdout)
            .to_string()
            .contains(&canister_id);

    Ok(canister_already_its_own_controller)
}

fn add_controller(canister_name: &str, dfx_network: &str, canister_id: &str) -> Result<(), String> {
    let add_controller_output = Command::new("dfx")
        .arg("canister")
        .arg("update-settings")
        .arg("--network")
        .arg(dfx_network)
        .arg("--add-controller")
        .arg(&canister_id)
        .arg(canister_name)
        .output()
        .map_err(|_| create_error_string("Failed to execute the dfx command"))?;

    if !add_controller_output.status.success() {
        return Err(create_error_string(&String::from_utf8_lossy(
            &add_controller_output.stderr,
        )));
    }

    Ok(())
}

fn install_app_canister(canister_name: &str, dfx_network: &str) -> Result<(), String> {
    println!("\nFinalizing...\n");

    let install_output = Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg("--network")
        .arg(dfx_network)
        .arg(canister_name)
        .arg("install_wasm")
        .output()
        .map_err(|_| create_error_string("Failed to execute the dfx command"))?;

    if !install_output.status.success() {
        handle_install_app_canister_failure(&install_output)?;
    }

    Ok(())
}

fn handle_install_app_canister_failure(
    install_output: &std::process::Output,
) -> Result<(), String> {
    let error_message = String::from_utf8_lossy(&install_output.stderr);

    if !error_message.contains("did not reply to the call")
        && !error_message.contains("function invocation does not match its signature")
    {
        return Err(create_error_string(&String::from_utf8_lossy(
            &install_output.stderr,
        )));
    }

    Ok(())
}

fn remove_permissions(
    canister_name: &str,
    dfx_network: &str,
    canister_id: &str,
    canister_already_its_own_controller: bool,
) -> Result<(), String> {
    if canister_already_its_own_controller {
        return Ok(());
    }

    let remove_controller_output = Command::new("dfx")
        .arg("canister")
        .arg("update-settings")
        .arg("--network")
        .arg(dfx_network)
        .arg("--remove-controller")
        .arg(canister_id)
        .arg(canister_name)
        .output()
        .map_err(|_| create_error_string("Failed to execute the dfx command"))?;

    if !remove_controller_output.status.success() {
        return Err(create_error_string(&String::from_utf8_lossy(
            &remove_controller_output.stderr,
        )));
    }

    Ok(())
}

fn upload_chunk(
    name: &str,
    canister_name: &str,
    bytecode_chunk: &Vec<u8>,
    canister_method_name: &str,
    dfx_network: &str,
    chunk_number: usize,
    chunk_total: usize,
) -> Result<(), String> {
    let blob_string = vec_u8_to_blob_string(bytecode_chunk);

    let mut temp_file =
        NamedTempFile::new().map_err(|_| create_error_string("Failed to create temporary file"))?;
    temp_file
        .write_all(blob_string.as_bytes())
        .map_err(|_| create_error_string("Failed to write data to temporary file"))?;

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
        .map_err(|_| create_error_string("Failed to execute the dfx command"))?;

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

fn error_to_string(e: &dyn std::error::Error) -> String {
    format!("{ERROR_PREFIX}: {}", e.to_string())
}

fn create_error_string(message: &str) -> String {
    format!("{ERROR_PREFIX}: {message}")
}
