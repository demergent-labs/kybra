import os
import shutil
import subprocess
import sys

from kybra.colors import red
from kybra.timed import timed_inline
from kybra.types import Paths


@timed_inline
def build_wasm_binary_or_exit(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool = False
):
    compile_generated_rust_code(paths, canister_name, cargo_env, verbose)
    copy_wasm_to_dev_location(paths, canister_name)
    run_wasi2ic_on_wasm(paths, canister_name, cargo_env, verbose)
    generate_and_create_candid_file(paths, canister_name, cargo_env, verbose)


def compile_generated_rust_code(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool
):
    # TODO We could maybe do this in a nicer way by using HTTP
    # TODO and calling into the state tree to get the module hash
    # TODO Or perhaps dfx could create an environment variable like
    # TODO DFX_INSTALL_MODE
    # TODO You can look into how install mode is done in dfx
    # TODO here: https://github.com/dfinity/sdk/blob/master/src/dfx/src/lib/operations/canister/install_canister.rs#L67-L75
    canister_status_result = subprocess.run(
        [
            "dfx",
            "canister",
            "--network",
            str(os.environ.get("DFX_NETWORK")),
            "status",
            canister_name,
        ],
        capture_output=True,
    )

    canister_status_string = canister_status_result.stderr.decode()

    # TODO change back to in
    azle_include_stdlib_feature = (
        "--features=azle_include_stdlib"
        if "Module hash: None" in canister_status_string
        else ""
    )

    run_subprocess(
        [
            f"{paths['global_kybra_rust_bin_dir']}/cargo",
            "build",
            f"--manifest-path={paths['canister']}/Cargo.toml",
            "--target=wasm32-wasi",
            f"--package={canister_name}",
            "--release",
            *([azle_include_stdlib_feature] if azle_include_stdlib_feature else []),
        ],
        verbose,
        cargo_env,
    )


def copy_wasm_to_dev_location(paths: Paths, canister_name: str):
    copy_file(
        f"{paths['global_kybra_target_dir']}/wasm32-wasi/release/{canister_name}.wasm",
        f"{paths['canister']}/{canister_name}.wasm",
    )


def generate_and_create_candid_file(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool
):
    # TODO do we want to do this in a better/fancier way with verbose etc?
    subprocess.run(
        [
            f"{paths['global_kybra_rust_bin_dir']}/candid-extractor {paths['canister']}/{canister_name}.wasm > {paths['did']}"
        ],
        shell=True,
        check=True,
    )


def run_wasi2ic_on_wasm(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool
):
    run_subprocess(
        [
            f"{paths['global_kybra_rust_bin_dir']}/wasi2ic",
            f"{paths['canister']}/{canister_name}.wasm",
            f"{paths['canister']}/{canister_name}.wasm",
        ],
        verbose,
        cargo_env,
    )


def run_subprocess(args: list[str], verbose: bool, env: dict[str, str]) -> bytes:
    result = subprocess.run(args, capture_output=not verbose, env=env)
    if result.returncode != 0:
        print_error_and_exit(result)
    return result.stdout


def copy_file(source: str, destination: str):
    shutil.copy(source, destination)


def create_file(path: str, content: str):
    with open(path, "w") as f:
        f.write(content)


def print_error_and_exit(result: subprocess.CompletedProcess[bytes]):
    print(red("\n💣 Kybra error: building Wasm binary"))
    print(result.stderr.decode("utf-8"))
    print("💀 Build failed")
    sys.exit(1)
