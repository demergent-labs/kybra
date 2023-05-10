pub mod missing_decorator_error;
pub mod service_must_have_methods;
pub mod service_with_non_function_defs;
pub mod too_many_decorators;
pub mod wrong_decorator;

pub use missing_decorator_error::MissingDecorator;
pub use service_must_have_methods::ServiceMustHaveMethods;
pub use service_with_non_function_defs::ServiceWithNonFunctionDefs;
pub use too_many_decorators::TooManyDecorators;
pub use wrong_decorator::WrongDecorator;
