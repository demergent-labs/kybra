use kybra_generate::kybra_generate;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let py_file_names_path = &args[1];
    let py_entry_module_name = &args[2];

    let py_file_names_string = std::fs::read_to_string(py_file_names_path).unwrap();
    let py_file_names: Vec<&str> = py_file_names_string.split(",").collect();

    let result = kybra_generate(&py_file_names, &py_entry_module_name);

    println!("{}", result);
}
