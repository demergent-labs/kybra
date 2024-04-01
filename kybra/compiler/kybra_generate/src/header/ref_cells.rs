use proc_macro2::TokenStream;
use quote::quote;

// TODO when we have the stable memory filesystem working let's see if you can just use that
const PYTHON_STDLIB_MEMORY_ID: u8 = 254;

pub fn generate() -> TokenStream {
    quote! {
        thread_local! {
            static PYTHON_STDLIB_STABLE_REF_CELL: std::cell::RefCell<
                ic_stable_structures::cell::Cell<
                    Vec<u8>,
                    ic_stable_structures::memory_manager::VirtualMemory<
                        ic_stable_structures::DefaultMemoryImpl
                    >
                >
            > = std::cell::RefCell::new(
                ic_stable_structures::cell::Cell::init(
                    MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(ic_stable_structures::memory_manager::MemoryId::new(#PYTHON_STDLIB_MEMORY_ID))), vec![]
                ).unwrap_or_trap()
            );
        }
    }
}
