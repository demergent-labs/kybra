from typing import TypedDict

class Args(TypedDict):
    empty: bool
    flags: "Args_flags"
    canister_name: str
    entry_point: str
    did_path: str

class Args_flags(TypedDict):
    verbose: bool

class Paths(TypedDict):
    py_entry_file: str
    py_entry_module_name: str
    canister: str
    python_source: str
    py_file_names_file: str
    did: str
    compiler: str
    lib: str
    generated_did: str
    target: str
    wasm: str
    gzipped_wasm: str
    custom_modules: str
