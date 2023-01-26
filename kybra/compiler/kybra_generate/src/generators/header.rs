pub fn generate() -> proc_macro2::TokenStream {
    quote::quote! {
        #![allow(warnings, unused)]

        use rustpython_vm::{AsObject, builtins::{PyDict, PyBaseException, PyGenerator, PyListRef, PyTupleRef, PyIntRef, PyStr, PyList, PyTuple, PyBytes}, class::PyClassImpl, convert::ToPyObject, function::IntoFuncArgs, PyObjectRef, PyObject, PyRef, VirtualMachine, protocol::{PyIter, PyIterReturn}, py_serde::{deserialize, serialize}};
        use rustpython_derive::{pyclass, PyPayload};
        use kybra_vm_value_derive::{CdkActTryIntoVmValue, CdkActTryFromVmValue};
        use std::str::FromStr;
        use ic_cdk::api::call::CallResult;
        use serde::de::{DeserializeSeed, Visitor};
        use serde::ser::{Serialize, SerializeMap, SerializeSeq, SerializeTuple};
        use slotmap::Key;
        use rand::{Rng, SeedableRng, rngs::StdRng};
        use std::convert::TryInto;

        thread_local! {
            static RNG_REF_CELL: std::cell::RefCell<StdRng> = std::cell::RefCell::new(SeedableRng::from_seed([0u8; 32]));
        }

        static mut _KYBRA_INTERPRETER_OPTION: Option<rustpython_vm::Interpreter> = None;
        static mut _KYBRA_SCOPE_OPTION: Option<rustpython_vm::scope::Scope> = None;

        // TODO this is broken https://github.com/dfinity/motoko/issues/3462#issuecomment-1260060874
        // #[link_section = "icp:public cdk"]
        // pub static NAME: [u8; 12] = *b"kybra v0.0.0";
    }
}
