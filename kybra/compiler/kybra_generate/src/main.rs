use std::{env, fs, fs::File, io::Write, process};

use kybra_generate::generate_canister;

mod exit_codes;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        let executable_name = &args[0];
        eprintln!("Usage: {executable_name} <path/to/py_file_names.csv> <path/to/main.py> <path/to/output/lib.rs>");
        process::exit(exit_codes::USAGE_ERROR);
    }

    let py_file_names_path = &args[1];
    let py_entry_module_name = &args[2];
    let output_file_path = &args[3];

    let py_file_names_string = fs::read_to_string(py_file_names_path).unwrap_or_else(|err| {
        eprintln!("Error reading {py_file_names_path}: {err}");
        process::exit(exit_codes::PY_FILE_NAMES_READ_ERROR);
    });
    let py_file_names: Vec<&str> = py_file_names_string.split(",").collect();

    let lib_file = generate_canister(&py_file_names, py_entry_module_name)
        .unwrap_or_else(|errors| {
            eprintln!("Canister compilation failed:");
            for error in errors {
                eprintln!("{}", error)
            }
            process::exit(exit_codes::CANISTER_COMPILATION_ERROR);
        })
        .to_string();

    let mut f = File::create(output_file_path).unwrap_or_else(|err| {
        eprintln!("Error creating file {output_file_path}: {err}");
        process::exit(exit_codes::LIB_FILE_CREATE_ERROR);
    });
    f.write_all(lib_file.as_bytes()).unwrap_or_else(|err| {
        eprintln!("Error writing to file {output_file_path}: {err}");
        process::exit(exit_codes::LIB_FILE_WRITE_ERROR);
    });
}
