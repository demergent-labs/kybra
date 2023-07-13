use errors::{
    call_result_error_to_string, rejection_code_to_string, value_error_to_string, UnwrapOrTrap,
};
use ic_cdk::{
    api::{call, management_canister},
    export::Principal,
};
use ic_cdk_macros::{init, post_upgrade, query, update};
use ic_stable_structures::{
    cell::Cell,
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};
use sha2::{Digest, Sha256};
use shared_utils::{PYTHON_STDLIB_MEMORY_ID, RANDOMNESS_MEMORY_ID};
use std::cell::RefCell;

mod errors;

thread_local! {
    static WASM_REF_CELL: RefCell<Vec<u8>> = RefCell::new(vec![]);
    static ARG_DATA_RAW_REF_CELL: RefCell<Vec<u8>> = RefCell::new(vec![]);
    static PYTHON_STDLIB_REF_CELL: RefCell<Vec<u8>> = RefCell::new(vec![]);

    static RANDOMNESS_STABLE_REF_CELL: RefCell<Cell<Vec<u8>, VirtualMemory<DefaultMemoryImpl>>>
    = RefCell::new(Cell::init(MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(MemoryId::new(RANDOMNESS_MEMORY_ID))), vec![]).unwrap_or_trap());

    static PYTHON_STDLIB_STABLE_REF_CELL: RefCell<Cell<Vec<u8>, VirtualMemory<DefaultMemoryImpl>>>
    = RefCell::new(Cell::init(MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(MemoryId::new(PYTHON_STDLIB_MEMORY_ID))), vec![]).unwrap_or_trap());

    static MEMORY_MANAGER_REF_CELL: RefCell<MemoryManager<DefaultMemoryImpl>>
    = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

#[init]
pub fn init() {
    initialize()
}

#[post_upgrade]
pub fn post_upgrade() {
    initialize()
}

fn initialize() {
    ARG_DATA_RAW_REF_CELL.with(|arg_data_raw_ref_cell| {
        let mut arg_data_ref_mut = arg_data_raw_ref_cell.borrow_mut();
        *arg_data_ref_mut = call::arg_data_raw();
    });
}

#[update(guard = "controller_guard")]
pub fn clear_chunks() {
    WASM_REF_CELL.with(|wasm_ref_cell| {
        let mut wasm_ref_mut = wasm_ref_cell.borrow_mut();
        wasm_ref_mut.clear();
    });

    PYTHON_STDLIB_REF_CELL.with(|python_stdlib_ref_cell| {
        let mut python_stdlib_ref_mut = python_stdlib_ref_cell.borrow_mut();
        python_stdlib_ref_mut.clear();
    });
}

#[update(guard = "controller_guard")]
pub fn upload_wasm_chunk(bytes: Vec<u8>) {
    WASM_REF_CELL.with(|wasm_ref_cell| {
        let mut wasm_ref_mut = wasm_ref_cell.borrow_mut();
        wasm_ref_mut.extend(bytes);
    });
}

#[update(guard = "controller_guard")]
pub fn upload_python_stdlib_chunk(bytes: Vec<u8>) {
    PYTHON_STDLIB_REF_CELL.with(|python_stdlib_ref_cell| {
        let mut python_stdlib_ref_mut = python_stdlib_ref_cell.borrow_mut();
        python_stdlib_ref_mut.extend(bytes);
    });
}

#[query(guard = "controller_guard")]
pub fn get_python_stdlib_hash() -> String {
    PYTHON_STDLIB_STABLE_REF_CELL.with(|python_stdlib_stable_ref_cell| {
        let python_stdlib_stable_ref = python_stdlib_stable_ref_cell.borrow();
        let python_stdlib_bytes = python_stdlib_stable_ref.get();

        if python_stdlib_bytes.len() == 0 {
            "".to_string()
        } else {
            hash(python_stdlib_bytes)
        }
    })
}

#[update(guard = "controller_guard")]
pub async fn install_wasm() -> Result<(), String> {
    stable_store_randomness().await?;
    stable_store_python_stdlib()?;
    install_code().map_err(|err| rejection_code_to_string(&err))

    // let wasm_module = WASM_REF_CELL.with(|wasm_ref_cell| wasm_ref_cell.borrow().clone());

    // let result = ic_cdk::api::management_canister::main::install_code(
    //     ic_cdk::api::management_canister::main::InstallCodeArgument {
    //         mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Upgrade,
    //         canister_id: ic_cdk::api::id(),
    //         wasm_module,
    //         arg: ARG_DATA_RAW_REF_CELL
    //             .with(|arg_data_raw_ref_cell| arg_data_raw_ref_cell.borrow().clone()),
    //     },
    // )
    // .await;

    // result.map_err(|err| call_result_error_to_string(&err))
}

async fn stable_store_randomness() -> Result<(), String> {
    let randomness_result = management_canister::main::raw_rand().await;
    let randomness = randomness_result
        .map_err(|err| call_result_error_to_string(&err))?
        .0;

    RANDOMNESS_STABLE_REF_CELL.with(|randomness_stable_ref_cell| {
        let mut randomness_stable_ref_mut = randomness_stable_ref_cell.borrow_mut();
        randomness_stable_ref_mut
            .set(randomness)
            .map_err(|err| value_error_to_string(&err))
    })?;

    Ok(())
}

fn stable_store_python_stdlib() -> Result<(), String> {
    PYTHON_STDLIB_STABLE_REF_CELL.with(|python_stdlib_stable_ref_cell| {
        let mut python_stdlib_stable_ref_mut = python_stdlib_stable_ref_cell.borrow_mut();

        let python_stdlib_bytes = PYTHON_STDLIB_REF_CELL
            .with(|python_stdlib_ref_cell| python_stdlib_ref_cell.borrow().clone());

        if python_stdlib_bytes.len() != 0 {
            return python_stdlib_stable_ref_mut
                .set(python_stdlib_bytes)
                .map_err(|err| value_error_to_string(&err));
        }

        Ok(vec![])
    })?;

    Ok(())
}

fn install_code() -> Result<(), ic_cdk::api::call::RejectionCode> {
    let wasm_module = WASM_REF_CELL.with(|wasm_ref_cell| wasm_ref_cell.borrow().clone());

    let result = call::notify(
        Principal::management_canister(),
        "install_code",
        (management_canister::main::InstallCodeArgument {
            mode: management_canister::main::CanisterInstallMode::Upgrade,
            canister_id: ic_cdk::api::id(),
            wasm_module,
            arg: ARG_DATA_RAW_REF_CELL
                .with(|arg_data_raw_ref_cell| arg_data_raw_ref_cell.borrow().clone()),
        },),
    );

    result

    // TODO we think that the response trying to execute a callback on a different Wasm binary
    // TODO may be causing problems, thus we are testing out notify
    // let result = ic_cdk::api::management_canister::main::install_code(
    //     ic_cdk::api::management_canister::main::InstallCodeArgument {
    //         mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Upgrade,
    //         canister_id: ic_cdk::api::id(),
    //         wasm_module,
    //         arg: ARG_DATA_RAW_REF_CELL
    //             .with(|arg_data_raw_ref_cell| arg_data_raw_ref_cell.borrow().clone()),
    //     },
    // )
    // .await;

    // If install_code succeeds this will never be reached because
    // the install_code call returns to a different Wasm binary
    // and the callback no longer exists
    // If there is an error in the cross-canister call this will be reached
    // result.map_err(|err| call_result_error_to_string(&err))
}

fn controller_guard() -> Result<(), String> {
    if ic_cdk::api::is_controller(&ic_cdk::caller()) {
        return Ok(());
    }

    Err("Not authorized".to_string())
}

fn hash(bytes: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();

    hasher.update(bytes);

    let result = hasher.finalize();

    hex::encode(result)
}
