use std::fmt;

use backtrace::Backtrace;

use crate::Error;

#[derive(Clone, Debug)]
pub struct Unreachable {
    pub backtrace: Backtrace,
}

impl std::fmt::Display for Unreachable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = "Oops! Looks like we introduced a bug while refactoring.";
        let ticket_url =
            "Please open a ticket at https://github.com/demergent-labs/kybra/issues/new";
        let stack_trace = "Please include the following backtrace:";
        write!(
            f,
            "{message}\n{ticket_url}\n{stack_trace}\n{:?}",
            self.backtrace
        )
    }
}

impl Unreachable {
    pub fn error() -> Error {
        Unreachable {
            backtrace: Backtrace::new(),
        }
        .into()
    }
}

impl From<Unreachable> for Error {
    fn from(value: Unreachable) -> Self {
        Self::Unreachable(value)
    }
}

/// Indicates a branch of logic that should be unreachable because other checks
/// should have caught this case before.
///
/// Unlike the builtin `unreachable!` however, this does not panic, but rather
/// returns an error.
///
/// Expands to:
/// ```rust
/// return Err(Error::Unreachable::error().into())
/// ```
#[macro_export]
macro_rules! kybra_unreachable {
    () => {
        return Err(crate::errors::Unreachable::error().into())
    };
}
