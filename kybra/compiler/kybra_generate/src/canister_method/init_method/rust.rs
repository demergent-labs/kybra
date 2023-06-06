// TODO currently init is never called because of our deployer infrastructure
// TODO in the future once dfx can automatically chunk upload any size binary
// TODO the hope is that we can use the regular init flow again
// TODO init should look almost identical to post_upgrade
// TODO but it should call the developer's init function defined in Python

use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {}
}
