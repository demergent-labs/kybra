mod errors;

use crate::{py_ast::PyAst, source_map::SourceMapped};
use cdk_framework::{act::node::DataType, ToDataType};
use num_bigint::{BigInt, Sign};
use rustpython_parser::ast::{Constant, ExprKind, KeywordData, Located, StmtKind};

// TODO all variables should be called stable_b_tree_map_nodes
#[derive(Clone)]
pub struct StableBTreeMapNode {
    pub memory_id: u8,
    pub key_type: DataType,
    pub value_type: DataType,
    pub max_key_size: u32,
    pub max_value_size: u32,
}
impl PyAst {
    pub fn build_stable_b_tree_map_nodes(&self) -> Vec<StableBTreeMapNode> {
        self.get_stmt_kinds()
            .iter()
            .filter_map(|stmt| stmt.as_stable_b_tree_map_node())
            .collect()
    }
}

impl SourceMapped<&Located<ExprKind>> {
    pub fn is_stable_b_tree_map_node(&self) -> bool {
        match &self.node {
            ExprKind::Call { func, .. } => match &func.node {
                ExprKind::Subscript { value, .. } => match &value.node {
                    ExprKind::Name { id, .. } => id == "StableBTreeMap",
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }

    pub fn get_value_type(&self) -> SourceMapped<&Located<ExprKind>> {
        match &self.node {
            ExprKind::Subscript { slice, .. } => match &slice.node {
                ExprKind::Tuple { elts, .. } => {
                    SourceMapped::new(&elts[1], self.source_map.clone())
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    pub fn get_key_type(&self) -> SourceMapped<&Located<ExprKind>> {
        match &self.node {
            ExprKind::Subscript { slice, .. } => match &slice.node {
                ExprKind::Tuple { elts, .. } => {
                    SourceMapped::new(&elts[0], self.source_map.clone())
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_stable_b_tree_map_node(&self) -> bool {
        match &self.node {
            StmtKind::Assign { value, .. } => {
                SourceMapped::new(value.as_ref(), self.source_map.clone())
                    .is_stable_b_tree_map_node()
            }
            _ => false,
        }
    }

    pub fn as_stable_b_tree_map_node(&self) -> Option<StableBTreeMapNode> {
        if !self.is_stable_b_tree_map_node() {
            return None;
        }
        let memory_id = self.get_memory_id();
        let key_type = self.get_key_type();
        let value_type = self.get_value_type();
        let max_key_size = self.get_max_key_size();
        let max_value_size = self.get_max_value_size();
        Some(StableBTreeMapNode {
            memory_id,
            key_type,
            value_type,
            max_key_size,
            max_value_size,
        })
    }

    fn get_memory_id(&self) -> u8 {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { args, keywords, .. } => {
                    if args.len() >= 1 {
                        return match &args[0].node {
                            ExprKind::Constant { value, .. } => match value {
                                Constant::Int(integer) => self.big_int_to_memory_id(integer),
                                _ => panic!("{}", self.memory_id_must_be_an_integer_error()),
                            },
                            _ => panic!("{}", self.invalid_memory_id_error()),
                        };
                    }
                    if let Some(memory_id) = self.get_memory_from_keywords(keywords) {
                        return memory_id;
                    }
                    panic!("{}", self.missing_memory_id_error())
                }
                _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
            },
            _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
        }
    }

    fn get_key_type(&self) -> DataType {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { func, .. } => {
                    SourceMapped::new(func.as_ref(), self.source_map.clone())
                        .get_key_type()
                        .to_data_type()
                }
                _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
            },
            _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
        }
    }

    fn get_value_type(&self) -> DataType {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { func, .. } => {
                    SourceMapped::new(func.as_ref(), self.source_map.clone())
                        .get_value_type()
                        .to_data_type()
                }
                _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
            },
            _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
        }
    }

    fn get_max_key_size(&self) -> u32 {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { args, keywords, .. } => {
                    if args.len() >= 2 {
                        return self.get_max_size_from_args(1, args);
                    }
                    if let Some(max_key_size) = self.get_max_size_from_keywords("key", keywords) {
                        return max_key_size;
                    }
                    panic!("{}", self.max_key_size_missing_error())
                }
                _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
            },
            _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
        }
    }

    fn get_max_value_size(&self) -> u32 {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { args, keywords, .. } => {
                    if args.len() >= 3 {
                        return self.get_max_size_from_args(2, args);
                    }
                    if let Some(max_key_size) = self.get_max_size_from_keywords("value", keywords) {
                        return max_key_size;
                    }
                    panic!("{}", self.max_value_size_missing_error())
                }
                _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
            },
            _ => panic!("{}", self.not_a_stable_b_tree_map_node_error()),
        }
    }

    // Helper method for get_max_key_size and get_max_value_size
    fn get_max_size_from_keywords(
        &self,
        name: &str,
        keywords: &Vec<Located<KeywordData>>,
    ) -> Option<u32> {
        keywords.iter().fold(None, |act_key_type, keyword| {
            if let Some(arg_name) = &keyword.node.arg {
                if arg_name == format!("max_{}_size", name).as_str() {
                    match &keyword.node.value.node {
                        ExprKind::Constant { value, .. } => match value {
                            Constant::Int(int) => Some(self.big_int_to_max_size(int)),
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                } else {
                    if let Some(_) = act_key_type {
                        act_key_type
                    } else {
                        None
                    }
                }
            } else {
                if let Some(_) = act_key_type {
                    act_key_type
                } else {
                    None
                }
            }
        })
    }

    fn get_max_size_from_args(&self, name: usize, keywords: &Vec<Located<ExprKind>>) -> u32 {
        match &keywords[name].node {
            ExprKind::Constant { value, .. } => match value {
                Constant::Int(integer) => self.big_int_to_max_size(integer),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    // Helper method for get_memory_id
    fn get_memory_from_keywords(&self, keywords: &Vec<Located<KeywordData>>) -> Option<u8> {
        keywords.iter().fold(None, |act_key_type, keyword| {
            let result = if let Some(arg_name) = &keyword.node.arg {
                if arg_name == "memory_id" {
                    match &keyword.node.value.node {
                        ExprKind::Constant { value, .. } => match value {
                            Constant::Int(int) => Some(self.big_int_to_memory_id(int)),
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(_) = act_key_type {
                act_key_type
            } else {
                result
            }
        })
    }

    fn big_int_to_max_size(&self, num: &BigInt) -> u32 {
        let digits = num.to_u32_digits();
        if digits.0 == Sign::Minus {
            panic!("{}", self.max_size_must_be_non_negative())
        }
        if digits.1.len() > 1 {
            panic!("{}", self.max_size_too_big_error())
        }
        digits.1[0]
    }

    fn big_int_to_memory_id(&self, num: &BigInt) -> u8 {
        let digits = num.to_u32_digits();
        if digits.0 == Sign::Minus {
            panic!("{}", self.memory_id_must_be_non_negative())
        }
        if digits.1.len() > 1 {
            panic!("{}", self.memory_id_too_big_error())
        }
        if digits.1.len() == 0 {
            return 0;
        }
        let value = digits.1[0];
        if value > u8::MAX as u32 {
            panic!("{}", self.memory_id_too_big_error())
        }
        value as u8
    }
}
