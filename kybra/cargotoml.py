def generate_cargo_toml(canister_name: str) -> str:
    return f"""
[package]
name = "{canister_name}"
version = "0.0.0"
edition = "2018"

[profile.release]
opt-level = 'z'

[profile.test]
opt-level = 'z'

[lib]
crate-type = ["cdylib"]

[dependencies]
ic-cdk = {{ version = "0.6.8", features = ["timers"] }}
ic-cdk-macros = "0.6.8"
candid = "0.8.4"
kybra-vm-value-derive = {{ path = "./kybra_vm_value_derive" }}
# TODO add this back once we support the full stdlib: https://github.com/demergent-labs/kybra/issues/12
# rustpython = {{ git = "https://github.com/demergent-labs/RustPython", rev = "9ca024b30446249cc2a584543bbc658ab4b65c6f", default-features = false, features = ["stdlib", "freeze-stdlib"] }}
rustpython = {{ git = "https://github.com/demergent-labs/RustPython", rev = "9ca024b30446249cc2a584543bbc658ab4b65c6f", default-features = false, features = ["stdlib"] }}
rustpython-vm = {{ git = "https://github.com/demergent-labs/RustPython", rev = "9ca024b30446249cc2a584543bbc658ab4b65c6f", default-features = false, features = [] }}
rustpython-stdlib = {{ git = "https://github.com/demergent-labs/RustPython", rev = "9ca024b30446249cc2a584543bbc658ab4b65c6f", default-features = false, features = [] }}
rustpython-derive = {{ git = "https://github.com/demergent-labs/RustPython", rev = "9ca024b30446249cc2a584543bbc658ab4b65c6f", default-features = false, features = [] }}
# TODO add this back once we support the full stdlib: https://github.com/demergent-labs/kybra/issues/12
# rustpython-pylib = {{ git = "https://github.com/demergent-labs/RustPython", rev = "9ca024b30446249cc2a584543bbc658ab4b65c6f", default-features = false, features = [] }}
# rustpython = {{ path = "../../../../../../RustPython", default-features = false, features = [] }}
getrandom = {{ version = "0.2.3", features = ["custom"] }}
serde = "1.0.137"
async-recursion = "1.0.0"
ic-stable-structures = "0.3.0"
slotmap = "1.0.6"
    """
