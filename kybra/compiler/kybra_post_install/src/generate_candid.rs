use crate::{
    dfx::dfx,
    error::{create_error_string, error_to_string},
};

pub fn generate_candid(canister_name: &str, candid_path: &str) -> Result<(), String> {
    let candid_output = dfx("canister", "call", &vec![canister_name, "export_candid"])?;

    if !candid_output.status.success() {
        return Err(create_error_string(&String::from_utf8_lossy(
            &candid_output.stderr,
        )));
    }

    // let candid = String::from_utf8_lossy(&candid_output.stdout)
    //     .trim()
    //     .trim_start_matches("(")
    //     .trim_start_matches("\"")
    //     .trim_end_matches(",)")
    //     .trim_end_matches("\"")
    //     .to_string();

    let mut candid = String::from_utf8_lossy(&candid_output.stdout).to_string();

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

    println!("{}", candid.replace("\\n", "\n"));
    println!("{}", candid.replace("\\\"", "\""));

    std::fs::write(
        candid_path,
        candid.replace("\\n", "\n").replace("\\\"", "\""),
    )
    .map_err(|e| error_to_string(&e))?;

    Ok(())
}
