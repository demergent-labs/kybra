#!/bin/bash

# TODO If we want to save a little bit of time we might be able to instruct rustup to not install some components initially, like clippy and docs
# TODO we might want to implement all of this in Python/Rust in the future for platform-independence etc
# TODO once ic-wasm is no longer required, we can probably just check for the wasm32-wasi target being installed: $global_kybra_rustup_bin target list | grep -q "wasm32-wasi (installed)"

kybra_version="$1"
rust_version="$2"

global_kybra_config_dir=~/.config/kybra
global_kybra_version_dir="$global_kybra_config_dir"/"$kybra_version"
global_kybra_rust_dir="$global_kybra_config_dir"/rust/"$rust_version"
global_kybra_rust_bin_dir="$global_kybra_rust_dir"/bin
global_kybra_logs_dir="$global_kybra_rust_dir"/logs
global_kybra_cargo_bin="$global_kybra_rust_bin_dir"/cargo
global_kybra_rustup_bin="$global_kybra_rust_bin_dir"/rustup

export CARGO_TARGET_DIR="$global_kybra_config_dir"/rust/target
export CARGO_HOME="$global_kybra_rust_dir"
export RUSTUP_HOME="$global_kybra_rust_dir"

function run() {
    ic_wasm_already_installed=$(test -e "$global_kybra_rust_bin_dir"/ic-wasm; echo $?)

    # TODO we should make this check much more robust
    if [ "$ic_wasm_already_installed" -eq 1 ]; then
        echo -e "\nKybra "$kybra_version" prerequisite installation (this may take a few minutes)\n"

        mkdir -p "$global_kybra_version_dir"
        mkdir -p "$global_kybra_rust_dir"
        mkdir -p "$global_kybra_logs_dir"

        install_rustup
        install_wasm32
        install_wasi2ic
        install_rust_python_stdlib
        install_ic_wasm "$ic_wasm_already_installed"
    else
        update_rustup
    fi
}

function install_rustup() {
    echo -e "1/5) Installing Rust"

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --no-modify-path -y --default-toolchain="$rust_version" &> "$global_kybra_logs_dir"/install_rustup
}

function update_rustup() {
    "$global_kybra_rustup_bin" update "$rust_version" &> "$global_kybra_logs_dir"/rustup_update
}

function install_wasm32() {
    echo -e "2/5) Installing wasm32"

    "$global_kybra_rustup_bin" target add wasm32-wasi &> "$global_kybra_logs_dir"/install_wasm32_wasi
    "$global_kybra_rustup_bin" target add wasm32-unknown-unknown &> "$global_kybra_logs_dir"/install_wasm32_unknown_unknown
}

function install_wasi2ic() {
    echo -e "3/5) Installing wasi2ic"

    "$global_kybra_cargo_bin" install --git https://github.com/wasm-forge/wasi2ic --rev 7418e0bd1a7810c8e9c55cc0155c921503a793b8 &> "$global_kybra_logs_dir"/install_wasi2ic
}

function install_rust_python_stdlib() {
    echo -e "5/6) Installing RustPython stdlib"

    cd "$global_kybra_version_dir"
    git clone https://github.com/RustPython/RustPython.git
    cd RustPython
    git checkout f12875027ce425297c07cbccb9be77514ed46157
    cd -
    cd -
}

function install_ic_wasm() {
    echo -e "6/6) Installing ic-wasm"

    if [ "$1" -eq 1 ]; then
        "$global_kybra_cargo_bin" install cargo-binstall
        "$global_kybra_cargo_bin" install candid-extractor
        # "$global_kybra_cargo_bin" binstall --quiet candid-extractor
        "$global_kybra_cargo_bin" install ic-wasm --version 0.3.6 &> "$global_kybra_logs_dir"/install_ic_wasm
    fi
}

run
