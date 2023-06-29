use crate::{dfx::dfx, error::create_error_string};

pub fn add_permissions(canister_name: &str) -> Result<(String, bool), String> {
    let canister_id = get_canister_id(canister_name)?;
    let canister_already_its_own_controller =
        get_canister_already_its_own_controller(canister_name, &canister_id)?;

    if !canister_already_its_own_controller {
        add_controller(canister_name, &canister_id)?;
    }

    Ok((canister_id, canister_already_its_own_controller))
}

fn get_canister_id(canister_name: &str) -> Result<String, String> {
    let canister_id_output = dfx("canister", "id", &vec![canister_name])?;

    let canister_id = String::from_utf8_lossy(&canister_id_output.stdout)
        .trim()
        .to_string();

    Ok(canister_id)
}

fn get_canister_already_its_own_controller(
    canister_name: &str,
    canister_id: &str,
) -> Result<bool, String> {
    let current_controllers_output = dfx("canister", "info", &vec![canister_name])?;

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

fn add_controller(canister_name: &str, canister_id: &str) -> Result<(), String> {
    let add_controller_output = dfx(
        "canister",
        "update-settings",
        &vec!["--add-controller", canister_id, canister_name],
    )?;

    if !add_controller_output.status.success() {
        return Err(create_error_string(&String::from_utf8_lossy(
            &add_controller_output.stderr,
        )));
    }

    Ok(())
}

pub fn remove_permissions(
    canister_name: &str,
    canister_id: &str,
    canister_already_its_own_controller: bool,
) -> Result<(), String> {
    if canister_already_its_own_controller {
        return Ok(());
    }

    let remove_controller_output = dfx(
        "canister",
        "update-settings",
        &vec!["--remove-controller", canister_id, canister_name],
    )?;

    if !remove_controller_output.status.success() {
        return Err(create_error_string(&String::from_utf8_lossy(
            &remove_controller_output.stderr,
        )));
    }

    Ok(())
}
