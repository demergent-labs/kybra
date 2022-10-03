# type: ignore

import kybra
import modulegraph.modulegraph # TODO this needs to be a dependency when installing kybra somehow
import os
import shutil
import subprocess
import sys

canister_name = sys.argv[1]
py_entry_file_path = sys.argv[2]
did_path = sys.argv[3]
compiler_path = os.path.dirname(kybra.__file__) + '/compiler'
canister_path=f'.dfx/kybra/{canister_name}'
build_sh_path = compiler_path + '/build.sh'

shutil.copytree(compiler_path, canister_path, dirs_exist_ok=True)

path = sys.path[:]
path[0] = os.path.dirname(py_entry_file_path)

graph = modulegraph.modulegraph.ModuleGraph(path)
entry_point = graph.run_script(py_entry_file_path)

python_source_path = f'{canister_path}/python_source'

if os.path.exists(python_source_path):
    shutil.rmtree(python_source_path)

os.makedirs(python_source_path)

for node in graph.flatten(start=entry_point):
    if type(node) == modulegraph.modulegraph.Script:
        shutil.copy(node.filename, f'{python_source_path}/{os.path.basename(node.filename)}')

    if type(node) == modulegraph.modulegraph.SourceModule:
        shutil.copy(node.filename, f'{python_source_path}/{os.path.basename(node.filename)}')

    if type(node) == modulegraph.modulegraph.Package:
        packagepath = node.packagepath[0]
        destination_path = f'{python_source_path}/{node.identifier}'
        shutil.copytree(packagepath, destination_path, dirs_exist_ok=True)

    if type(node) == modulegraph.modulegraph.BuiltinModule:
        print(node)

    if type(node) == modulegraph.modulegraph.MissingModule:
        print(node)

subprocess.call([build_sh_path, canister_name, py_entry_file_path, did_path, compiler_path])
