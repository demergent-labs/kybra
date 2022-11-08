# type: ignore

import kybra
import modulegraph.modulegraph
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

def main():
    args = parse_args_or_exit(sys.argv)
    paths = create_paths(args)

    # This is the name of the canister passed into python -m kybra from the dfx.json build command
    canister_name = args['canister_name']

    verbose_mode_qualifier = ' in verbose mode' if args['flags']['verbose'] else ''

    print(f'\nBuilding canister {green(canister_name)}{verbose_mode_qualifier}\n')

    # Copy all of the Rust project structure from the pip package to an area designed for Rust compiling
    shutil.copytree(paths['compiler'], paths['canister'], dirs_exist_ok=True)

    # Add CARGO_TARGET_DIR to env for all cargo commands
    cargo_env = { **os.environ.copy(), 'CARGO_TARGET_DIR': paths['target'] }

    compile_python_or_exit(paths, cargo_env)
    build_wasm_binary_or_exit(paths, cargo_env)
    generate_candid_file_or_exit(paths, cargo_env)

    print(f"\n🎉 Built canister {green(canister_name)} at {dim(paths['gzipped_wasm'])}")

def parse_args_or_exit(args: list[str]):
    args = args[1:] # Discard the path to kybra

    flags = [arg for arg in args if (arg.startswith('-') or arg.startswith('--')) ]
    args = [arg for arg in args if not (arg.startswith('-') or arg.startswith('--')) ]

    if len(args) == 0:
        print('\nkybra v0.0.4')
        print('\nUsage: kybra [-v|--verbose] <canister_name> <entry_point> <did_path>')
        sys.exit(0)

    if len(args) != 3:
        print(red('\n💣 wrong number of arguments\n'))
        print('Usage: kybra [-v|--verbose] <canister_name> <entry_point> <did_path>')
        print('\n💀 Build failed!')
        sys.exit(1)

    return {
        'empty': False,
        'flags': {
            'verbose': '--verbose' in flags or '-v' in flags
        },
        'canister_name': args[0],
        'entry_point': args[1],
        'did_path': args[2]
    }

def create_paths(args):
    canister_name = args['canister_name']

    # This is the path to the developer's entry point Python file passed into python -m kybra from the dfx.json build command
    py_entry_file_path = args['entry_point']

    # This is the Python module name of the developer's Python project, derived from the entry point Python file passed into python -m kybra from the dfx.json build command
    py_entry_module_name = Path(py_entry_file_path).stem

    # This is the location of all code used to generate the final canister Rust code
    canister_path = f'.dfx/kybra/{canister_name}'

    # We want to bundle/gather all Python files into the python_source directory for RustPython freezing
    # The location that Kybra will look to when running py_freeze!
    # py_freeze! will compile all of the Python code in the directory recursively (modules must have an __init__.py to be included)
    python_source_path = f'{canister_path}/python_source'

    py_file_names_file_path = f'{canister_path}/file_names.txt'

    # This is the path to the developer's Candid file passed into python -m kybra from the dfx.json build command
    did_path = args['did_path']

    # This is the path to the Kybra compiler Rust code delivered with the Python package
    compiler_path = os.path.dirname(kybra.__file__) + '/compiler'

    # This is the final generated Rust file that is the canister
    lib_path = f'{canister_path}/src/lib.rs'

    # This is the location of the Candid file generated from the final generated Rust file
    generated_did_path = f'{canister_path}/index.did'

    # This is the Rust target directory
    target_path = f'{canister_path}/target'

    # This is the unzipped generated Wasm that is the canister
    wasm_path = f'{target_path}/wasm32-unknown-unknown/release/{canister_name}.wasm'

    # This is the final zipped generated Wasm that will actually run on the Internet Computer
    gzipped_wasm_path = f'{wasm_path}.gz'

    # This is where we store custom Python modules, such as stripped-down versions of stdlib modules
    custom_modules_path = f'{compiler_path}/custom_modules'

    return {
        'py_entry_file': py_entry_file_path,
        'py_entry_module_name': py_entry_module_name,
        'canister': canister_path,
        'python_source': python_source_path,
        'py_file_names_file': py_file_names_file_path,
        'did': did_path,
        'compiler': compiler_path,
        'lib': lib_path,
        'generated_did': generated_did_path,
        'target': target_path,
        'wasm': wasm_path,
        'gzipped_wasm': gzipped_wasm_path,
        'custom_modules': custom_modules_path,
    }

def compile_python_or_exit(paths, cargo_env):
    print('[1/3] 🔨 Compiling Python...')
    bundle_python_code(paths)
    add_wasm_compilation_target_or_exit()
    install_ic_ckd_optimizer_or_exit()
    generated_rust_code = run_kybra_generate_or_exit(paths, cargo_env)
    formatted_lib_file = run_rustfmt_or_exit(generated_rust_code)
    create_file(paths['lib'], formatted_lib_file)

def bundle_python_code(paths):
    # Begin module bundling/gathering process
    path = list(filter(lambda x: x.startswith(os.getcwd()), sys.path)) + [os.path.dirname(paths['py_entry_file'])]

    graph = modulegraph.modulegraph.ModuleGraph(path)
    entry_point = graph.run_script(paths['py_entry_file'])

    python_source_path = paths['python_source']

    if os.path.exists(python_source_path):
        shutil.rmtree(python_source_path)

    os.makedirs(python_source_path)

    # Copy our custom Python modules into the python_source directory
    shutil.copytree(paths['custom_modules'], python_source_path, dirs_exist_ok=True)

    flattened_graph = list(graph.flatten(start=entry_point))

    for node in flattened_graph:
        if type(node) == modulegraph.modulegraph.Script:
            shutil.copy(node.filename, f'{python_source_path}/{os.path.basename(node.filename)}')

        if type(node) == modulegraph.modulegraph.SourceModule:
            shutil.copy(node.filename, f'{python_source_path}/{os.path.basename(node.filename)}')

        if type(node) == modulegraph.modulegraph.Package:
            shutil.copytree(node.packagepath[0], f'{python_source_path}/{node.identifier}', dirs_exist_ok=True)

        if type(node) == modulegraph.modulegraph.NamespacePackage:
            shutil.copytree(node.packagepath[0], f'{python_source_path}/{node.identifier}', dirs_exist_ok=True)

    py_file_names = list(
        filter(
            lambda filename: filename is not None,
            map(
                lambda node: node.filename,
                filter(
                    lambda node: node.filename is not '-', # This filters out namespace packages
                    flattened_graph
                )
            )
        )
    )

    create_file(paths['py_file_names_file'], ','.join(py_file_names))

def add_wasm_compilation_target_or_exit():
    add_wasm_target_result = subprocess.run(['rustup', 'target', 'add', 'wasm32-unknown-unknown'], capture_output=True)

    if add_wasm_target_result.returncode != 0:
        print(red("\n💣 Unable to add wasm32-unknown-unknown compilation target\n"))
        print(add_wasm_target_result.stderr.decode('utf-8'))
        print('💀 Build failed')
        sys.exit(1)

def install_ic_ckd_optimizer_or_exit():
    # TODO: Figure out why this fails and how we want to handle it. Previously, we chained on an `||  true`.
    # TODO: We might also be able to do `--force`

    # TODO this should eventually be replaced with ic-wasm once this is resolved: https://forum.dfinity.org/t/wasm-module-contains-a-function-that-is-too-complex/15407/43?u=lastmjs
    install_cdk_optimizer_result = subprocess.run(['cargo', 'install', 'ic-cdk-optimizer', '--version=0.3.4'], capture_output=True)
    # install_ic_wasm_result = subprocess.run(['cargo', 'install', 'ic-wasm', '--version=0.3.0'], capture_output=True)

    if install_cdk_optimizer_result.returncode != 0:
        print(red("\n💣 Unable to install dependency \"ic-cdk-optimizer\"\n"))
        print(install_cdk_optimizer_result.stderr.decode('utf-8'))
        print('💀 Build failed')
        sys.exit(1)

def run_kybra_generate_or_exit(paths, cargo_env):
    # Generate the Rust code
    kybra_generate_result = subprocess.run(
        [
            'cargo',
            'run',
            f"--manifest-path={paths['canister']}/kybra_generate/Cargo.toml",
            paths['py_file_names_file'],
            paths['py_entry_module_name']
        ],
        capture_output=True,
        env=cargo_env
    )

    if kybra_generate_result.returncode != 0:
        print(red("\n💣 Something about your Python code violates Kybra's requirements\n"))
        print(parse_kybra_generate_error(kybra_generate_result.stderr))
        print('\nIf you are unable to decipher the error above, reach out in the #typescript')
        print('channel of the DFINITY DEV OFFICIAL discord:')
        print('\nhttps://discord.com/channels/748416164832608337/1019372359775440988\n')
        print('💀 Build failed')
        sys.exit(1)

    return kybra_generate_result.stdout

def parse_kybra_generate_error(stdout: bytes):
    err = stdout.decode('utf-8')
    std_err_lines = err.splitlines()
    try:
        line_where_error_message_starts = next(i for i,v in enumerate(std_err_lines) if v.startswith("thread 'main' panicked at '"))
        line_where_error_message_ends = next(i for i,v in enumerate(std_err_lines) if "', src/" in v)
    except:
        return 'The underlying cause is likely at the bottom of the following output:\n\n' + err

    err_lines = std_err_lines[line_where_error_message_starts:line_where_error_message_ends + 1]
    err_lines[0] = err_lines[0].replace("thread 'main' panicked at '", '')
    err_lines[-1] = re.sub("', src\/.*", "", err_lines[-1])

    return red("\n".join(err_lines))

def run_rustfmt_or_exit(generated_rust_code):
    rustfmt_result = subprocess.run(['rustfmt', '--edition=2018'], capture_output=True, input=generated_rust_code)

    if rustfmt_result.returncode != 0:
        print(red("\n💣 Kybra has experienced an internal error while trying to\n   format your generated rust canister"))
        print(f'\nPlease open an issue at https://github.com/demergent-labs/kybra/issues/new\nincluding this message and the following error:\n\n {red(rustfmt_result.stderr.decode("utf-8"))}')
        print('💀 Build failed')
        sys.exit(1)

    return rustfmt_result.stdout.decode('utf-8')

def build_wasm_binary_or_exit(paths, cargo_env):
    # Compile the generated Rust code
    print('[2/3] 🚧 Building Wasm binary...')
    cargo_build_result = subprocess.run(
        [
            "cargo",
            "build",
            f"--manifest-path={paths['canister']}/Cargo.toml",
            "--target=wasm32-unknown-unknown",
            "--package=kybra_generated_canister",
            "--release",
        ],
        capture_output=True,
        env=cargo_env,
    )

    if cargo_build_result.returncode != 0:
        print(red("\n💣 Error building Wasm binary:"))
        print(cargo_build_result.stderr.decode('utf-8'))
        print('💀 Build failed')
        sys.exit(1)

    cargo_bin_root = os.environ.get('CARGO_INSTALL_ROOT') or os.environ.get('CARGO_HOME') or f'{os.environ["HOME"]}/.cargo'

    # Optimize the Wasm binary
    # TODO this should eventually be replaced with ic-wasm once this is resolved: https://forum.dfinity.org/t/wasm-module-contains-a-function-that-is-too-complex/15407/43?u=lastmjs
    optimization_result = subprocess.run(
        [
            f'{cargo_bin_root}/bin/ic-cdk-optimizer',
            f"{paths['target']}/wasm32-unknown-unknown/release/kybra_generated_canister.wasm",
            f"-o={paths['wasm']}",
        ],
        capture_output=True
    )
    # optimization_result = subprocess.run(
    #     [
    #         f'{cargo_bin_root}/bin/ic-wasm',
    #         f"{paths['target']}/wasm32-unknown-unknown/release/kybra_generated_canister.wasm",
    #         f"-o={paths['wasm']}",
    #         'shrink'
    #     ],
    #     capture_output=True
    # )


    if optimization_result.returncode != 0:
        print(red("\n💣 Error optimizing generated Wasm:"))
        print(optimization_result.stderr.decode('utf-8'))
        print('💀 Build failed')
        sys.exit(1)

    # Copy the generated Candid file to the developer's source directory
    os.system(f"cp {paths['generated_did']} {paths['did']}")


    # gzip the Wasm binary
    os.system(f"gzip -f -k {paths['wasm']}")

def generate_candid_file_or_exit(paths, cargo_env):
    # Generate the Candid file
    print('[3/3] 📝 Generating Candid file...')
    generate_candid_result = subprocess.run(
        [
            "cargo",
            "test",
            f"--manifest-path={paths['canister']}/Cargo.toml",
        ],
        capture_output=True,
        env=cargo_env,
    )

    if generate_candid_result.returncode != 0:
        print(red("\n💣 Error generating candid:"))
        print(generate_candid_result.stderr.decode('utf-8'))
        print('💀 Build failed')
        sys.exit(1)

    # Copy the generated Candid file to the developer's source directory
    os.system(f"cp {paths['generated_did']} {paths['did']}")

# region Colors
def red(text):
    return f'\x1b[31m{text}\x1b[0m'

def yellow(text):
    return f'\x1b[33m{text}\x1b[0m'

def green(text):
    return f'\x1b[32m{text}\x1b[0m'

def blue(text):
    return f'\x1b[34m{text}\x1b[0m'

def purple(text):
    return f'\x1b[35m{text}\x1b[0m'

def dim(text):
    return f'\x1b[2m{text}\x1b[0m'
# endregion Colors

def create_file(file_path: str, contents: str):
    file = open(file_path, 'w')
    file.write(contents)
    file.close()

def timed(body):
    start_time = time.time()
    body()
    end_time = time.time()
    duration = end_time - start_time
    print(f'\nDone in {round(duration, 2)}s.')
    return end_time - start_time

timed(main)
