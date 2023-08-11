use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        async fn call_global_python_function<'a, T>(
            function_name: &str,
            args: impl _KybraTraitIntoFuncArgs,
        ) -> Result<T, String>
        where
            for<'b> rustpython::vm::PyObjectRef: CdkActTryFromVmValue<
                T,
                rustpython_vm::builtins::PyBaseExceptionRef,
                &'b rustpython::vm::VirtualMachine,
            >,
        {
            let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
                .ok_or_else(|| "SystemError: missing python interpreter".to_string())?;
            let scope = unsafe { SCOPE_OPTION.as_mut() }
                .ok_or_else(|| "SystemError: missing python scope".to_string())?;
            let vm = &interpreter.vm;
            let py_object_ref = scope
                .globals
                .get_item(function_name, vm)
                .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?
                .call(args, vm)
                .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?;

            async_result_handler(vm, &py_object_ref, vm.ctx.none())
                .await
                .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?
                .try_from_vm_value(vm)
                .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))
        }

        fn call_global_python_function_sync<'a, T>(
            function_name: &str,
            args: impl _KybraTraitIntoFuncArgs,
        ) -> Result<T, String>
        where
            for<'b> rustpython::vm::PyObjectRef: CdkActTryFromVmValue<
                T,
                rustpython_vm::builtins::PyBaseExceptionRef,
                &'b rustpython::vm::VirtualMachine,
            >,
        {
            let interpreter = unsafe { INTERPRETER_OPTION.as_mut() }
                .ok_or_else(|| "SystemError: missing python interpreter".to_string())?;
            let scope = unsafe { SCOPE_OPTION.as_mut() }
                .ok_or_else(|| "SystemError: missing python scope".to_string())?;

            interpreter.enter(|vm| {
                scope
                    .globals
                    .get_item(function_name, vm)
                    .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?
                    .call(args, vm)
                    .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))?
                    .try_from_vm_value(vm)
                    .map_err(|py_base_exception| py_base_exception.to_rust_err_string(vm))
            })
        }
    }
}
