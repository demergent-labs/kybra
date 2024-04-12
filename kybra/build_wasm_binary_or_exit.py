import os
import shutil
import subprocess
import sys

import kybra
from kybra.colors import red
from kybra.timed import timed_inline
from kybra.types import Paths


@timed_inline
def build_wasm_binary_or_exit(
    paths: Paths, canister_name: str, cargo_env: dict[str, str], verbose: bool = False
):
    compile_python_stdlib(paths, cargo_env, verbose)
    compile_generated_rust_code(paths, canister_name, cargo_env, verbose)
    copy_wasm_to_dev_location(paths, canister_name)
    run_wasi2ic_on_wasm(paths, canister_name, cargo_env, verbose)
    generate_and_create_candid_file(paths, canister_name, cargo_env, verbose)


def compile_python_stdlib(paths: Paths, cargo_env: dict[str, str], verbose: bool):
    if os.environ.get("KYBRA_COMPILE_RUST_PYTHON_STDLIB") == "true":
        rust_python_path = f"{paths['global_kybra_version_dir']}/RustPython"

        if not os.path.exists(rust_python_path):
            run_subprocess(
                ["git", "clone", "https://github.com/RustPython/RustPython.git"],
                cargo_env,
                verbose,
                cwd=paths["global_kybra_version_dir"],
            )

            run_subprocess(
                ["git", "checkout", "f12875027ce425297c07cbccb9be77514ed46157"],
                cargo_env,
                verbose,
                cwd=f"{paths['global_kybra_version_dir']}/RustPython",
            )

        shutil.copytree(
            f"{rust_python_path}/Lib",
            f"{paths['canister']}/Lib",
        )

        os.makedirs(f"{paths['canister']}/rust_python_stdlib")
        shutil.copy(
            os.path.dirname(kybra.__file__) + "/compiler/LICENSE-RustPython",
            f"{paths['canister']}/rust_python_stdlib/LICENSE-RustPython",
        )
        shutil.copy(
            os.path.dirname(kybra.__file__) + "/compiler/python_3_10_13_licenses.pdf",
            f"{paths['canister']}/rust_python_stdlib/python_3_10_13_licenses.pdf",
        )

        run_subprocess(
            [
                f"{paths['global_kybra_rust_bin_dir']}/cargo",
                "run",
                f"--manifest-path={paths['canister']}/kybra_compile_python_stdlib/Cargo.toml",
                f"--package=kybra_compile_python_stdlib",
                f"{paths['canister']}/rust_python_stdlib/stdlib",
            ],
            cargo_env,
            verbose,
        )
    else:
        rust_python_stdlib_path = (
            f"{paths['global_kybra_version_dir']}/rust_python_stdlib"
        )

        if not os.path.exists(rust_python_stdlib_path):
            run_subprocess(
                [
                    "curl",
                    "-Lf",
                    f"https://github.com/demergent-labs/kybra/releases/download/{kybra.__version__}/rust_python_stdlib.tar.gz",
                    "-o",
                    "rust_python_stdlib.tar.gz",
                ],
                cargo_env,
                verbose,
                cwd=paths["global_kybra_version_dir"],
            )

            run_subprocess(
                ["tar", "-xvf", "rust_python_stdlib.tar.gz"],
                cargo_env,
                verbose,
                cwd=paths["global_kybra_version_dir"],
            )

        shutil.copytree(
            rust_python_stdlib_path,
            f"{paths['canister']}/rust_python_stdlib",
        )


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
    args: list[str],
    env: dict[str, str],
    verbose: bool,
    throw: bool = True,
    cwd: str | None = None,
) -> bytes:
    result = subprocess.run(args, env=env, capture_output=not verbose, cwd=cwd)

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
