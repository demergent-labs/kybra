use quote::quote;

use crate::generators::ic_object::functions::candid_decode::generate_candid_decode;
use crate::generators::ic_object::functions::candid_encode::generate_candid_encode;
use crate::generators::ic_object::functions::notify_raw::generate_notify_raw;
use crate::generators::ic_object::functions::print::generate_print;
use crate::generators::ic_object::functions::stable64_grow::generate_stable64_grow;
use crate::generators::ic_object::functions::stable64_read::generate_stable64_read;
use crate::generators::ic_object::functions::stable64_size::generate_stable64_size;
use crate::generators::ic_object::functions::stable64_write::generate_stable64_write;
use crate::generators::ic_object::functions::stable_bytes::generate_stable_bytes;
use crate::generators::ic_object::functions::stable_grow::generate_stable_grow;
use crate::generators::ic_object::functions::stable_read::generate_stable_read;
use crate::generators::ic_object::functions::stable_size::generate_stable_size;
use crate::generators::ic_object::functions::stable_write::generate_stable_write;

mod functions;

pub fn generate_ic_object() -> proc_macro2::TokenStream {
    let candid_decode = generate_candid_decode();
    let candid_encode = generate_candid_encode();
    let notify_raw = generate_notify_raw();
    let print = generate_print();
    let stable_bytes = generate_stable_bytes();
    let stable_grow = generate_stable_grow();
    let stable_read = generate_stable_read();
    let stable_size = generate_stable_size();
    let stable_write = generate_stable_write();
    let stable64_grow = generate_stable64_grow();
    let stable64_read = generate_stable64_read();
    let stable64_size = generate_stable64_size();
    let stable64_write = generate_stable64_write();

    quote! {
            #[pyclass(module = false, name = "ic")]
            #[derive(Debug, PyPayload)]
            struct Ic {}

        #[pyclass]
        impl Ic {
            #candid_decode
            #candid_encode
            #notify_raw
            #print
            #stable_bytes
            #stable_grow
            #stable_read
            #stable_size
            #stable_write
            #stable64_grow
            #stable64_read
            #stable64_size
            #stable64_write
        }

    }
}
