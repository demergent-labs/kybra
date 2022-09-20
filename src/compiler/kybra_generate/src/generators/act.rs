use crate::cdk_act::{
    act::AbstractCanisterTree,
    act_node::{ActNode, PrimitiveInfo},
    nodes::canister_method::{CanisterMethod, CanisterMethodActNode},
};

pub fn generate_act(python_source: &str) -> AbstractCanisterTree {
    let test_query_method = CanisterMethod {
        body: quote::quote! {
            unsafe {
                let _kybra_interpreter = _KYBRA_INTERPRETER_OPTION.as_mut().unwrap();
                let _kybra_scope = _KYBRA_SCOPE_OPTION.as_mut().unwrap();

                let result = _kybra_interpreter.enter(|vm| {
                    let hello_world_py_object_ref = _kybra_scope.globals.get_item("test", vm).unwrap();

                    // let result_py_object_ref = vm.invoke(&hello_world_py_object_ref, (x.try_into_vm_value(vm).unwrap(), y.try_into_vm_value(vm).unwrap())).unwrap();
                    let result_py_object_ref = vm.invoke(&hello_world_py_object_ref, ()).unwrap();

                    result_py_object_ref.try_from_vm_value(vm).unwrap()
                });

                result
            }
        },
        param_names: vec![],
        param_types: vec![],
        inline_types: Box::new(vec![]),
        is_manual: false,
        name: "test".to_string(),
        return_type: ActNode::Primitive(PrimitiveInfo {
            identifier: quote::quote!(bool),
        }),
    };

    let test_query_method_act_node = CanisterMethodActNode::QueryMethod(test_query_method);

    AbstractCanisterTree {
        query_methods: vec![test_query_method_act_node],
        update_methods: vec![],
    }
}
