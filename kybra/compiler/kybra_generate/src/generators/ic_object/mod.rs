use quote::quote;

use crate::generators::ic_object::functions::candid_decode::generate_ic_object_function_candid_decode;
use crate::generators::ic_object::functions::candid_encode::generate_ic_object_function_candid_encode;
use crate::generators::ic_object::functions::print::generate_ic_object_function_print;

mod functions;

pub fn generate_ic_object() -> proc_macro2::TokenStream {
    let candid_decode = generate_ic_object_function_candid_decode();
    let candid_encode = generate_ic_object_function_candid_encode();
    let print = generate_ic_object_function_print();

    quote! {
        #[pyclass(module = false, name = "ic")]
        #[derive(Debug, PyPayload)]
        struct Ic {}

        #[pyclass]
        impl Ic {
            #candid_decode
            #candid_encode
            #print
        }

    }
}
