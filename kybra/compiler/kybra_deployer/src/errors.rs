pub trait UnwrapOrTrap<T> {
    fn unwrap_or_trap(self) -> T;
}

impl<T> UnwrapOrTrap<T> for Result<T, ic_stable_structures::cell::InitError> {
    fn unwrap_or_trap(self) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => ic_cdk::trap(&init_error_to_string(&err)),
        }
    }
}

pub fn call_result_error_to_string(err: &(ic_cdk::api::call::RejectionCode, String)) -> String {
    let rejection_code_string = rejection_code_to_string(&err.0);
    let message = &err.1;

    format!("{rejection_code_string}, {message}")
}

pub fn rejection_code_to_string(err: &ic_cdk::api::call::RejectionCode) -> String {
    match err {
        ic_cdk::api::call::RejectionCode::NoError => "RejectionCode: NoError".to_string(),
        ic_cdk::api::call::RejectionCode::SysFatal => "RejectionCode: SysFatal".to_string(),
        ic_cdk::api::call::RejectionCode::SysTransient => "RejectionCode: SysTransient".to_string(),
        ic_cdk::api::call::RejectionCode::DestinationInvalid => {
            "RejectionCode: DestinationInvalid".to_string()
        }
        ic_cdk::api::call::RejectionCode::CanisterReject => {
            "RejectionCode: CanisterReject".to_string()
        }
        ic_cdk::api::call::RejectionCode::CanisterError => {
            "RejectionCode: CanisterError".to_string()
        }
        ic_cdk::api::call::RejectionCode::Unknown => "RejectionCode: Unknown".to_string(),
    }
}

pub fn value_error_to_string(err: &ic_stable_structures::cell::ValueError) -> String {
    match err {
        ic_stable_structures::cell::ValueError::ValueTooLarge { value_size } => {
            format!("ValueError: ValueTooLarge {value_size}")
        }
    }
}

fn init_error_to_string(err: &ic_stable_structures::cell::InitError) -> String {
    match err {
        ic_stable_structures::cell::InitError::IncompatibleVersion {
            last_supported_version,
            decoded_version,
        } => format!("InitError: IncompatibleVersion, last_supported_version {last_supported_version}, decoded_version {decoded_version}"),
        ic_stable_structures::cell::InitError::ValueTooLarge { value_size } => {
            format!("InitError: ValueTooLarge {value_size}")
        }
    }
}
