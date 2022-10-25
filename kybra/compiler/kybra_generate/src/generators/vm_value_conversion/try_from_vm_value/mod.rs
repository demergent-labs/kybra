mod basic;
mod generic;
mod numeric;
mod vec;

use basic::generate_basic_impls;
use generic::generate_generic_impls;
use numeric::generate_numeric_impls;
use vec::generate_vec_impls;

pub fn generate_try_from_vm_value_impls() -> proc_macro2::TokenStream {
    let basic_impls = generate_basic_impls();
    let generic_impls = generate_generic_impls();
    let numeric_impls = generate_numeric_impls();
    let vec_impls = generate_vec_impls();

    quote::quote! {
        #basic_impls
        #generic_impls
        #numeric_impls
        #vec_impls
    }
}
