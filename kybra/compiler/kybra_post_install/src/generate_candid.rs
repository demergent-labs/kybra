use crate::{
    dfx::dfx,
    error::{create_error_string, error_to_string},
};

pub fn generate_candid(canister_name: &str, candid_path: &str) -> Result<(), String> {
    let candid_output = dfx(
        "canister",
        "call",
        &vec![canister_name, "__get_candid_interface_tmp_hack"],
    )?;

    if !candid_output.status.success() {
        return Err(create_error_string(&String::from_utf8_lossy(
            &candid_output.stderr,
        )));
    }

    let mut candid = String::from_utf8_lossy(&candid_output.stdout)
        .to_string()
        .replace("\\n", "\n")
        .replace("\\\"", "\"");

    if let Some(index) = candid.find("(") {
        candid.drain(index..index + 1);
    }

    if let Some(index) = candid.find("\"") {
        candid.drain(index..index + 1);
    }

    if let Some(index) = candid.rfind(")") {
        candid.drain(index..index + 1);
    }

    if let Some(index) = candid.rfind(",") {
        candid.drain(index..index + 1);
    }

    if let Some(index) = candid.rfind("\"") {
        candid.drain(index..index + 1);
    }

    let candid_with_newline = format!("{}\n", candid.trim());

    std::fs::write(candid_path, candid_with_newline).map_err(|e| error_to_string(&e))?;

    Ok(())
}
