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

    handle_install_app_canister_failure(&install_output, canister_name)?;

    Ok(())
}

fn handle_install_app_canister_failure(
    install_output: &std::process::Output,
    canister_name: &str,
) -> Result<(), String> {
    let stdout_message = String::from_utf8_lossy(&install_output.stdout);

    if stdout_message.to_lowercase().contains("error") {
        return Err(create_error_string(&stdout_message));
    }

    let stderr_message = String::from_utf8_lossy(&install_output.stderr);

    if install_output.status.success() == false
        && !stderr_message.contains("did not reply to the call")
        && !stderr_message.contains("function invocation does not match its signature")
    {
        return Err(create_error_string(&stderr_message));
    }

    check_for_interpreter(canister_name)?;

    Ok(())
}

fn check_for_interpreter(canister_name: &str) -> Result<(), String> {
    let err_message = "The RustPython interpreter could not be initialized. init or post_upgrade has most likely trapped. Check your local replica logs if available".to_string();

    let does_interpreter_exist_output = dfx(
        "canister",
        "call",
        &vec![canister_name, "does_interpreter_exist"],
    )?;

    if !does_interpreter_exist_output.status.success() {
        println!(
            "{}",
            String::from_utf8_lossy(&does_interpreter_exist_output.stderr,)
        );

        return Err(err_message);
    }

    let does_interpreter_exist = String::from_utf8_lossy(&does_interpreter_exist_output.stdout)
        .to_string()
        .trim()
        == "(true)";

    if does_interpreter_exist == false {
        return Err(err_message);
    }

    Ok(())
}
