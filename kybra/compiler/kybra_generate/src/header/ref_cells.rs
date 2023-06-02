use proc_macro2::TokenStream;
use quote::quote;

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
                    MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(ic_stable_structures::memory_manager::MemoryId::new(253))), vec![]
                ).unwrap()
            );

            static INITIALIZED_MAP_REF_CELL: std::cell::RefCell<
                ic_stable_structures::cell::Cell<
                    u8,
                    ic_stable_structures::memory_manager::VirtualMemory<
                        ic_stable_structures::DefaultMemoryImpl
                    >
                >
            > = std::cell::RefCell::new(
                ic_stable_structures::cell::Cell::init(
                    MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(ic_stable_structures::memory_manager::MemoryId::new(254))), 0
                ).unwrap()
            );
        }
    }
}
