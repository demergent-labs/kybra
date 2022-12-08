mod derive_try_into_vm_value {
    pub mod derive_try_into_vm_value_enum;
    pub mod derive_try_into_vm_value_struct;
}
mod derive_try_from_vm_value {
    pub mod derive_try_from_vm_value_enum;
    pub mod derive_try_from_vm_value_struct;
}

use derive_try_from_vm_value::{
    derive_try_from_vm_value_enum::derive_try_from_vm_value_enum,
    derive_try_from_vm_value_struct::derive_try_from_vm_value_struct,
};
use derive_try_into_vm_value::{
    derive_try_into_vm_value_enum::derive_try_into_vm_value_enum,
    derive_try_into_vm_value_struct::derive_try_into_vm_value_struct,
};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(CdkActTryIntoVmValue)]
pub fn derive_kybra_try_into_vm_value(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;

    let generated_code = match input.data {
        Data::Enum(data_enum) => derive_try_into_vm_value_enum(&name, &data_enum),
        Data::Struct(data_struct) => derive_try_into_vm_value_struct(&name, &data_struct),
        _ => panic!("Can only derive from Structs or Enums"),
    };

    TokenStream::from(generated_code)
}

#[proc_macro_derive(CdkActTryFromVmValue)]
pub fn derive_kybra_try_from_vm_value(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;

    let generated_code = match input.data {
        Data::Enum(data_enum) => derive_try_from_vm_value_enum(&name, &data_enum),
        Data::Struct(data_struct) => derive_try_from_vm_value_struct(&name, &data_struct),
        _ => panic!("Can only derive from Structs or Enums"),
    };

    TokenStream::from(generated_code)
}

const PYTHON_KEYWORDS: [&str; 35] = [
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

fn get_python_keywords() -> Vec<String> {
    PYTHON_KEYWORDS
        .iter()
        .map(|keyword| keyword.to_string())
        .collect()
}
