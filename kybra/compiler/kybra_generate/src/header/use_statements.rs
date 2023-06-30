use proc_macro2::TokenStream;

pub fn generate() -> TokenStream {
    quote::quote! {
        use candid::{Decode, Encode};
        use kybra_vm_value_derive::{CdkActTryIntoVmValue, CdkActTryFromVmValue};
        use rustpython_vm::{
            class::PyClassImpl as _KybraTraitPyClassImpl,
            convert::ToPyObject as _KybraTraitToPyObject,
            function::IntoFuncArgs as _KybraTraitIntoFuncArgs,
            AsObject as _KybraTraitAsObject,
            TryFromObject as _KybraTraitTryFromObject
        };
        use serde::{
            de::{
                DeserializeSeed as _KybraTraitDeserializeSeed,
                Visitor as _KybraTraitVisitor
            },
            ser::{
                Serialize as _KybraTraitSerialize,
                SerializeMap as _KybraTraitSerializeMap,
                SerializeSeq as _KybraTraitSerializeSeq,
                SerializeTuple as _KybraTraitSerializeTuple
            }
        };
        use slotmap::Key as _KybraTraitSlotMapKey;
        use std::{
            convert::TryInto as _KybraTraitTryInto,
            str::FromStr as _KybraTraitFromStr
        };
    }
}
