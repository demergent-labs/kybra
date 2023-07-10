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
    generate_and_create_candid_file(paths, canister_name)
    run_wasi2ic_on_app_wasm(paths, canister_name, cargo_env, verbose)
    handle_deployer_wasm(paths, canister_name, cargo_env, verbose)


def compile_generated_rust_code(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool
):
    run_subprocess(
        [
            f"{paths['global_kybra_rust_bin_dir']}/cargo",
            "build",
            f"--manifest-path={paths['canister']}/Cargo.toml",
            "--target=wasm32-wasi",
            f"--package={canister_name}",
            "--release",
        ],
        verbose,
        cargo_env,
    )


def copy_wasm_to_dev_location(paths: Paths, canister_name: str):
    copy_file(
        f"{paths['global_kybra_target_dir']}/wasm32-wasi/release/{canister_name}.wasm",
        f"{paths['canister']}/{canister_name}_app.wasm",
    )


def generate_and_create_candid_file(paths: Paths, canister_name: str):
    if not os.path.isfile(paths["did"]):
        create_file(paths["did"], "service : () -> {}")


def run_wasi2ic_on_app_wasm(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool
):
    run_subprocess(
        [
            f"{paths['global_kybra_rust_bin_dir']}/wasi2ic",
            f"{paths['canister']}/{canister_name}_app.wasm",
            f"{paths['canister']}/{canister_name}_app.wasm",
        ],
        verbose,
        cargo_env,
    )


def handle_deployer_wasm(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool
):
    should_rebuild = get_should_rebuild(paths)
    if should_rebuild:
        copy_existing_deployer_wasm(paths, canister_name)
    else:
        build_and_copy_new_deployer_wasm(paths, canister_name, cargo_env, verbose)


def copy_existing_deployer_wasm(paths: Paths, canister_name: str):
    copy_file(
        f"{paths['global_kybra_bin_dir']}/deployer.wasm",
        f"{paths['canister']}/{canister_name}.wasm",
    )


def build_and_copy_new_deployer_wasm(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool
):
    run_subprocess(
        [
            f"{paths['global_kybra_rust_bin_dir']}/cargo",
            "build",
            f"--manifest-path={paths['canister']}/kybra_deployer/Cargo.toml",
            "--target=wasm32-unknown-unknown",
            f"--package=kybra_deployer",
            "--release",
        ],
        verbose,
        cargo_env,
    )

    copy_file(
        f"{paths['global_kybra_target_dir']}/wasm32-unknown-unknown/release/kybra_deployer.wasm",
        f"{paths['canister']}/{canister_name}.wasm",
    )

    copy_file(
        f"{paths['canister']}/{canister_name}.wasm",
        f"{paths['global_kybra_bin_dir']}/deployer.wasm",
    )


def get_should_rebuild(paths: Paths) -> bool:
    return os.environ.get("KYBRA_REBUILD") != "true" and os.path.exists(
        f"{paths['global_kybra_bin_dir']}/deployer.wasm"
    )


def run_subprocess(args: list[str], verbose: bool, env: dict[str, str]):
    result = subprocess.run(args, capture_output=not verbose, env=env)
    if result.returncode != 0:
        print_error_and_exit(result)


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
