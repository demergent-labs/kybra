use cdk_framework::act::node::{
    candid::Service,
    canister_method::{QueryMethod, UpdateMethod},
};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    ic_object::functions::{
        accept_message, arg_data_raw, arg_data_raw_size, caller, candid_decode, candid_encode,
        canister_balance, canister_balance128, clear_timer, data_certificate, id, method_name,
        msg_cycles_accept, msg_cycles_accept128, msg_cycles_available, msg_cycles_available128,
        msg_cycles_refunded, msg_cycles_refunded128, notify_functions, notify_raw,
        notify_with_payment128_functions, performance_counter, print, reject, reject_code,
        reject_message, reply, reply_raw, set_certified_data, set_timer, set_timer_interval,
        stable64_grow, stable64_read, stable64_size, stable64_write, stable_b_tree_map,
        stable_bytes, stable_grow, stable_read, stable_size, stable_write, time, trap,
    },
    StableBTreeMapNode,
};

mod functions;

pub fn generate(
    update_methods: &Vec<UpdateMethod>,
    query_methods: &Vec<QueryMethod>,
    services: &Vec<Service>,
    stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>,
) -> TokenStream {
    let accept_message = accept_message::generate();
    let arg_data_raw = arg_data_raw::generate();
    let arg_data_raw_size = arg_data_raw_size::generate();
    let caller = caller::generate();
    let candid_decode = candid_decode::generate();
    let candid_encode = candid_encode::generate();
    let canister_balance = canister_balance::generate();
    let canister_balance128 = canister_balance128::generate();
    let clear_timer = clear_timer::generate();
    let data_certificate = data_certificate::generate();
    let id = id::generate();
    let method_name = method_name::generate();
    let msg_cycles_accept = msg_cycles_accept::generate();
    let msg_cycles_accept128 = msg_cycles_accept128::generate();
    let msg_cycles_available = msg_cycles_available::generate();
    let msg_cycles_available128 = msg_cycles_available128::generate();
    let msg_cycles_refunded = msg_cycles_refunded::generate();
    let msg_cycles_refunded128 = msg_cycles_refunded128::generate();
    let notify_functions = notify_functions::generate(services);
    let notify_raw = notify_raw::generate();
    let notify_with_payment128_functions = notify_with_payment128_functions::generate(services);
    let performance_counter = performance_counter::generate();
    let print = print::generate();
    let reject = reject::generate();
    let reject_code = reject_code::generate();
    let reject_message = reject_message::generate();
    let reply = reply::generate(update_methods, query_methods);
    let reply_raw = reply_raw::generate();
    let set_certified_data = set_certified_data::generate();
    let set_timer = set_timer::generate();
    let set_timer_interval = set_timer_interval::generate();
    let stable_bytes = stable_bytes::generate();
    let stable_grow = stable_grow::generate();
    let stable_read = stable_read::generate();
    let stable_size = stable_size::generate();
    let stable_write = stable_write::generate();
    let stable_b_tree_map = stable_b_tree_map::generate(stable_b_tree_map_nodes);
    let stable64_grow = stable64_grow::generate();
    let stable64_read = stable64_read::generate();
    let stable64_size = stable64_size::generate();
    let stable64_write = stable64_write::generate();
    let time = time::generate();
    let trap = trap::generate();

    quote! {
        #[rustpython_derive::pyclass(module = false, name = "ic")]
        #[derive(Debug, rustpython_derive::PyPayload)]
        struct Ic {}

        #[rustpython_derive::pyclass]
        impl Ic {
            #accept_message
            #arg_data_raw
            #arg_data_raw_size
            #caller
            #candid_decode
            #candid_encode
            #canister_balance
            #canister_balance128
            #clear_timer
            #data_certificate
            #id
            #method_name
            #msg_cycles_accept
            #msg_cycles_accept128
            #msg_cycles_available
            #msg_cycles_available128
            #msg_cycles_refunded
            #msg_cycles_refunded128
            #(#notify_functions)*
            #notify_raw
            #(#notify_with_payment128_functions)*
            #performance_counter
            #print
            #reject
            #reject_code
            #reject_message
            #reply
            #reply_raw
            #set_certified_data
            #set_timer
            #set_timer_interval
            #stable_bytes
            #stable_grow
            #stable_read
            #stable_size
            #stable_write
            #stable_b_tree_map
            #stable64_grow
            #stable64_read
            #stable64_size
            #stable64_write
            #time
            #trap
        }
    }
}
