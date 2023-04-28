# install cargo expand
RUSTUP_HOME=~/.config/kybra/1.68.2 CARGO_HOME=~/.config/kybra/1.68.2 CARGO_TARGET_DIR=~/.config/kybra/target/ ~/.config/kybra/1.68.2/bin/cargo install cargo-expand

# install the nightly compiler
RUSTUP_HOME=~/.config/kybra/1.68.2 CARGO_HOME=~/.config/kybra/1.68.2 CARGO_TARGET_DIR=~/.config/kybra/target/ ~/.config/kybra/1.68.2/bin/rustup default nightly

# revert to the stable compiler
RUSTUP_HOME=~/.config/kybra/1.68.2 CARGO_HOME=~/.config/kybra/1.68.2 CARGO_TARGET_DIR=~/.config/kybra/target/ ~/.config/kybra/1.68.2/bin/rustup default stable

# install wasm32-unknown-unknown
RUSTUP_HOME=~/.config/kybra/1.68.2 CARGO_HOME=~/.config/kybra/1.68.2 CARGO_TARGET_DIR=~/.config/kybra/target/ ~/.config/kybra/1.68.2/bin/rustup target add wasm32-unknown-unknown

# run cargo expand
RUSTUP_HOME=~/.config/kybra/1.68.2 CARGO_HOME=~/.config/kybra/1.68.2 CARGO_TARGET_DIR=~/.config/kybra/target/ ~/.config/kybra/1.68.2/bin/cargo expand --target wasm32-unknown-unknown