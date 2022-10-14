use quote::quote;

use crate::generators::ic_object::functions::print::generate_ic_object_function_print;

mod functions;

pub fn generate_ic_object() -> proc_macro2::TokenStream {
    let print = generate_ic_object_function_print();

    quote! {
        #[pyclass(module = false, name = "ic")]
        #[derive(Debug, PyPayload)]
        struct Ic {}

        #print
    }
}
