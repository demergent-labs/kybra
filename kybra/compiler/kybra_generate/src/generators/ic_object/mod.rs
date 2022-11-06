use quote::quote;

use crate::generators::ic_object::functions::accept_message::generate_accept_message;
use crate::generators::ic_object::functions::candid_decode::generate_candid_decode;
use crate::generators::ic_object::functions::candid_encode::generate_candid_encode;
use crate::generators::ic_object::functions::canister_balance::generate_canister_balance;
use crate::generators::ic_object::functions::canister_balance128::generate_canister_balance128;
use crate::generators::ic_object::functions::method_name::generate_method_name;
use crate::generators::ic_object::functions::msg_cycles_accept::generate_msg_cycles_accept;
use crate::generators::ic_object::functions::msg_cycles_accept128::generate_msg_cycles_accept128;
use crate::generators::ic_object::functions::msg_cycles_available::generate_msg_cycles_available;
use crate::generators::ic_object::functions::msg_cycles_available128::generate_msg_cycles_available128;
use crate::generators::ic_object::functions::msg_cycles_refunded::generate_msg_cycles_refunded;
use crate::generators::ic_object::functions::msg_cycles_refunded128::generate_msg_cycles_refunded128;
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
use crate::generators::ic_object::functions::trap::generate_trap;

mod functions;

pub fn generate_ic_object() -> proc_macro2::TokenStream {
    let accept_message = generate_accept_message();
    let candid_decode = generate_candid_decode();
    let candid_encode = generate_candid_encode();
    let canister_balance = generate_canister_balance();
    let canister_balance128 = generate_canister_balance128();
    let method_name = generate_method_name();
    let msg_cycles_accept = generate_msg_cycles_accept();
    let msg_cycles_accept128 = generate_msg_cycles_accept128();
    let msg_cycles_available = generate_msg_cycles_available();
    let msg_cycles_available128 = generate_msg_cycles_available128();
    let msg_cycles_refunded = generate_msg_cycles_refunded();
    let msg_cycles_refunded128 = generate_msg_cycles_refunded128();
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
    let trap = generate_trap();

    quote! {
            #[pyclass(module = false, name = "ic")]
            #[derive(Debug, PyPayload)]
            struct Ic {}

        #[pyclass]
        impl Ic {
            #accept_message
            #candid_decode
            #candid_encode
            #canister_balance
            #canister_balance128
            #method_name
            #msg_cycles_accept
            #msg_cycles_accept128
            #msg_cycles_available
            #msg_cycles_available128
            #msg_cycles_refunded
            #msg_cycles_refunded128
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
            #trap
        }

    }
}
