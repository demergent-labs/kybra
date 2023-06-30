use crate::{
    dfx::dfx,
    error::create_error_string,
    permissions::{add_permissions, remove_permissions},
};

pub fn install_app_canister(canister_name: &str) -> Result<(), String> {
    println!("\nFinalizing...\n");

    let (canister_id, canister_already_its_own_controller_before_post_install) =
        add_permissions(canister_name)?;

    let install_output = dfx("canister", "call", &vec![canister_name, "install_wasm"])?;

    remove_permissions(
        canister_name,
        &canister_id,
        canister_already_its_own_controller_before_post_install,
    )?;

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
