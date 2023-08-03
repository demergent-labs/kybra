use proc_macro2::TokenStream;
use quote::quote;

use crate::{stable_b_tree_map_nodes::rust, StableBTreeMapNode};

pub fn generate(stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>) -> TokenStream {
    let match_arms = generate_match_arms(stable_b_tree_map_nodes);

    quote! {
        #[pymethod]
        fn stable_b_tree_map_items(
            &self,
            memory_id_py_object_ref: rustpython_vm::PyObjectRef,
            vm: &rustpython_vm::VirtualMachine
        ) -> rustpython_vm::PyResult {
            let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm)?;

            match memory_id {
                #(#match_arms)*
                // TODO: Consider creating a custom error or using
                // IndexError, KeyError, or ValueError
                _ => Err(vm.new_lookup_error(format!(
                    "memory_id {} does not have an associated StableBTreeMap",
                    memory_id
                )))
            }
        }
    }
}

fn generate_match_arms(stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>) -> Vec<TokenStream> {
    stable_b_tree_map_nodes
        .iter()
        .map(|stable_b_tree_map_node| {
            let memory_id = stable_b_tree_map_node.memory_id;
            let stable_b_tree_map_ref_cell =
                rust::ref_cell_ident::generate(stable_b_tree_map_node.memory_id);

            quote! {
                #memory_id => {
                    #stable_b_tree_map_ref_cell
                        .with(|map_ref_cell| {
                            let (key_value_pairs, type_errors) = map_ref_cell
                                .borrow()
                                .iter()
                                .map(|(key_wrapper_type, value_wrapper_type)| -> Result<
                                    rustpython_vm::PyObjectRef,
                                    rustpython_vm::builtins::PyBaseExceptionRef
                                > {
                                    let key = key_wrapper_type.0
                                        .try_into_vm_value(vm)
                                        .map_err(|vmc_err| vm.new_type_error(vmc_err.0))?;

                                    let value = value_wrapper_type.0
                                        .try_into_vm_value(vm)
                                        .map_err(|vmc_err| vm.new_type_error(vmc_err.0))?;

                                    Ok(vm.ctx.new_tuple(vec![key, value]).into())
                                })
                                .fold((vec![], vec![]), |mut acc, result| {
                                    match result {
                                        Ok(key_value_pair) => acc.0.push(key_value_pair),
                                        Err(type_error) => acc.1.push(type_error),
                                    }
                                    acc
                                });

                            if type_errors.is_empty() {
                                return Ok(vm.ctx.new_list(key_value_pairs).into());
                            }

                            Err(type_errors[0].clone())
                        })
                }
            }
        })
        .collect()
}
