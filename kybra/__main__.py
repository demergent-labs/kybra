import modulegraph.modulegraph  # type: ignore
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time
import site
from typing import Any, Callable

import kybra
from kybra.build_wasm_binary_or_exit import build_wasm_binary_or_exit
from kybra.cargotoml import generate_cargo_toml, generate_cargo_lock
from kybra.colors import red, green, dim
from kybra.run_kybra_generate_or_exit import run_kybra_generate_or_exit
from kybra.timed import timed, timed_inline
from kybra.types import Args, Paths


@timed
def main():
    args = parse_args_or_exit(sys.argv)
    paths = create_paths(args)
    is_verbose = args["flags"]["verbose"] or os.environ.get("KYBRA_VERBOSE") == "true"

    subprocess.run(
        [
            f"{paths['compiler']}/install_rust_dependencies.sh",
            kybra.__version__,
            kybra.__rust_version__,
        ]
    )

    # This is the name of the canister passed into python -m kybra from the dfx.json build command
    canister_name = args["canister_name"]

    verbose_mode_qualifier = " in verbose mode" if is_verbose else ""

    print(f"\nBuilding canister {green(canister_name)}{verbose_mode_qualifier}\n")

    # Copy all of the Rust project structure from the pip package to an area designed for Rust compiling
    if os.path.exists(paths["canister"]):
        shutil.rmtree(paths["canister"])
    shutil.copytree(paths["compiler"], paths["canister"], dirs_exist_ok=True)
    create_file(f"{paths['canister']}/Cargo.toml", generate_cargo_toml(canister_name))
    create_file(f"{paths['canister']}/Cargo.lock", generate_cargo_lock())

    # Add CARGO_TARGET_DIR to env for all cargo commands
    cargo_env = {
        **os.environ.copy(),
        "CARGO_TARGET_DIR": paths["global_kybra_target_dir"],
        "CARGO_HOME": paths["global_kybra_rust_dir"],
        "RUSTUP_HOME": paths["global_kybra_rust_dir"],
    }

    if not os.path.exists(paths["global_kybra_bin_dir"]):
        os.makedirs(paths["global_kybra_bin_dir"])

    compile_python_or_exit(
        paths, cargo_env, verbose=is_verbose, label="[1/2] 🔨 Compiling Python..."
    )

    build_wasm_binary_or_exit(
        paths,
        canister_name,
        cargo_env,
        verbose=is_verbose,
        label=f"[2/2] 🚧 Building Wasm binary...",
    )

    print(f"\n🎉 Built canister {green(canister_name)} at {dim(paths['wasm'])}")


def parse_args_or_exit(args: list[str]) -> Args:
    args = args[1:]  # Discard the path to kybra

    flags = [arg for arg in args if (arg.startswith("-") or arg.startswith("--"))]
    args = [arg for arg in args if not (arg.startswith("-") or arg.startswith("--"))]

    if len(args) == 0:
        print(f"\nkybra {kybra.__version__}")
        print("\nUsage: kybra [-v|--verbose] <canister_name> <entry_point> <did_path>")
        sys.exit(0)

    if len(args) != 3:
        print(red("\n💣 Kybra error: wrong number of arguments\n"))
        print("Usage: kybra [-v|--verbose] <canister_name> <entry_point> <did_path>")
        print("\n💀 Build failed!")
        sys.exit(1)

    return {
        "empty": False,
        "flags": {"verbose": "--verbose" in flags or "-v" in flags},
        "canister_name": args[0],
        "entry_point": args[1],
        "did_path": args[2],
    }


def create_paths(args: Args) -> Paths:
    canister_name = args["canister_name"]

    # This is the path to the developer's entry point Python file passed into python -m kybra from the dfx.json build command
    py_entry_file_path = args["entry_point"]

    # This is the Python module name of the developer's Python project, derived from the entry point Python file passed into python -m kybra from the dfx.json build command
    py_entry_module_name = Path(py_entry_file_path).stem

    # This is the location of all code used to generate the final canister Rust code
    canister_path = f".kybra/{canister_name}"

    # We want to bundle/gather all Python files into the python_source directory for RustPython freezing
    # The location that Kybra will look to when running py_freeze!
    # py_freeze! will compile all of the Python code in the directory recursively (modules must have an __init__.py to be included)
    python_source_path = f"{canister_path}/python_source"

    py_file_names_file_path = f"{canister_path}/py_file_names.csv"

    # This is the path to the developer's Candid file passed into python -m kybra from the dfx.json build command
    did_path = args["did_path"]

    # This is the path to the Kybra compiler Rust code delivered with the Python package
    compiler_path = os.path.dirname(kybra.__file__) + "/compiler"

    # This is the final generated Rust file that is the canister
    lib_path = f"{canister_path}/src/lib.rs"

    # This is the location of the Candid file generated from the final generated Rust file
    generated_did_path = f"{canister_path}/index.did"

    # This is the unzipped generated Wasm that is the canister
    wasm_path = f"{canister_path}/{canister_name}.wasm"

    # This is where we store custom Python modules, such as stripped-down versions of stdlib modules
    custom_modules_path = f"{compiler_path}/custom_modules"

    home_dir = os.path.expanduser("~")
    global_kybra_config_dir = f"{home_dir}/.config/kybra"
    global_kybra_version_dir = f"{global_kybra_config_dir}/{kybra.__version__}"
    global_kybra_rust_dir = f"{global_kybra_config_dir}/rust/{kybra.__rust_version__}"
    global_kybra_rust_bin_dir = f"{global_kybra_rust_dir}/bin"
    global_kybra_target_dir = f"{global_kybra_config_dir}/rust/target"
    global_kybra_bin_dir = f"{global_kybra_config_dir}/{kybra.__version__}/bin"

    return {
        "py_entry_file": py_entry_file_path,
        "py_entry_module_name": py_entry_module_name,
        "canister": canister_path,
        "python_source": python_source_path,
        "py_file_names_file": py_file_names_file_path,
        "did": did_path,
        "compiler": compiler_path,
        "lib": lib_path,
        "generated_did": generated_did_path,
        "wasm": wasm_path,
        "custom_modules": custom_modules_path,
        "global_kybra_config_dir": global_kybra_config_dir,
        "global_kybra_version_dir": global_kybra_version_dir,
        "global_kybra_rust_dir": global_kybra_rust_dir,
        "global_kybra_rust_bin_dir": global_kybra_rust_bin_dir,
        "global_kybra_target_dir": global_kybra_target_dir,
        "global_kybra_bin_dir": global_kybra_bin_dir,
    }


@timed_inline
def compile_python_or_exit(
    paths: Paths, cargo_env: dict[str, str], verbose: bool = False
):
    bundle_python_code(paths)
    run_kybra_generate_or_exit(paths, cargo_env, verbose)
    run_rustfmt_or_exit(paths, cargo_env, verbose)


def bundle_python_code(paths: Paths):
    # Begin module bundling/gathering process
    path = (
        list(filter(lambda x: x.startswith(os.getcwd()), sys.path))
        + [
            os.path.dirname(paths["py_entry_file"]),
        ]
        + site.getsitepackages()
    )

    graph = modulegraph.modulegraph.ModuleGraph(path)  # type: ignore
    entry_point = graph.run_script(paths["py_entry_file"])  # type: ignore

    python_source_path = paths["python_source"]

    if os.path.exists(python_source_path):
        shutil.rmtree(python_source_path)

    os.makedirs(python_source_path)

    # Copy our custom Python modules into the python_source directory
    shutil.copytree(paths["custom_modules"], python_source_path, dirs_exist_ok=True)

    flattened_graph = list(graph.flatten(start=entry_point))  # type: ignore

    for node in flattened_graph:  # type: ignore
        if type(node) == modulegraph.modulegraph.Script:  # type: ignore
            shutil.copy(
                node.filename, f"{python_source_path}/{os.path.basename(node.filename)}"  # type: ignore
            )

        if type(node) == modulegraph.modulegraph.SourceModule:  # type: ignore
            shutil.copy(
                node.filename, f"{python_source_path}/{os.path.basename(node.filename)}"  # type: ignore
            )

        if type(node) == modulegraph.modulegraph.Package:  # type: ignore
            shutil.copytree(
                node.packagepath[0],  # type: ignore
                f"{python_source_path}/{node.identifier}",  # type: ignore
                dirs_exist_ok=True,
                ignore=ignore_specific_dir,
            )

        if type(node) == modulegraph.modulegraph.NamespacePackage:  # type: ignore
            shutil.copytree(
                node.packagepath[0],  # type: ignore
                f"{python_source_path}/{node.identifier}",  # type: ignore
                dirs_exist_ok=True,
                ignore=ignore_specific_dir,
            )

    py_file_names = list(  # type: ignore
        filter(
            lambda filename: filename is not None and filename.endswith(".py"),  # type: ignore
            map(
                lambda node: node.filename,  # type: ignore
                filter(
                    lambda node: node.filename  # type: ignore
                    is not "-",  # This filters out namespace packages
                    flattened_graph,  # type: ignore
                ),  # type: ignore
            ),  # type: ignore
        )  # type: ignore
    )

    create_file(paths["py_file_names_file"], ",".join(py_file_names))  # type: ignore


def ignore_specific_dir(dirname: str, filenames: list[str]) -> list[str]:
    if "kybra_post_install/src/Lib" in dirname:
        return filenames
    else:
        return []


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


def run_rustfmt_or_exit(paths: Paths, cargo_env: dict[str, str], verbose: bool = False):
    rustfmt_result = subprocess.run(
        [
            f"{paths['global_kybra_rust_bin_dir']}/rustfmt",
            "--edition=2018",
            paths["lib"],
        ],
        capture_output=not verbose,
        env=cargo_env,
    )

    if rustfmt_result.returncode != 0:
        print(red("\n💣 Kybra error: internal Rust formatting"))
        print(
            f'\nPlease open an issue at https://github.com/demergent-labs/kybra/issues/new\nincluding this message and the following error:\n\n {red(rustfmt_result.stderr.decode("utf-8"))}'
        )
        print("💀 Build failed")
        sys.exit(1)


def create_file(file_path: str, contents: str):
    file = open(file_path, "w")
    file.write(contents)
    file.close()


def inline_timed(
    label: str,
    body: Callable[..., Any],
    *args: Any,
    verbose: bool = False,
    **kwargs: Any,
) -> float:
    print(label)
    start_time = time.time()
    body(*args, verbose=verbose, **kwargs)
    end_time = time.time()
    duration = end_time - start_time

    if verbose:
        print(f"{label} finished in {round(duration, 2)}s")
    else:
        move_cursor_up_one_line = "\x1b[1A"
        print(f'{move_cursor_up_one_line}{label} {dim(f"{round(duration, 2)}s")}')

    return end_time - start_time


main()
