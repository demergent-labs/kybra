use proc_macro2::TokenStream;

// pub struct VirtualMachine {}
// #[derive(Clone)]
// struct PyBaseException {}

// impl PyBaseException {
//     fn to_pyobject(&self, vm: &VirtualMachine) -> PyObject {
//         todo!()
//     }
// }

// struct PyObject {}

// impl PyObject {
//     fn class(&self) -> Class {
//         todo!()
//     }
//     fn str(&self, vm: &VirtualMachine) -> Result<String, ()> {
//         todo!()
//     }
// }

// struct Class {}

// impl Class {
//     fn name(&self) -> String {
//         todo!()
//     }
// }

// fn trap(message: &str) -> ! {
//     panic!("{}", message.to_string())
// }

pub fn generate() -> TokenStream {
    quote::quote! {
        pub trait UnwrapOrTrapWithMessage<T> {
            fn unwrap_or_trap(self, err_message: &str) -> T;
        }

        impl<T> UnwrapOrTrapWithMessage<T> for Option<T> {
            fn unwrap_or_trap(self, err_message: &str) -> T {
                match self {
                    Some(some) => return some,
                    None => {
                        ic_cdk::trap(err_message)
                    }
                }
            }
        }

        pub trait UnwrapOrTrap<T> {
            fn unwrap_or_trap(self, vm: &rustpython::vm::VirtualMachine, err_message: Option<&str>) -> T;
        }

        impl<T> UnwrapOrTrap<T> for Result<T, rustpython::vm::PyRef<rustpython_vm::builtins::PyBaseException>> {
            fn unwrap_or_trap(self, vm: &rustpython::vm::VirtualMachine, err_message: Option<&str>) -> T {
                match self {
                    Ok(ok) => return ok,
                    Err(err) => {
                        let type_name = err.clone().to_pyobject(vm).class().name().to_string();
                        let err_message = match &err.to_pyobject(vm).str(vm) {
                            Ok(string) => string.to_string(),
                            Err(_) => ic_cdk::trap(
                                format!(
                                    "Attribute Error: '{}' object has no attribute '__str__'",
                                    type_name
                                )
                                .as_str(),
                            ),
                        };
                        ic_cdk::trap(format!("{type_name}: {err_message}").as_str())
                    }
                }
            }
        }

        // pub trait UnwrapOrTrap<T> {
        //     fn unwrap_or_trap(&self, vm: &rustpython::vm::VirtualMachine, err_message: Option<&str>) -> T;
        // }

        // impl<T> UnwrapOrTrap for Result<T, rustpython_vm::PyRef<rustpython_vm::builtins::PyBaseException>> {
        //     fn unwrap_or_trap(self, vm: &rustpython::vm::VirtualMachine, err_message: Option<&str>) -> T {
        //         match self {
        //             Ok(ok) => return ok,
        //             Err(err) => {
        //                 let type_name = err.clone().to_pyobject(vm).class().name().to_string();
        //                 let err_message = match &err.to_pyobject(vm).str(vm) {
        //                     Ok(string) => string.to_string(),
        //                     Err(_) => ic_cdk::trap(
        //                         format!(
        //                             "Attribute Error: '{}' object has no attribute '__str__'",
        //                             type_name
        //                         )
        //                         .as_str(),
        //                     ),
        //                 };
        //                 ic_cdk::trap(format!("{type_name}: {err_message}").as_str())
        //             }
        //         }
        //     }
        // }

        // pub fn unwrap_rust_python_result<T>(
        //     rust_python_result: Result<T, rustpython_vm::PyRef<rustpython_vm::builtins::PyBaseException>>,
        //     vm: &rustpython::vm::VirtualMachine
        // ) -> T {
        //     rust_python_result.unwrap_or_trap(vm, None)
        //     // match rust_python_result {
        //     //     Ok(ok) => return ok,
        //     //     Err(err) => {
        //     //         let type_name = err.clone().to_pyobject(vm).class().name().to_string();
        //     //         let err_message = match &err.to_pyobject(vm).str(vm) {
        //     //             Ok(string) => string.to_string(),
        //     //             Err(_) => ic_cdk::trap(format!("Attribute Error: '{}' object has no attribute '__str__'", type_name).as_str()),
        //     //         };
        //     //         ic_cdk::trap(format!("{type_name}: {err_message}").as_str());
        //     //     },
        //     // }
        // }
    }
}
