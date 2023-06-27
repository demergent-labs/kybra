// TODO Once we upgrade to ic-cdk 0.8.1 and dfx >= 14 something
// TODO then I think we can use the new ic_cdk::api::is_controller check
// TODO and get rid of INSTALLER_REF_CELL

use errors::{
    call_result_error_to_string, rejection_code_to_string, value_error_to_string, UnwrapOrTrap,
};
use sha2::{Digest, Sha256};
use shared_utils::{PYTHON_STDLIB_MEMORY_ID, RANDOMNESS_MEMORY_ID};

mod errors;

thread_local! {
    static WASM_REF_CELL: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(vec![]);

    static INSTALLER_REF_CELL: std::cell::RefCell<String> = std::cell::RefCell::new("".to_string());

    static ARG_DATA_RAW_REF_CELL: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(vec![]);

    static PYTHON_STDLIB_REF_CELL: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(vec![]);

    static RANDOMNESS_STABLE_REF_CELL: std::cell::RefCell<
        ic_stable_structures::cell::Cell<
            Vec<u8>,
            ic_stable_structures::memory_manager::VirtualMemory<
                ic_stable_structures::DefaultMemoryImpl
            >
        >
    > = std::cell::RefCell::new(
        ic_stable_structures::cell::Cell::init(
            MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(ic_stable_structures::memory_manager::MemoryId::new(RANDOMNESS_MEMORY_ID))), vec![]
        ).unwrap_or_trap()
    );

    static PYTHON_STDLIB_STABLE_REF_CELL: std::cell::RefCell<
        ic_stable_structures::cell::Cell<
            Vec<u8>,
            ic_stable_structures::memory_manager::VirtualMemory<
                ic_stable_structures::DefaultMemoryImpl
            >
        >
    > = std::cell::RefCell::new(
        ic_stable_structures::cell::Cell::init(
            MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(ic_stable_structures::memory_manager::MemoryId::new(PYTHON_STDLIB_MEMORY_ID))), vec![]
        ).unwrap_or_trap()
    );

    static MEMORY_MANAGER_REF_CELL: std::cell::RefCell<
        ic_stable_structures::memory_manager::MemoryManager<
            ic_stable_structures::DefaultMemoryImpl
        >
    > = std::cell::RefCell::new(
        ic_stable_structures::memory_manager::MemoryManager::init(
            ic_stable_structures::DefaultMemoryImpl::default()
        )
    );
}

#[ic_cdk_macros::init]
pub fn init() {
    INSTALLER_REF_CELL.with(|installer_ref_cell| {
        let mut installer_ref_mut = installer_ref_cell.borrow_mut();
        *installer_ref_mut = ic_cdk::caller().to_string();
    });

    ARG_DATA_RAW_REF_CELL.with(|arg_data_raw_ref_cell| {
        let mut arg_data_ref_mut = arg_data_raw_ref_cell.borrow_mut();
        *arg_data_ref_mut = ic_cdk::api::call::arg_data_raw();
    });
}

#[ic_cdk_macros::post_upgrade]
pub fn post_upgrade() {
    INSTALLER_REF_CELL.with(|installer_ref_cell| {
        let mut installer_ref_mut = installer_ref_cell.borrow_mut();
        *installer_ref_mut = ic_cdk::caller().to_string();
    });

    ARG_DATA_RAW_REF_CELL.with(|arg_data_raw_ref_cell| {
        let mut arg_data_ref_mut = arg_data_raw_ref_cell.borrow_mut();
        *arg_data_ref_mut = ic_cdk::api::call::arg_data_raw();
    });
}

#[ic_cdk_macros::update(guard = "installer_guard")]
pub fn upload_wasm_chunk(bytes: Vec<u8>) {
    WASM_REF_CELL.with(|wasm_ref_cell| {
        let mut wasm_ref_mut = wasm_ref_cell.borrow_mut();
        wasm_ref_mut.extend(bytes);
    });
}

#[ic_cdk_macros::update(guard = "installer_guard")]
pub fn upload_python_stdlib_chunk(bytes: Vec<u8>) {
    PYTHON_STDLIB_REF_CELL.with(|python_stdlib_ref_cell| {
        let mut python_stdlib_ref_mut = python_stdlib_ref_cell.borrow_mut();
        python_stdlib_ref_mut.extend(bytes);
    });
}

#[ic_cdk_macros::query(guard = "installer_guard")]
pub fn python_stdlib_hash() -> String {
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

#[ic_cdk_macros::update(guard = "installer_guard")]
pub async fn install_wasm() -> Result<(), String> {
    stable_store_randomness().await?;
    stable_store_python_stdlib()?;
    install_code().map_err(|err| rejection_code_to_string(&err))
}

async fn stable_store_randomness() -> Result<(), String> {
    let randomness_result = ic_cdk::api::management_canister::main::raw_rand().await;
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
    ic_cdk::api::call::notify(
        ic_cdk::export::Principal::management_canister(),
        "install_code",
        (
            ic_cdk::api::management_canister::main::InstallCodeArgument {
                mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Upgrade,
                canister_id: ic_cdk::api::id(),
                wasm_module,
                arg: ARG_DATA_RAW_REF_CELL
                    .with(|arg_data_raw_ref_cell| arg_data_raw_ref_cell.borrow().clone()),
            },
        ),
    )

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

fn installer_guard() -> Result<(), String> {
    let installer =
        INSTALLER_REF_CELL.with(|installer_ref_cell| installer_ref_cell.borrow().clone());

    if installer == ic_cdk::caller().to_string() {
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
