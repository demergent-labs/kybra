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
    shutil.copytree(
        f"{paths['global_kybra_version_dir']}/RustPython/Lib",
        f"{paths['canister']}/Lib",
    )
    compile_generated_rust_code(paths, canister_name, cargo_env, verbose)
    copy_wasm_to_dev_location(paths, canister_name)
    run_wasi2ic_on_wasm(paths, canister_name, cargo_env, verbose)
    generate_and_create_candid_file(paths, canister_name, cargo_env, verbose)


def compile_generated_rust_code(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool
):
    python_stdlib_is_installed = check_if_python_stdlib_installed(
        canister_name, cargo_env, verbose
    )
    features = get_cargo_build_features(python_stdlib_is_installed)
    run_cargo_build(paths, canister_name, features, cargo_env, verbose)


def check_if_python_stdlib_installed(
    canister_name: str, cargo_env: dict[str, str], verbose: bool
) -> bool:
    check_if_python_stdlib_installed_result = run_subprocess(
        [
            "dfx",
            "canister",
            "--network",
            str(os.environ.get("DFX_NETWORK")),
            "call",
            canister_name,
            "_kybra_check_if_python_stdlib_installed",
        ],
        cargo_env,
        False,  # Passing verbose along as True messes with the std outputs
        False,
    )

    check_if_python_stdlib_installed = (
        check_if_python_stdlib_installed_result.decode().strip()
    )

    if verbose == True:
        print(check_if_python_stdlib_installed)

    return check_if_python_stdlib_installed == "(true)"


def get_cargo_build_features(python_stdlib_is_installed: bool) -> str:
    return "" if python_stdlib_is_installed else "--features=azle_include_stdlib"


def run_cargo_build(
    paths: Paths,
    canister_name: str,
    features: str,
    cargo_env: dict[str, str],
    verbose: bool,
):
    run_subprocess(
        [
            f"{paths['global_kybra_rust_bin_dir']}/cargo",
            "build",
            f"--manifest-path={paths['canister']}/Cargo.toml",
            "--target=wasm32-wasi",
            f"--package={canister_name}",
            "--release",
            *([features] if features != "" else []),
        ],
        cargo_env,
        verbose,
    )


def copy_wasm_to_dev_location(paths: Paths, canister_name: str):
    copy_file(
        f"{paths['global_kybra_target_dir']}/wasm32-wasi/release/{canister_name}.wasm",
        f"{paths['canister']}/{canister_name}.wasm",
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
        cargo_env,
        verbose,
    )


def generate_and_create_candid_file(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool
):
    candid_bytes = run_subprocess(
        [
            f"{paths['global_kybra_rust_bin_dir']}/candid-extractor",
            f"{paths['canister']}/{canister_name}.wasm",
        ],
        cargo_env,
        False,  # Passing verbose along as True messes with the std outputs
    )

    candid_string = candid_bytes.decode()

    if verbose == True:
        print(candid_string)

    create_file(paths["did"], candid_string)


def run_subprocess(
    args: list[str], env: dict[str, str], verbose: bool, throw: bool = True
) -> bytes:
    result = subprocess.run(args, env=env, capture_output=not verbose)

    if result.returncode != 0:
        if throw == True:
            print_error_and_exit(result)
        else:
            return result.stderr

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
