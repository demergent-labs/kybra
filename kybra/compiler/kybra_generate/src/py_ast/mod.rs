use crate::cdk_act::{ActCanisterMethod, ActDataType};

pub use self::{kybra_ast::KybraAst, kybra_types::KybraProgram};

mod kybra_ast;
mod kybra_types;
mod system_methods;

pub struct PyAst<'a> {
    pub kybra_programs: Vec<KybraProgram<'a>>,
    pub entry_module_name: String,
}

impl PyAst<'_> {
    pub fn to_kybra_ast(self) -> KybraAst {
        let thing = KybraAst {
            init_method: self.build_init_method(),
            post_upgrade: self.build_post_upgrade_method(),
            canister_methods: self.build_canister_methods(),
            canister_types: self.build_canister_types(),
        };
        eprintln!(
            "Here are the canister_methods,\n{:#?}",
            thing.canister_methods
        );
        thing
    }

    fn build_canister_methods(&self) -> Vec<ActCanisterMethod> {
        self.kybra_programs.iter().fold(vec![], |acc, py_mod| {
            vec![acc, py_mod.build_canister_method_act_nodes()].concat()
        })
    }

    fn build_canister_types(&self) -> Vec<ActDataType> {
        self.kybra_programs.iter().fold(vec![], |acc, py_mod| {
            vec![acc, py_mod.get_act_data_type_nodes()].concat()
        })
    }
}
