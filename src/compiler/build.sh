#!/usr/bin/env bash

# TODO Find a better way to get the path to the Kybra code...really we want something like npx kybra, pip kybra
# COMPILER_PATH=.dfx/kybra/venv/lib/python3.8/site-packages/compiler
COMPILER_PATH=$(pip show kybra | grep "Location:" | cut -d " " -f2)/compiler
CANISTER_PATH=.dfx/kybra/$1
TARGET_PATH=$CANISTER_PATH/target

cp -a $COMPILER_PATH/. $CANISTER_PATH

CARGO_TARGET_DIR=$TARGET_PATH cargo run --manifest-path $CANISTER_PATH/kybra_generate/Cargo.toml $2 | rustfmt > $CANISTER_PATH/src/lib.rs
CARGO_TARGET_DIR=$TARGET_PATH cargo build --manifest-path $CANISTER_PATH/Cargo.toml --target wasm32-unknown-unknown --package kybra_generated_canister --release
ic-cdk-optimizer $TARGET_PATH/wasm32-unknown-unknown/release/kybra_generated_canister.wasm -o $TARGET_PATH/wasm32-unknown-unknown/release/$1.wasm
gzip -f -k $TARGET_PATH/wasm32-unknown-unknown/release/$1.wasm
