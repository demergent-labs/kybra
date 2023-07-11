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

    let raw_candid = String::from_utf8_lossy(&candid_output.stdout).to_string();
    let escaped_candid = raw_candid.replace("\\n", "\n").replace("\\\"", "\"");

    let left_index = escaped_candid
        .find("\"")
        .ok_or("Generate Candid Error: could not find starting quote")?
        + 1;
    let right_index = escaped_candid
        .rfind("\"")
        .ok_or("Generate Candid Error: could not find ending quote")?;

    let clean_candid = format!(
        "{}\n",
        escaped_candid[left_index..right_index].to_string().trim()
    );

    std::fs::write(candid_path, clean_candid).map_err(|e| error_to_string(&e))?;

    Ok(())
}
