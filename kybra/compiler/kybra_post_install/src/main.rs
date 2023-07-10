// TODO share with DFINITY how I'm doing this, maybe they can build that into dfx to overcome some of the Wasm binary limitations temporarily
// TODO a future version of dfx should take care of all of this chunk uploading for us
use std::{thread, time::Duration};

use error::create_error_string;
use generate_candid::generate_candid;
use install_app_canister::install_app_canister;
use upload_app_canister::upload_app_canister;
use upload_python_stdlib::upload_python_stdlib;

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

    upload_app_canister(canister_name, max_chunk_size)?;
    upload_python_stdlib(canister_name, max_chunk_size)?;
    install_app_canister(canister_name)?;
    generate_candid(canister_name, candid_path)?;

    // TODO this is here because of some complications with the install_code self-referential cross-canister call
    // TODO the call is a notify and thus won't wait for the canister's post_upgrade function to complete
    // TODO we wait here to make sure that the canister is most likely initialized before ending the post_install script
    thread::sleep(Duration::from_secs(5));

    Ok(())
}
