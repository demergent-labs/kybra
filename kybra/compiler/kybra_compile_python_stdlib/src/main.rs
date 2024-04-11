use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let python_stdlib_dest_path = &args[1];

    let python_stdlib_modules = rustpython_vm::py_freeze!(dir = "../Lib");
    let python_stdlib_bytecode = python_stdlib_modules.bytes.to_vec();

    fs::write(python_stdlib_dest_path, &python_stdlib_bytecode).unwrap();
}
