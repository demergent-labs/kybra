use kybra_generate::kybra_generate;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let py_entry_path = &args[1];
    let py_entry_module_name = &args[2];

    let py_entry_source = std::fs::read_to_string(py_entry_path).unwrap();

    let result = kybra_generate(&py_entry_source, &py_entry_module_name);

    println!("{}", result);
}
