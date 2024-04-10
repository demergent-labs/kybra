use cdk_framework::act::node::canister_method::{
    CanisterMethod, CanisterMethodType, PostUpgradeMethod,
};

use crate::{errors::Unreachable, py_ast::PyAst, Error};

impl PyAst {
    pub fn build_post_upgrade_method(&self) -> Result<PostUpgradeMethod, Vec<Error>> {
        let init_or_post_upgrade_method =
            self.build_init_or_post_upgrade_method(CanisterMethodType::PostUpgrade)?;

        match init_or_post_upgrade_method {
            CanisterMethod::PostUpgrade(post_upgrade_method) => Ok(post_upgrade_method),
            _ => Err(vec![Unreachable::error()]),
        }
    }
}
