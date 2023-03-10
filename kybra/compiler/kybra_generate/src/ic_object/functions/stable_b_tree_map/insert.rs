use proc_macro2::TokenStream;
use quote::quote;

use crate::{stable_b_tree_map_nodes::stable_b_tree_map, StableBTreeMapNode};

pub fn generate(stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>) -> TokenStream {
    let match_arms = generate_match_arms(stable_b_tree_map_nodes);

    quote! {
        #[pymethod]
        fn _kybra_stable_b_tree_map_insert(&self, memory_id_py_object_ref: PyObjectRef, key_py_object_ref: PyObjectRef, value_py_object_ref: PyObjectRef, vm: &VirtualMachine) -> PyObjectRef {
            let memory_id: u8 = memory_id_py_object_ref.try_from_vm_value(vm).unwrap();

            match memory_id {
                #(#match_arms),*
                _ => panic!("memory_id {} does not have an associated StableBTreeMap", memory_id)
            }
        }
    }
}

fn generate_match_arms(stable_b_tree_map_nodes: &Vec<StableBTreeMapNode>) -> Vec<TokenStream> {
    stable_b_tree_map_nodes
        .iter()
        .map(|stable_b_tree_map_node| {
            let memory_id = stable_b_tree_map_node.memory_id;
            let stable_b_tree_map_ref_cell = stable_b_tree_map::ref_cell_ident::generate(stable_b_tree_map_node.memory_id);

            let (key_wrapper_type_name, _) = stable_b_tree_map::wrapper_type::generate(&stable_b_tree_map_node.key_type, memory_id, "Key");
            let (value_wrapper_type_name, _) = stable_b_tree_map::wrapper_type::generate(&stable_b_tree_map_node.value_type, memory_id, "Value");

            // TODO the return value here might need a little work like in get
            quote! {
                #memory_id => {
                    let key = #key_wrapper_type_name(key_py_object_ref.try_from_vm_value(vm).unwrap());
                    let value = #value_wrapper_type_name(value_py_object_ref.try_from_vm_value(vm).unwrap());

                    let result = #stable_b_tree_map_ref_cell.with(|stable_b_tree_map_ref_cell| {
                        stable_b_tree_map_ref_cell
                            .borrow_mut()
                            .insert(key, value)
                    });

                    let insert_result_class = _kybra_unwrap_rust_python_result(vm.run_block_expr(
                    vm.new_scope_with_builtins(),
                        r#"
from kybra import InsertResult

InsertResult
                        "#
                    ), vm);

                    match result {
                        Ok(ok) => {
                            let method_result = vm.invoke(&insert_result_class, (ok.try_into_vm_value(vm).unwrap(), vm.ctx.none()));

                            _kybra_unwrap_rust_python_result(method_result, vm)

                            // TODO Consider using dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                            // let dict = vm.ctx.new_dict();

                            // dict.set_item("ok", ok.try_into_vm_value(vm).unwrap(), vm);

                            // dict
                        },
                        Err(err) => {
                            let method_result = vm.invoke(&insert_result_class, (vm.ctx.none(), err.try_into_vm_value(vm).unwrap()));
                            _kybra_unwrap_rust_python_result(method_result, vm)

                            // TODO Consider using dict once we are on Python 3.11: https://github.com/python/cpython/issues/89026
                            // let dict = vm.ctx.new_dict();

                            // dict.set_item("err", err_string.try_into_vm_value(vm).unwrap(), vm);

                            // dict
                        }
                    }
                }
            }
        })
        .collect()
}
