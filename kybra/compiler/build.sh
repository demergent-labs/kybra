#!/usr/bin/env bash

# This causes the script to exit with a non-zero exit code if any command exits with a non-zero exit code
set -e

# This is the name of the canister passed into python -m kybra from the dfx.json build command
CANISTER_NAME=$1

# This is the path to the developer's entry point Python file passed into python -m kybra from the dfx.json build command
PY_ENTRY_FILE_PATH=$2

# This is root directory of the developer's Python project, derived from the entry point Python file passed into python -m kybra from the dfx.json build command
PY_ENTRY_DIR_PATH=$(dirname $PY_ENTRY_FILE_PATH)

# This is the Python module name of the developer's Python project, derived from the entry point Python file passed into python -m kybra from the dfx.json build command
PY_ENTRY_MODULE_NAME=$(basename $PY_ENTRY_FILE_PATH .py)

# This is the path to the developer's Candid file passed into python -m kybra from the dfx.json build command
DID_PATH=$3

# This is the path to the Kybra compiler Rust code delivered with the Python package
COMPILER_PATH=$4

# This is the location of all code used to generate the final canister Rust code
CANISTER_PATH=.dfx/kybra/$CANISTER_NAME

# This is the final generated Rust file that is the canister
LIB_PATH=$CANISTER_PATH/src/lib.rs

# This is the location of the Candid file generated from the final generated Rust file
GENERATED_DID_PATH=$CANISTER_PATH/main.did

# This is the Rust target directory
TARGET_PATH=$CANISTER_PATH/target

# The location that Kybra will look to when running py_freeze!
# py_freeze! will compile all of the Python code in the directory recursively (modules must have an __init__.py to be included)
PY_FREEZE_PATH=$CANISTER_PATH/python_source

rustup target add wasm32-unknown-unknown
cargo install ic-cdk-optimizer --version 0.3.4

# Generate the Rust code
CARGO_TARGET_DIR=$TARGET_PATH cargo run --manifest-path $CANISTER_PATH/kybra_generate/Cargo.toml $PY_ENTRY_FILE_PATH $PY_ENTRY_MODULE_NAME | rustfmt --edition 2018 > $LIB_PATH

# Compile the generated Rust code
CARGO_TARGET_DIR=$TARGET_PATH cargo build --manifest-path $CANISTER_PATH/Cargo.toml --target wasm32-unknown-unknown --package kybra_generated_canister --release

# Generate the Candid file
CARGO_TARGET_DIR=$TARGET_PATH cargo test --manifest-path $CANISTER_PATH/Cargo.toml

# Copy the generated Candid file to the developer's source directory
cp $GENERATED_DID_PATH $DID_PATH

# TODO we need to do this the way that Azle does it so it works well on M1s and other environments: https://github.com/demergent-labs/azle/blob/main/src/azle.ts#L138

# Optimize the Wasm binary
# TODO this should eventually be replaced with ic-wasm once this is resolved: https://forum.dfinity.org/t/wasm-module-contains-a-function-that-is-too-complex/15407/22
# ic-wasm $TARGET_PATH/wasm32-unknown-unknown/release/kybra_generated_canister.wasm -o $TARGET_PATH/wasm32-unknown-unknown/release/$CANISTER_NAME.wasm shrink
ic-cdk-optimizer $TARGET_PATH/wasm32-unknown-unknown/release/kybra_generated_canister.wasm -o $TARGET_PATH/wasm32-unknown-unknown/release/$CANISTER_NAME.wasm

# gzip the Wasm binary
gzip -f -k $TARGET_PATH/wasm32-unknown-unknown/release/$CANISTER_NAME.wasm
