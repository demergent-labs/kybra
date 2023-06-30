use crate::ERROR_PREFIX;

pub fn error_to_string(e: &dyn std::error::Error) -> String {
    format!("{ERROR_PREFIX}: {}", e.to_string())
}

pub fn create_error_string(message: &str) -> String {
    format!("{ERROR_PREFIX}: {message}")
}
