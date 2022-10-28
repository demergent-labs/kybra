use crate::cdk_act::{
    nodes::{act_canister_method, data_type_nodes},
    ActCanisterMethod, ActDataType,
};

pub use self::{kybra_ast::KybraAst, kybra_types::KybraProgram};

mod kybra_ast;
mod kybra_types;
mod system_methods;
pub mod traits;
mod what_is_it;

pub struct PyAst<'a> {
    pub kybra_programs: Vec<KybraProgram<'a>>,
    pub entry_module_name: String,
}

impl PyAst<'_> {
    pub fn to_kybra_ast(&self) -> KybraAst {
        let canister_types = self.build_canister_types();
        for can_type in &canister_types {
            eprintln!("This is a canister type: {:#?}", can_type)
        }
        let types_alias_inline_types = data_type_nodes::build_inline_type_acts(&canister_types);
        let canister_methods = self.build_canister_methods();
        let canister_method_types =
            act_canister_method::get_all_types_from_canister_method_acts(&canister_methods);
        let canister_method_inline_types =
            data_type_nodes::build_inline_type_acts(&canister_method_types);

        let all_types = vec![
            canister_types,
            types_alias_inline_types,
            canister_method_inline_types,
        ]
        .concat();

        KybraAst {
            init_method: self.build_init_method(),
            post_upgrade: self.build_post_upgrade_method(),
            canister_types: all_types,
            canister_methods: self.build_canister_methods(),
        }
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
