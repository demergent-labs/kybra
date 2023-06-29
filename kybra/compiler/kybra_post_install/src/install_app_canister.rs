use crate::{dfx::dfx, error::create_error_string};

pub fn install_app_canister(canister_name: &str) -> Result<(), String> {
    println!("\nFinalizing...\n");

    let install_output = dfx("canister", "call", &vec![canister_name, "install_wasm"])?;

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
