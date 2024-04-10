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
    wasm: str
    custom_modules: str
    global_kybra_config_dir: str
    global_kybra_version_dir: str
    global_kybra_rust_dir: str
    global_kybra_rust_bin_dir: str
    global_kybra_bin_dir: str
    global_kybra_target_dir: str
