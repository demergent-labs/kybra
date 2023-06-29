use std::process::Command;

use crate::error::create_error_string;

pub fn dfx(
    command: &str,
    subcommand: &str,
    args: &Vec<&str>,
) -> Result<std::process::Output, String> {
    let dfx_network = std::env::var("DFX_NETWORK")
        .map_err(|_| create_error_string("DFX_NETWORK environment variable not present"))?;

    let mut dfx_command = Command::new("dfx");
    dfx_command.arg(command);
    dfx_command.arg(subcommand);
    dfx_command.arg("--network");
    dfx_command.arg(dfx_network);

    for arg in args {
        dfx_command.arg(arg);
    }

    dfx_command.output().map_err(|e| e.to_string())
}
