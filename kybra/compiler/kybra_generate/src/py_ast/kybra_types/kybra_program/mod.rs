use rustpython_parser::ast::Mod;

use crate::{
    cdk_act::{ActCanisterMethod, ActDataType, CanisterMethodType},
    source_map::SourceMap,
};

use super::KybraStmt;

pub struct KybraProgram<'a> {
    pub program: Mod,
    pub source_map: &'a SourceMap,
}

impl KybraProgram<'_> {
    pub fn build_canister_method_act_nodes(&self) -> Vec<ActCanisterMethod> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_canister_method_type(CanisterMethodType::Query)
                        || kybra_stmt.is_canister_method_type(CanisterMethodType::Update)
                })
                .map(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    match kybra_stmt.as_canister_method() {
                        Some(canister_method) => {
                            if kybra_stmt.is_canister_method_type(CanisterMethodType::Query) {
                                ActCanisterMethod::QueryMethod(canister_method)
                            } else {
                                ActCanisterMethod::UpdateMethod(canister_method)
                            }
                        }
                        None => panic!("Unreachable"),
                    }
                })
                .collect(),
            _ => vec![],
        }
    }

    pub fn get_act_data_type_nodes(&self) -> Vec<ActDataType> {
        match &self.program {
            Mod::Module { body, .. } => body
                .iter()
                .filter(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    kybra_stmt.is_record() || kybra_stmt.is_tuple() || kybra_stmt.is_variant()
                })
                .map(|stmt_kind| {
                    let kybra_stmt = KybraStmt {
                        stmt_kind,
                        source_map: self.source_map,
                    };
                    kybra_stmt.build_act_data_type()
                })
                .collect(),
            _ => vec![],
        }
    }
    // fn get_ast_function_defs(self) -> Vec<KybraFunctionDef> {
    //     match self.program {
    //         Mod::Module { body, .. } => body
    //             .into_iter()
    //             .filter(|located_stmt| match located_stmt.node {
    //                 StmtKind::FunctionDef { .. } => true,
    //                 _ => false,
    //             })
    //             .map(|stmt| match stmt.node {
    //                 StmtKind::FunctionDef {
    //                     name,
    //                     args,
    //                     body,
    //                     decorator_list,
    //                     returns,
    //                     type_comment,
    //                 } => KybraFunctionDef {
    //                     name,
    //                     args,
    //                     body,
    //                     decorator_list,
    //                     returns,
    //                     type_comment,
    //                 },
    //                 _ => panic!("Unreachable"),
    //             })
    //             .collect(),
    //         Mod::Interactive { .. } => vec![],
    //         Mod::Expression { .. } => vec![],
    //         Mod::FunctionType { .. } => vec![],
    //     }
    // }

    // fn get_kybra_type_alias_decls(&self) -> Vec<String> {
    //     todo!()
    // }
}

// pub trait KybraProgramVecHelperMethods {
//     fn get_kybra_fn_decls(self) -> Vec<KybraFunctionDef>;
//     fn get_kybra_fn_decls_of_type(
//         self,
//         canister_method_type: &CanisterMethodType,
//     ) -> Vec<KybraFunctionDef>;

//     fn get_kybra_type_alias_decls(self) -> Vec<KybraTypeAliasDecl>;
//     fn get_dependent_types(self) -> HashSet<String>;
// }

// impl KybraProgramVecHelperMethods for Vec<KybraMod> {
//     fn get_kybra_fn_decls_of_type(
//         self,
//         canister_method_type: &CanisterMethodType,
//     ) -> Vec<KybraFunctionDef> {
//         let kybra_fn_decls = self.get_kybra_fn_decls();

//         kybra_fn_decls
//             .into_iter()
//             .filter(|kybra_fn_decl| kybra_fn_decl.is_canister_method_type(canister_method_type))
//             .collect()
//     }

//     fn get_kybra_fn_decls(self) -> Vec<KybraFunctionDef> {
//         self.into_iter().fold(vec![], |mut acc, kybra_program| {
//             let mut kybra_fn_decls = kybra_program.get_ast_function_defs();

//             acc.append(&mut kybra_fn_decls);
//             acc
//         })
//     }

//     fn get_kybra_type_alias_decls(self) -> Vec<KybraTypeAliasDecl> {
//         todo!()
//     }

//     fn get_dependent_types(self) -> HashSet<String> {
//         todo!()
//     }
// }
