use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        pub trait UnwrapOrTrapWithMessage<T> {
            fn unwrap_or_trap(self, err_message: &str) -> T;
        }

        impl<T> UnwrapOrTrapWithMessage<T> for Option<T> {
            fn unwrap_or_trap(self, err_message: &str) -> T {
                match self {
                    Some(some) => some,
                    None => {
                        ic_cdk::trap(err_message)
                    }
                }
            }
        }

        pub trait UnwrapOrTrap<T> {
            fn unwrap_or_trap(self) -> T;
        }

        impl<T> UnwrapOrTrap<T> for Result<T, CdkActTryIntoVmValueError> {
            fn unwrap_or_trap(self) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => ic_cdk::trap(&err.0)
                }
            }
        }

        impl<T> UnwrapOrTrap<T> for Result<T, ic_stable_structures::cell::ValueError> {
            fn unwrap_or_trap(self) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => ic_cdk::trap(&match err {
                        ic_stable_structures::cell::ValueError::ValueTooLarge { value_size } => {
                            format!("ValueError: ValueTooLarge {value_size}")
                        }
                    }),
                }
            }
        }

        impl<T> UnwrapOrTrap<T> for Result<T, ic_stable_structures::cell::InitError> {
            fn unwrap_or_trap(self) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => ic_cdk::trap(&init_error_to_string(&err)),
                }
            }
        }

        impl<T> UnwrapOrTrap<T> for candid::Result<T> {
            fn unwrap_or_trap(self) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => ic_cdk::trap(&format!("CandidError: {}", err.to_string())),
                }
            }
        }

        impl<T> UnwrapOrTrap<T> for Result<T, String> {
            fn unwrap_or_trap(self) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => ic_cdk::trap(&err),
                }
            }
        }

        pub trait UnwrapOrTrapWithVm<T> {
            fn unwrap_or_trap(self, vm: &rustpython::vm::VirtualMachine) -> T;
        }

        impl<T> UnwrapOrTrapWithVm<T> for Result<T, rustpython::vm::PyRef<rustpython_vm::builtins::PyBaseException>> {
            fn unwrap_or_trap(self, vm: &rustpython::vm::VirtualMachine) -> T {
                match self {
                    Ok(ok) => ok,
                    Err(err) => {
                        let py_object = err.to_pyobject(vm);
                        let type_name = py_object.class().name().to_string();
                        let err_message = match py_object.str(vm) {
                            Ok(str) => str,
                            Err(_) => ic_cdk::trap(
                                format!("Attribute Error: '{type_name}' object has no attribute '__str__'").as_str()
                            ),
                        };
                        ic_cdk::trap(format!("{type_name}: {err_message}").as_str())
                    }
                }
            }
        }

        fn init_error_to_string(err: &ic_stable_structures::cell::InitError) -> String {
            match err {
                ic_stable_structures::cell::InitError::IncompatibleVersion {
                    last_supported_version,
                    decoded_version,
                } => format!("InitError: IncompatibleVersion, last_supported_version {last_supported_version}, decoded_version {decoded_version}"),
                ic_stable_structures::cell::InitError::ValueTooLarge { value_size } => {
                    format!("InitError: ValueTooLarge {value_size}")
                }
            }
        }
    }
}
