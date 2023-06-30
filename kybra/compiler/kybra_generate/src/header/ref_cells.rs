use proc_macro2::TokenStream;
use quote::quote;
use shared_utils::{CANISTER_INITIALIZED_MEMORY_ID, PYTHON_STDLIB_MEMORY_ID, RANDOMNESS_MEMORY_ID};

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

            static CANISTER_INITIALIZED_REF_CELL: std::cell::RefCell<
                ic_stable_structures::cell::Cell<
                    u8,
                    ic_stable_structures::memory_manager::VirtualMemory<
                        ic_stable_structures::DefaultMemoryImpl
                    >
                >
            > = std::cell::RefCell::new(
                ic_stable_structures::cell::Cell::init(
                    MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(ic_stable_structures::memory_manager::MemoryId::new(#CANISTER_INITIALIZED_MEMORY_ID))), 0
                ).unwrap_or_trap()
            );

            static RANDOMNESS_STABLE_REF_CELL: std::cell::RefCell<
                ic_stable_structures::cell::Cell<
                    Vec<u8>,
                    ic_stable_structures::memory_manager::VirtualMemory<
                        ic_stable_structures::DefaultMemoryImpl
                    >
                >
            > = std::cell::RefCell::new(
                ic_stable_structures::cell::Cell::init(
                    MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(ic_stable_structures::memory_manager::MemoryId::new(#RANDOMNESS_MEMORY_ID))), vec![]
                ).unwrap_or_trap()
            );
        }
    }
}
