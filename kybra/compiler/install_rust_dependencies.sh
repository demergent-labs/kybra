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
global_kybra_wasi2ic_bin="$global_kybra_rust_bin_dir"/wasi2ic
global_kybra_rustc_bin="$global_kybra_rust_bin_dir"/rustc

export CARGO_TARGET_DIR="$global_kybra_config_dir"/rust/target
export CARGO_HOME="$global_kybra_rust_dir"
export RUSTUP_HOME="$global_kybra_rust_dir"

function run() {
    if ! ([ -e "$global_kybra_rustup_bin" ] && [ -e "$global_kybra_wasi2ic_bin" ] && [ -e "$global_kybra_cargo_bin" ] && [ -e "$global_kybra_rustc_bin" ] && [ -e "$global_kybra_version_dir"/RustPython ] && $global_kybra_rustup_bin target list | grep -q "wasm32-wasi (installed)"); then
        echo -e "\nKybra "$kybra_version" prerequisite installation (this may take a few minutes)\n"

        mkdir -p "$global_kybra_version_dir"
        mkdir -p "$global_kybra_rust_dir"
        mkdir -p "$global_kybra_logs_dir"

        install_rustup
        install_wasm32
        install_wasi2ic
        install_rust_python
        install_candid_extractor
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
}

function install_wasi2ic() {
    echo -e "3/5) Installing wasi2ic"

    "$global_kybra_cargo_bin" install --git https://github.com/wasm-forge/wasi2ic --rev 7418e0bd1a7810c8e9c55cc0155c921503a793b8 &> "$global_kybra_logs_dir"/install_wasi2ic
}

function install_rust_python() {
    echo -e "4/5) Installing RustPython"

    cd "$global_kybra_version_dir"
    git clone https://github.com/RustPython/RustPython.git
    cd RustPython
    git checkout f12875027ce425297c07cbccb9be77514ed46157
    cd -
    cd -
}

function install_candid_extractor() {
    echo -e "5/5) Installing candid-extractor"

    "$global_kybra_cargo_bin" install candid-extractor
}

run
