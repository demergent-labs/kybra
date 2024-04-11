import os
import re
import shutil
import subprocess
import sys
from typing import Callable

import kybra
from kybra.colors import red
from kybra.types import Paths


def run_kybra_generate_or_exit(paths: Paths, cargo_env: dict[str, str], verbose: bool):
    bin_path, bin_path_debug = construct_bin_paths(paths)
    should_rebuild = check_rebuild(bin_path)
    if should_rebuild:
        run_build(bin_path, bin_path_debug, paths, cargo_env, verbose)
    run_kybra_generate(bin_path, paths, cargo_env, verbose)


def construct_bin_paths(paths: Paths) -> tuple[str, str]:
    bin_path = (
        f"{paths['global_kybra_config_dir']}/{kybra.__version__}/bin/kybra_generate"
    )
    bin_path_debug = f"{paths['global_kybra_target_dir']}/debug/kybra_generate"
    return bin_path, bin_path_debug


def check_rebuild(bin_path: str) -> bool:
    return not os.path.exists(bin_path) or os.environ.get("KYBRA_REBUILD") == "true"


def run_build(
    bin_path: str,
    bin_path_debug: str,
    paths: Paths,
    cargo_env: dict[str, str],
    verbose: bool,
):
    build_result = execute_build_process(paths, cargo_env, verbose)
    validate_process(build_result, parse_kybra_generate_error)
    copy_file(bin_path_debug, bin_path)


def execute_build_process(
    paths: Paths, cargo_env: dict[str, str], verbose: bool
) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        [
            f"{paths['global_kybra_rust_bin_dir']}/cargo",
            "build",
            f"--manifest-path={paths['canister']}/kybra_generate/Cargo.toml",
        ],
        capture_output=not verbose,
        env=cargo_env,
    )


def copy_file(source: str, destination: str):
    shutil.copy(source, destination)


def run_kybra_generate(
    bin_path: str, paths: Paths, cargo_env: dict[str, str], verbose: bool
):
    generate_result = execute_generate_process(bin_path, paths, cargo_env, verbose)
    validate_process(generate_result, parse_kybra_generate_error)


def execute_generate_process(
    bin_path: str, paths: Paths, cargo_env: dict[str, str], verbose: bool
) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run(
        [
            bin_path,
            paths["py_file_names_file"],
            paths["py_entry_module_name"],
            paths["lib"],
            kybra.__version__,
        ],
        capture_output=not verbose,
        env=cargo_env,
    )


def validate_process(
    result: subprocess.CompletedProcess[bytes], error_parser: Callable[[bytes], str]
) -> None:
    if result.returncode != 0:
        print_error_and_exit_kybra_generate(result, error_parser)


def print_error_and_exit_kybra_generate(
    result: subprocess.CompletedProcess[bytes], error_parser: Callable[[bytes], str]
) -> None:
    print(red("\n💣 Kybra error: compilation\n"))
    print(error_parser(result.stderr))
    print_help_message()
    sys.exit(1)


def print_help_message():
    print(
        "\nFor help reach out in the #python channel of the ICP Developer Community discord:"
    )
    print("\nhttps://discord.com/channels/748416164832608337/1019372359775440988\n")
    print("💀 Build failed")


def parse_kybra_generate_error(stdout: bytes) -> str:
    err = stdout.decode("utf-8")
    std_err_lines = err.splitlines()
    try:
        line_where_error_message_starts = next(
            i
            for i, v in enumerate(std_err_lines)
            if v.startswith("thread 'main' panicked at '")
        )
        line_where_error_message_ends = next(
            i for i, v in enumerate(std_err_lines) if "', src/" in v
        )
    except:
        return err

    err_lines = std_err_lines[
        line_where_error_message_starts : line_where_error_message_ends + 1
    ]
    err_lines[0] = err_lines[0].replace("thread 'main' panicked at '", "")
    err_lines[-1] = re.sub("', src/.*", "", err_lines[-1])

    return red("\n".join(err_lines))
