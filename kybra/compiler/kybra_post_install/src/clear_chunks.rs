use crate::{dfx::dfx, error::create_error_string};

pub fn clear_chunks(canister_name: &str) -> Result<(), String> {
    let output = dfx("canister", "call", &vec![canister_name, "clear_chunks"])?;

    if !output.status.success() {
        return Err(create_error_string(&String::from_utf8_lossy(
            &output.stderr,
        )));
    }

    Ok(())
}
