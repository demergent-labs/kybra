pub mod errors;
mod rust;

use cdk_framework::{
    act::node::{
        candid::{self, service::Method},
        node_parts::mode::Mode,
        ReturnType,
    },
    traits::CollectResults,
};
use rustpython_parser::ast::{ExprKind, Located, StmtKind};

use crate::{
    constants::{SERVICE, SERVICE_QUERY, SERVICE_UPDATE},
    errors::CollectResults as OtherCollectResults,
    get_name::HasName,
    method_utils::params::InternalOrExternal,
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};

use self::errors::{
    MissingDecorator, ServiceMustHaveMethods, ServiceWithNonFunctionDefs, TooManyDecorators,
    WrongDecorator,
};

impl PyAst {
    pub fn build_services(&self) -> Result<Vec<candid::Service>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_service())
            .collect_results()?
            .drain(..)
            .filter_map(|x| x)
            .collect())
    }
}

impl SourceMapped<&Located<StmtKind>> {
    fn to_service_method(&self, canister_name: &String) -> Result<Method, Vec<Error>> {
        match &self.node {
            StmtKind::FunctionDef {
                name,
                decorator_list,
                ..
            } => {
                let (params, return_type) = (
                    self.build_params(InternalOrExternal::External),
                    self.build_return_type(),
                )
                    .collect_results()?;
                Ok(Method {
                    name: name.clone(),
                    params,
                    return_type: ReturnType::new(return_type),
                    mode: match build_mode(self, decorator_list, &canister_name, name) {
                        Ok(mode) => mode,
                        Err(err) => return Err(err.into()),
                    },
                })
            }
            _ => Err(ServiceWithNonFunctionDefs::err_from_stmt(self, &canister_name).into()),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    fn as_service(&self) -> Result<Option<candid::Service>, Vec<Error>> {
        match self.get_child_class_of(SERVICE) {
            Some(service) => {
                let method_results: Vec<_> = service
                    .body
                    .iter()
                    .map(|located_statement| {
                        SourceMapped::new(located_statement, self.source_map.clone())
                            .to_service_method(&service.name)
                    })
                    .collect();

                let methods = method_results.into_iter().collect_results()?;

                if methods.len() == 0 {
                    return Err(ServiceMustHaveMethods::err_from_stmt(self, service.name).into());
                }

                Ok(Some(candid::Service {
                    name: service.name.clone(),
                    methods,
                    to_vm_value: rust::to_vm_value,
                    list_to_vm_value: rust::list_to_vm_value,
                    from_vm_value: rust::from_vm_value,
                    list_from_vm_value: rust::list_from_vm_value,
                }))
            }
            None => Ok(None),
        }
    }
}

fn build_mode(
    method_stmt: &SourceMapped<&Located<StmtKind>>,
    decorator_list: &Vec<Located<ExprKind>>,
    canister_name: &String,
    method_name: &String,
) -> Result<Mode, Error> {
    if decorator_list.len() == 0 {
        return Err(MissingDecorator::err_from_stmt(
            method_stmt,
            canister_name,
            method_name,
        ));
    }

    if decorator_list.len() > 1 {
        return Err(TooManyDecorators::err_from_stmt(
            method_stmt,
            canister_name,
            method_name,
        ));
    }

    match decorator_list[0].get_name() {
        Some(SERVICE_QUERY) => Ok(Mode::Query),
        Some(SERVICE_UPDATE) => Ok(Mode::Update),
        Some(decorator_name) => Err(WrongDecorator::err_from_stmt(
            method_stmt,
            canister_name,
            method_name,
            Some(decorator_name),
        )),
        None => Err(WrongDecorator::err_from_stmt(
            method_stmt,
            canister_name,
            method_name,
            None,
        )),
    }
}
