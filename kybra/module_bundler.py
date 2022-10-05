# TODO this is the old module bundler for the stdlib
# TODO this code may become useful once this issue is resolved: https://github.com/demergent-labs/kybra/issues/12

# type: ignore

# TODO let's just get the bundler working for local files for now, no stdlib and no pip packages

# TODO just move all of the bash stuff into there probably


# TODO we need to make sure that RustPython/Lib is being analyzed and nothing builtin or implemented in Rust is being analyzed??

import kybra
import modulegraph.modulegraph # TODO this needs to be a dependency when installing kybra somehow
import os
import shutil
import subprocess
import sys

def handle_builtin_module(node) -> bool:
    stdlib_path = f'{canister_path}/RustPython/Lib'
    module_name = node.identifier

    if os.path.exists(f'{stdlib_path}/{module_name}'):
        shutil.copytree(f'{stdlib_path}/{module_name}', f'{python_source_path}/{module_name}', dirs_exist_ok=True)
        return True


    if os.path.exists(f'{stdlib_path}/{module_name}.py'):
        shutil.copy(f'{stdlib_path}/{module_name}.py', f'{python_source_path}/{module_name}.py')
        return True

    return False

canister_name = sys.argv[1]
py_entry_file_path = sys.argv[2]
did_path = sys.argv[3]
compiler_path = os.path.dirname(kybra.__file__) + '/compiler'
canister_path=f'.dfx/kybra/{canister_name}'
build_sh_path = compiler_path + '/build.sh'

shutil.copytree(compiler_path, canister_path, dirs_exist_ok=True)

# TODO git clone RustPython into .dfx/kybra/RustPython if it isn't already there
if not os.path.exists(f'{canister_path}/RustPython'):
    subprocess.call(['git', 'clone', '--single-branch', '--branch', 'kybra_initial', 'https://github.com/demergent-labs/RustPython', f'{canister_path}/RustPython'])

path = sys.path[:]
path[0] = os.path.dirname(py_entry_file_path)

# TODO xml.parsers has its own directory created for some reason

# TODO look up that cross-canister call way to install Wasm binaries, I think it allowed for 10mb uncompressed binaries
# TODO if that works we could at least hack it for now
# TODO new plan, just remove unnecessary modules into it works...then really push to get the binary limit increased or figure out future optimizations

# TODO warnings, collections, subprocess seem to be the real offenders
# TODO warnings, types, functools, collections, getopt, os, posixpath, subprocess
graph = modulegraph.modulegraph.ModuleGraph(path, ['json', 'array', 'binascii', 'bisect', 'bz2', 'cmath', 'contextvars', 'csv', 'dis', 'faulthandler', 'fcntl', 'gc', 'grp', 'hashlib', 'math', 'mmap', 'multiprocessing', 'posixsubprocess', 'pyexpat', 'struct', 'random', 're', 'resource', 'scproxy', 'select', 'socket', 'ssl', 'statistics', 'syslog', 'termios', 'unicodedata', 'uuid', 'zlib', 'test', 'unittest', 'warnings', 'distutils', 'pydoc_data', 'email', 'html', 'http', 'xml', 'encodings'])
entry_point = graph.run_script(py_entry_file_path)

python_source_path = f'{canister_path}/python_source'

if os.path.exists(python_source_path):
    shutil.rmtree(python_source_path)

os.makedirs(python_source_path)

num_nodes = 0

for node in graph.flatten(start=entry_point):
    num_nodes += 1

    print(node)

    if type(node) == modulegraph.modulegraph.Script:
        shutil.copy(node.filename, f'{python_source_path}/{os.path.basename(node.filename)}')

    if type(node) == modulegraph.modulegraph.SourceModule:
        if '.' in node.identifier:
            print(f'skipping {node.identifier}')
            continue

        builtin_module_handled = handle_builtin_module(node)

        if builtin_module_handled:
            print(f'{node.identifier} copied from RustPython/Lib')

        if not builtin_module_handled:
            shutil.copy(node.filename, f'{python_source_path}/{os.path.basename(node.filename)}')

    if type(node) == modulegraph.modulegraph.Package:
        builtin_module_handled = handle_builtin_module(node)

        if builtin_module_handled:
            print(f'{node.identifier} copied from RustPython/Lib')

        if not builtin_module_handled:
            packagepath = node.packagepath[0]
            destination_path = f'{python_source_path}/{node.identifier}'
            shutil.copytree(packagepath, destination_path, dirs_exist_ok=True)

    if type(node) == modulegraph.modulegraph.BuiltinModule:
        builtin_module_handled = handle_builtin_module(node)

        if builtin_module_handled:
            print(f'{node.identifier} copied from RustPython/Lib')

    if type(node) == modulegraph.modulegraph.MissingModule:
        print(node)

print(f'num_nodes: {num_nodes}')

subprocess.call([build_sh_path, canister_name, py_entry_file_path, did_path, compiler_path])
