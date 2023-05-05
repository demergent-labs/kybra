pub mod invalid_annotation_assign;
pub mod invalid_assign;
pub mod invalid_class;
pub mod invalid_member;
pub mod invalid_subscriptable;
pub mod invalid_target;
pub mod none_cannot_be_a_type;
pub mod not_exactly_one_target;
pub mod target_must_be_a_name;
pub mod unsupported_type;

use rustpython_parser::ast::{ExprKind, Located};

use crate::{errors::CreateMessage, source_map::SourceMapped, Error};

pub use invalid_annotation_assign::InvalidAnnAssign;
pub use invalid_assign::InvalidAssign;
pub use invalid_class::InvalidClass;
pub use invalid_member::InvalidMember;
pub use invalid_target::InvalidTarget;
pub use not_exactly_one_target::NotExactlyOneTarget;
pub use target_must_be_a_name::TargetMustBeAName;
pub use unsupported_type::UnsupportedType;

impl SourceMapped<&Located<ExprKind>> {
    pub fn invalid_subscriptable_error(&self) -> Error {
        let title =
            "Only Async, Vec, Manual, Opt, or Tuple are allowed subscriptables for candid values";
        let annotation = "Invalid subscriptable here";
        Error::InvalidSubscriptable(self.create_error_message(title, annotation, None))
    }

    pub fn none_cant_be_a_type_error(&self) -> Error {
        Error::NoneCannotBeAType(self.create_error_message("None must not be used as a type, but only as a value. Please specify either kybra.null or kybra.void.", "Ambiguous None here", None))
    }
}
