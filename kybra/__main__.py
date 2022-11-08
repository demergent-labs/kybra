# type: ignore

import kybra
import modulegraph.modulegraph
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

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

# region Helper Methods
def parse_args(args: list[str]):
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

def create_file(file_path: str, contents: str):
    file = open(file_path, 'w')
    file.write(contents)
    file.close()
# endregion Helper Methods

args = parse_args(sys.argv)

# if args['flags']['verbose']:
#     print('We are in verbose mode!')

# This is the name of the canister passed into python -m kybra from the dfx.json build command
canister_name = args['canister_name']

print(f'\nBuilding canister {green(canister_name)}\n')

# This is the path to the developer's entry point Python file passed into python -m kybra from the dfx.json build command
py_entry_file_path = args['entry_point']

# This is the Python module name of the developer's Python project, derived from the entry point Python file passed into python -m kybra from the dfx.json build command
py_entry_module_name = Path(py_entry_file_path).stem

# This is the location of all code used to generate the final canister Rust code
canister_path=f'.dfx/kybra/{canister_name}'

py_file_names_file_path = f'{canister_path}/file_names.txt'

# This is the path to the developer's Candid file passed into python -m kybra from the dfx.json build command
did_path = args['did_path']

# This is the path to the Kybra compiler Rust code delivered with the Python package
compiler_path = os.path.dirname(kybra.__file__) + '/compiler'

# This is the final generated Rust file that is the canister
lib_path=f'{canister_path}/src/lib.rs'

# This is the location of the Candid file generated from the final generated Rust file
generated_did_path = f'{canister_path}/index.did'

# This is the Rust target directory
target_path=f'{canister_path}/target'

# This is the unzipped generated Wasm that is the canister
wasm_path = f'{target_path}/wasm32-unknown-unknown/release/{canister_name}.wasm'

# This is the final zipped generated Wasm that will actually run on the Internet Computer
gzipped_wasm_path = f'{wasm_path}.gz'

cargo_bin_root = os.environ.get('CARGO_INSTALL_ROOT') or os.environ.get('CARGO_HOME') or f'{os.environ["HOME"]}/.cargo'

# This is where we store custom Python modules, such as stripped-down versions of stdlib modules
custom_modules_path = f'{compiler_path}/custom_modules'

# Copy all of the Rust project structure from the pip package to an area designed for Rust compiling
shutil.copytree(compiler_path, canister_path, dirs_exist_ok=True)

# Begin module bundling/gathering process
path = list(filter(lambda x: x.startswith(os.getcwd()), sys.path)) + [os.path.dirname(py_entry_file_path)]

graph = modulegraph.modulegraph.ModuleGraph(path)
entry_point = graph.run_script(py_entry_file_path)

# We want to bundle/gather all Python files into the python_source directory for RustPython freezing
# The location that Kybra will look to when running py_freeze!
# py_freeze! will compile all of the Python code in the directory recursively (modules must have an __init__.py to be included)
python_source_path = f'{canister_path}/python_source'

if os.path.exists(python_source_path):
    shutil.rmtree(python_source_path)

os.makedirs(python_source_path)

# Copy our custom Python modules into the python_source directory
shutil.copytree(custom_modules_path, python_source_path, dirs_exist_ok=True)

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

print('[1/3] 🔨 Compiling Python...')

# Rust prerequisites
add_wasm_target_result = subprocess.run(['rustup', 'target', 'add', 'wasm32-unknown-unknown'], capture_output=True)

if add_wasm_target_result.returncode != 0:
    print(red("\n💣 Unable to add wasm32-unknown-unknown compilation target\n"))
    print(add_wasm_target_result.stderr.decode('utf-8'))
    print('💀 Build failed')
    sys.exit(1)


# TODO this should eventually be replaced with ic-wasm once this is resolved: https://forum.dfinity.org/t/wasm-module-contains-a-function-that-is-too-complex/15407/43?u=lastmjs
install_cdk_optimizer_result = subprocess.run(['cargo', 'install', 'ic-cdk-optimizer', '--version=0.3.4'], capture_output=True)
# install_cdk_optimizer_result = subprocess.run(['cargo', 'install', 'ic-wasm', '--version=0.3.0'], capture_output=True)

if install_cdk_optimizer_result.returncode != 0:
    print(red("\n💣 Unable to install dependency \"ic-cdk-optimizer\"\n"))
    print(install_cdk_optimizer_result.stderr.decode('utf-8'))
    print('💀 Build failed')
    sys.exit(1)

create_file(py_file_names_file_path, ','.join(py_file_names))

# Add CARGO_TARGET_DIR to env for all cargo commands
cargo_env = { **os.environ.copy(), 'CARGO_TARGET_DIR': target_path }

# Generate the Rust code
kybra_generate_result = subprocess.run(['cargo', 'run', f'--manifest-path={canister_path}/kybra_generate/Cargo.toml', py_file_names_file_path, py_entry_module_name], capture_output=True, env=cargo_env)

if kybra_generate_result.returncode != 0:
    print(red("\n💣 Something about your Python code violates Kybra's requirements\n"))
    print(parse_kybra_generate_error(kybra_generate_result.stderr))
    print('\nIf you are unable to decipher the error above, reach out in the #typescript')
    print('channel of the DFINITY DEV OFFICIAL discord:')
    print('\nhttps://discord.com/channels/748416164832608337/1019372359775440988\n')
    print('💀 Build failed')
    sys.exit(1)

generated_rust_code = kybra_generate_result.stdout

rustfmt_result = subprocess.run(['rustfmt', '--edition=2018'], capture_output=True, input=generated_rust_code)

if rustfmt_result.returncode != 0:
    print(red("\n💣 Kybra has experienced an internal error while trying to\n   format your generated rust canister"))
    print(f'\nPlease open an issue at https://github.com/demergent-labs/kybra/issues/new\nincluding this message and the following error:\n\n {red(rustfmt_result.stderr.decode("utf-8"))}')
    print('💀 Build failed')
    sys.exit(1)

formatted_lib_file = rustfmt_result.stdout.decode('utf-8')

create_file(lib_path, formatted_lib_file)

# Compile the generated Rust code
print('[2/3] 🚧 Building Wasm binary...')
cargo_build_result = subprocess.run(
    [
        "cargo",
        "build",
        f"--manifest-path={canister_path}/Cargo.toml",
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


# Generate the Candid file
print('[3/3] 📝 Generating Candid file...')
generate_candid_result = subprocess.run(
    [
        "cargo",
        "test",
        f"--manifest-path={canister_path}/Cargo.toml",
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
os.system(f'cp {generated_did_path} {did_path}')

# Optimize the Wasm binary
# TODO this should eventually be replaced with ic-wasm once this is resolved: https://forum.dfinity.org/t/wasm-module-contains-a-function-that-is-too-complex/15407/43?u=lastmjs
optimization_result = subprocess.run(
    [
        f'{cargo_bin_root}/bin/ic-cdk-optimizer',
        f'{target_path}/wasm32-unknown-unknown/release/kybra_generated_canister.wasm',
        f'-o={wasm_path}',
    ],
    capture_output=True
)
# optimization_result = subprocess.run(
#     [
#         f'{cargo_bin_root}/bin/ic-wasm',
#         f'{target_path}/wasm32-unknown-unknown/release/kybra_generated_canister.wasm',
#         f'-o={wasm_path}',
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
os.system(f'cp {generated_did_path} {did_path}')


# gzip the Wasm binary
os.system(f'gzip -f -k {wasm_path}')

print(f'\n🎉 Built canister {green(canister_name)} at {dim(gzipped_wasm_path)}')
