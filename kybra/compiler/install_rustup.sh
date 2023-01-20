#!/bin/bash

# TODO If we want to save a little bit of time we might be able to instruct rustup to not install some components initially, like clippy and docs
# TODO we might want to implement all of this in Python in the future for platform-independence etc

kybra_version=$1
rust_version=$2

global_kybra_config_dir=~/.config/kybra/$kybra_version
global_kybra_bin_dir=$global_kybra_config_dir/bin
global_kybra_cargo_bin=$global_kybra_bin_dir/cargo
global_kybra_logs_dir=$global_kybra_config_dir/logs
global_kybra_rustup_bin=$global_kybra_bin_dir/rustup
global_kybra_target_dir=$global_kybra_config_dir/target

export CARGO_HOME=$global_kybra_config_dir
export RUSTUP_HOME=$global_kybra_config_dir

function install_wasm32_unknown_unknown() {
    $global_kybra_rustup_bin target add wasm32-unknown-unknown &> $global_kybra_logs_dir/step_2_wasm32_unknown_unknown
}

function install_ic_wasm() {
    $global_kybra_cargo_bin install ic-wasm --version 0.3.0 &> $global_kybra_logs_dir/step_3_ic_wasm
}

function install_ic_cdk_optimizer() {
    $global_kybra_cargo_bin install ic-cdk-optimizer --version 0.3.4  &> $global_kybra_logs_dir/step_4_ic_cdk_optimizer
}

if [ -x "$(command -v $global_kybra_rustup_bin)" ]; then
    $global_kybra_rustup_bin update $rust_version &> $global_kybra_logs_dir/rustup_update
else
    echo -e "\nKybra $kybra_version initial installation (this takes a few minutes)\n"

    mkdir -p ~/.config/kybra
    mkdir -p $global_kybra_config_dir
    mkdir -p $global_kybra_logs_dir

    echo -e "1/4) Installing Rust"

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --no-modify-path -y --default-toolchain=$rust_version &> $global_kybra_logs_dir/step_1_rust

    echo -e "2/4) Installing wasm32-unknown-unknown"

    install_wasm32_unknown_unknown

    echo -e "3/4) Installing ic-wasm"

    install_ic_wasm

    echo -e "4/4) Installing ic-cdk-optimizer\n"

    install_ic_cdk_optimizer
fi

if ! $global_kybra_rustup_bin target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo -e "\nKybra $kybra_version is installing wasm32-unknown-unknown\n"

    install_wasm32_unknown_unknown
fi

if ! test -e $global_kybra_bin_dir/ic-wasm; then
    echo -e "\nKybra $kybra_version is installing ic-wasm\n"

    install_ic_wasm
fi

if ! test -e $global_kybra_bin_dir/ic-cdk-optimizer; then
    echo -e "\nKybra $kybra_version is installing ic-cdk-optimizer\n"

    install_ic_cdk_optimizer
fi
