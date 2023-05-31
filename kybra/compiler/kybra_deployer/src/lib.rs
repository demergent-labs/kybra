thread_local! {
    static WASM_REF_CELL: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(vec![]);
    // static STDLIB_REF_CELL
}

#[ic_cdk_macros::update]
pub fn upload_wasm_chunk(bytes: Vec<u8>) {
    WASM_REF_CELL.with(|wasm_ref_cell| {
        let mut wasm_ref_mut = wasm_ref_cell.borrow_mut();
        wasm_ref_mut.extend(bytes);
    });
}

// TODO we want to use notify
// TODO figure out args
// TODO it will probably be similar to kybra_modules_init
// TODO I think for the init stuff...I wonder if I can just store the args as binary
#[ic_cdk_macros::update]
pub async fn install_wasm() {
    let wasm_module = WASM_REF_CELL.with(|wasm_ref_cell| wasm_ref_cell.borrow().clone());

    // TODO figure out init/post_upgrade
    let result = ic_cdk::api::management_canister::main::install_code(
        ic_cdk::api::management_canister::main::InstallCodeArgument {
            mode: ic_cdk::api::management_canister::main::CanisterInstallMode::Upgrade,
            canister_id: ic_cdk::api::id(),
            wasm_module,
            arg: vec![],
        },
    )
    .await;

    match result {
        Ok(_) => {}
        Err(err) => {
            panic!("{:#?}", err);
        }
    }
}
