# type: ignore

import kybra
import modulegraph.modulegraph
import os
import shutil
import sys
from pathlib import Path

# This is the name of the canister passed into python -m kybra from the dfx.json build command
canister_name = sys.argv[1]

print(f'\nBuilding canister {canister_name}\n')

# This is the path to the developer's entry point Python file passed into python -m kybra from the dfx.json build command
py_entry_file_path = sys.argv[2]

# This is the Python module name of the developer's Python project, derived from the entry point Python file passed into python -m kybra from the dfx.json build command
py_entry_module_name = Path(py_entry_file_path).stem

# This is the location of all code used to generate the final canister Rust code
canister_path=f'.dfx/kybra/{canister_name}'

py_file_names_file_path = f'{canister_path}/file_names.txt'

# This is the path to the developer's Candid file passed into python -m kybra from the dfx.json build command
did_path = sys.argv[3]

# This is the path to the Kybra compiler Rust code delivered with the Python package
compiler_path = os.path.dirname(kybra.__file__) + '/compiler'

# This is the final generated Rust file that is the canister
lib_path=f'{canister_path}/src/lib.rs'

# This is the location of the Candid file generated from the final generated Rust file
generated_did_path = f'{canister_path}/index.did'

# This is the Rust target directory
target_path=f'{canister_path}/target'

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

# Rust prerequisites
os.system('rustup target add wasm32-unknown-unknown')
# TODO this should eventually be replaced with ic-wasm once this is resolved: https://forum.dfinity.org/t/wasm-module-contains-a-function-that-is-too-complex/15407/43?u=lastmjs
os.system('cargo install ic-cdk-optimizer --version 0.3.4 || true')
# os.system('cargo install ic-wasm --version 0.3.0 || true')

py_file_names_file = open(py_file_names_file_path, 'w')
py_file_names_file.write(','.join(py_file_names))
py_file_names_file.close()

# Generate the Rust code
print('[1/3] 🔨 Compiling Python...')
os.system(f'CARGO_TARGET_DIR={target_path} cargo run --manifest-path {canister_path}/kybra_generate/Cargo.toml {py_file_names_file_path} {py_entry_module_name} | rustfmt --edition 2018 > {lib_path}')

# Compile the generated Rust code
print('[2/3] 🚧 Building Wasm binary...')
os.system(f'CARGO_TARGET_DIR={target_path} cargo build --manifest-path {canister_path}/Cargo.toml --target wasm32-unknown-unknown --package kybra_generated_canister --release')

# Generate the Candid file
print('[3/3] 📝 Generating Candid file...')
os.system(f'CARGO_TARGET_DIR={target_path} cargo test --manifest-path {canister_path}/Cargo.toml')

# Copy the generated Candid file to the developer's source directory
os.system(f'cp {generated_did_path} {did_path}')

# Optimize the Wasm binary
# TODO this should eventually be replaced with ic-wasm once this is resolved: https://forum.dfinity.org/t/wasm-module-contains-a-function-that-is-too-complex/15407/43?u=lastmjs
os.system(f'{cargo_bin_root}/bin/ic-cdk-optimizer {target_path}/wasm32-unknown-unknown/release/kybra_generated_canister.wasm -o {target_path}/wasm32-unknown-unknown/release/{canister_name}.wasm')
# os.system(f'{cargo_bin_root}/bin/ic-wasm {target_path}/wasm32-unknown-unknown/release/kybra_generated_canister.wasm -o {target_path}/wasm32-unknown-unknown/release/{canister_name}.wasm shrink')

# gzip the Wasm binary
os.system(f'gzip -f -k {target_path}/wasm32-unknown-unknown/release/{canister_name}.wasm')
