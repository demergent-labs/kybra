use clear_chunks::clear_chunks;
use error::create_error_string;
use generate_candid::generate_candid;
use install_app_canister::install_app_canister;
use upload_app_canister::upload_app_canister;
use upload_python_stdlib::upload_python_stdlib;

mod clear_chunks;
mod dfx;
mod error;
mod generate_candid;
mod install_app_canister;
mod permissions;
mod upload_app_canister;
mod upload_chunk;
mod upload_python_stdlib;

pub static ERROR_PREFIX: &str = "Kybra Post Install Error";

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let canister_name = args
        .get(1)
        .ok_or(create_error_string("Canister name argument not present"))?;
    let candid_path = args
        .get(2)
        .ok_or(create_error_string("Candid path argument not present"))?;
    let max_chunk_size = 2 * 1_000 * 1_000; // 2 MB message size limit currently on the Internet Computer

    clear_chunks(canister_name)?;
    upload_app_canister(canister_name, max_chunk_size)?;
    upload_python_stdlib(canister_name, max_chunk_size)?;
    install_app_canister(canister_name)?;
    generate_candid(canister_name, candid_path)?;

    Ok(())
}
