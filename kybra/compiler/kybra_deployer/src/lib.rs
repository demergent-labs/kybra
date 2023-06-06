// TODO installer guard not necessary, we should be able to use is_controller in that guard

use sha2::{Digest, Sha256};

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
            MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(ic_stable_structures::memory_manager::MemoryId::new(252))), vec![]
        ).unwrap()
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
            MEMORY_MANAGER_REF_CELL.with(|m| m.borrow().get(ic_stable_structures::memory_manager::MemoryId::new(253))), vec![]
        ).unwrap()
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
            let mut hasher = Sha256::new();

            hasher.update(python_stdlib_bytes);

            let result = hasher.finalize();

            hex::encode(result)
        }
    })
}

#[ic_cdk_macros::update(guard = "installer_guard")]
pub async fn install_wasm() {
    let randomness_result: ic_cdk::api::call::CallResult<(Vec<u8>,)> = ic_cdk::api::call::call(
        ic_cdk::export::Principal::from_text("aaaaa-aa").unwrap(),
        "raw_rand",
        (),
    )
    .await;

    RANDOMNESS_STABLE_REF_CELL.with(|randomness_stable_ref_cell| {
        let mut randomness_stable_ref_mut = randomness_stable_ref_cell.borrow_mut();

        let randomness = randomness_result.unwrap().0;

        randomness_stable_ref_mut.set(randomness).unwrap();
    });

    PYTHON_STDLIB_STABLE_REF_CELL.with(|python_stdlib_stable_ref_cell| {
        let mut python_stdlib_stable_ref_mut = python_stdlib_stable_ref_cell.borrow_mut();

        let python_stdlib_bytes = PYTHON_STDLIB_REF_CELL
            .with(|python_stdlib_ref_cell| python_stdlib_ref_cell.borrow().clone());

        if python_stdlib_bytes.len() != 0 {
            python_stdlib_stable_ref_mut
                .set(python_stdlib_bytes)
                .unwrap();
        }
    });

    let wasm_module = WASM_REF_CELL.with(|wasm_ref_cell| wasm_ref_cell.borrow().clone());

    let result = ic_cdk::api::call::notify(
        ic_cdk::api::management_canister::main::CanisterId::management_canister(),
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
    );

    match result {
        Ok(_) => {}
        Err(err) => {
            panic!("{:#?}", err);
        }
    }
}

fn installer_guard() -> Result<(), String> {
    let installer =
        INSTALLER_REF_CELL.with(|installer_ref_cell| installer_ref_cell.borrow().clone());

    if installer == ic_cdk::caller().to_string() {
        return Ok(());
    }

    Err("Not authorized".to_string())
}
