mod errors;

use crate::{errors::KybraResult, py_ast::PyAst, source_map::SourceMapped};
use cdk_framework::act::node::CandidType;
use num_bigint::{BigInt, Sign};
use rustpython_parser::ast::{Constant, ExprKind, KeywordData, Located, StmtKind};

// TODO all variables should be called stable_b_tree_map_nodes
#[derive(Clone)]
pub struct StableBTreeMapNode {
    pub memory_id: u8,
    pub key_type: CandidType,
    pub value_type: CandidType,
    pub max_key_size: u32,
    pub max_value_size: u32,
}
impl PyAst {
    pub fn build_stable_b_tree_map_nodes(&self) -> KybraResult<Vec<StableBTreeMapNode>> {
        Ok(crate::errors::collect_kybra_results(
            self.get_stmt_kinds()
                .iter()
                .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_stable_b_tree_map_node())
                .collect(),
        )?
        .drain(..)
        .filter_map(|x| x)
        .collect())
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

    pub fn get_value_type(&self) -> KybraResult<SourceMapped<&Located<ExprKind>>> {
        match &self.node {
            ExprKind::Subscript { slice, .. } => match &slice.node {
                ExprKind::Tuple { elts, .. } => {
                    Ok(SourceMapped::new(&elts[1], self.source_map.clone()))
                }
                _ => Err(self.stable_b_tree_map_node_format_error()),
            },
            _ => Err(crate::errors::unreachable()),
        }
    }

    pub fn get_key_type(&self) -> KybraResult<SourceMapped<&Located<ExprKind>>> {
        match &self.node {
            ExprKind::Subscript { slice, .. } => match &slice.node {
                ExprKind::Tuple { elts, .. } => {
                    Ok(SourceMapped::new(&elts[0], self.source_map.clone()))
                }
                _ => Err(self.stable_b_tree_map_node_format_error()),
            },
            _ => Err(crate::errors::unreachable()),
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

    pub fn as_stable_b_tree_map_node(&self) -> KybraResult<Option<StableBTreeMapNode>> {
        if !self.is_stable_b_tree_map_node() {
            return Ok(None);
        }
        let memory_id = self.get_memory_id()?;
        let key_type = self.get_key_type()?;
        let value_type = self.get_value_type()?;
        let max_key_size = self.get_max_key_size()?;
        let max_value_size = self.get_max_value_size()?;
        Ok(Some(StableBTreeMapNode {
            memory_id,
            key_type,
            value_type,
            max_key_size,
            max_value_size,
        }))
    }

    fn get_memory_id(&self) -> KybraResult<u8> {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { args, keywords, .. } => {
                    if args.len() >= 1 {
                        return match &args[0].node {
                            ExprKind::Constant { value, .. } => match value {
                                Constant::Int(integer) => self.big_int_to_memory_id(integer),
                                _ => Err(self.memory_id_must_be_an_integer_error()),
                            },
                            _ => Err(self.invalid_memory_id_error()),
                        };
                    }
                    if let Some(memory_id) = self.get_memory_from_keywords(keywords) {
                        return memory_id;
                    }
                    Err(self.missing_memory_id_error())
                }
                _ => Err(self.not_a_stable_b_tree_map_node_error()),
            },
            _ => Err(self.not_a_stable_b_tree_map_node_error()),
        }
    }

    fn get_key_type(&self) -> KybraResult<CandidType> {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { func, .. } => {
                    SourceMapped::new(func.as_ref(), self.source_map.clone())
                        .get_key_type()?
                        .to_data_type()
                }
                _ => Err(self.not_a_stable_b_tree_map_node_error()),
            },
            _ => Err(self.not_a_stable_b_tree_map_node_error()),
        }
    }

    fn get_value_type(&self) -> KybraResult<CandidType> {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { func, .. } => {
                    SourceMapped::new(func.as_ref(), self.source_map.clone())
                        .get_value_type()?
                        .to_data_type()
                }
                _ => Err(self.not_a_stable_b_tree_map_node_error()),
            },
            _ => Err(self.not_a_stable_b_tree_map_node_error()),
        }
    }

    fn get_max_key_size(&self) -> KybraResult<u32> {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { args, keywords, .. } => {
                    if args.len() >= 2 {
                        return self.get_max_size_from_args(1, args);
                    }
                    if let Some(max_key_size) = self.get_max_size_from_keywords("key", keywords) {
                        return max_key_size;
                    }
                    Err(self.max_key_size_missing_error())
                }
                _ => Err(self.not_a_stable_b_tree_map_node_error()),
            },
            _ => Err(self.not_a_stable_b_tree_map_node_error()),
        }
    }

    fn get_max_value_size(&self) -> KybraResult<u32> {
        match &self.node {
            StmtKind::Assign { value, .. } => match &value.node {
                ExprKind::Call { args, keywords, .. } => {
                    if args.len() >= 3 {
                        return self.get_max_size_from_args(2, args);
                    }
                    if let Some(max_key_size) = self.get_max_size_from_keywords("value", keywords) {
                        return max_key_size;
                    }
                    Err(self.max_value_size_missing_error())
                }
                _ => Err(self.not_a_stable_b_tree_map_node_error()),
            },
            _ => Err(self.not_a_stable_b_tree_map_node_error()),
        }
    }

    // Helper method for get_max_key_size and get_max_value_size
    fn get_max_size_from_keywords(
        &self,
        name: &str,
        keywords: &Vec<Located<KeywordData>>,
    ) -> Option<KybraResult<u32>> {
        keywords.iter().fold(None, |act_key_type, keyword| {
            if let Some(arg_name) = &keyword.node.arg {
                if arg_name == format!("max_{}_size", name).as_str() {
                    Some(match &keyword.node.value.node {
                        ExprKind::Constant { value, .. } => match value {
                            Constant::Int(int) => self.big_int_to_max_size(int),
                            _ => Err(self.max_size_must_be_integer_constant_error()),
                        },
                        _ => Err(self.max_size_must_be_integer_constant_error()),
                    })
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

    fn get_max_size_from_args(
        &self,
        name: usize,
        keywords: &Vec<Located<ExprKind>>,
    ) -> KybraResult<u32> {
        match &keywords[name].node {
            ExprKind::Constant { value, .. } => match value {
                Constant::Int(integer) => self.big_int_to_max_size(integer),
                _ => Err(self.max_size_must_be_integer_constant_error()),
            },
            _ => Err(self.max_size_must_be_integer_constant_error()),
        }
    }

    // Helper method for get_memory_id
    fn get_memory_from_keywords(
        &self,
        keywords: &Vec<Located<KeywordData>>,
    ) -> Option<KybraResult<u8>> {
        keywords.iter().fold(None, |act_key_type, keyword| {
            let result = if let Some(arg_name) = &keyword.node.arg {
                if arg_name == "memory_id" {
                    Some(match &keyword.node.value.node {
                        ExprKind::Constant { value, .. } => match value {
                            Constant::Int(int) => self.big_int_to_memory_id(int),
                            _ => Err(self.memory_id_must_by_integer_constant_error()),
                        },
                        _ => Err(self.memory_id_must_by_integer_constant_error()),
                    })
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

    fn big_int_to_max_size(&self, num: &BigInt) -> KybraResult<u32> {
        let digits = num.to_u32_digits();
        if digits.0 == Sign::Minus {
            return Err(self.max_size_must_be_non_negative());
        }
        if digits.1.len() > 1 {
            return Err(self.max_size_too_big_error());
        }
        Ok(digits.1[0])
    }

    fn big_int_to_memory_id(&self, num: &BigInt) -> KybraResult<u8> {
        let digits = num.to_u32_digits();
        if digits.0 == Sign::Minus {
            return Err(self.memory_id_must_be_non_negative());
        }
        if digits.1.len() > 1 {
            return Err(self.memory_id_too_big_error());
        }
        if digits.1.len() == 0 {
            return Ok(0);
        }
        let value = digits.1[0];
        if value > u8::MAX as u32 {
            return Err(self.memory_id_too_big_error());
        }
        Ok(value as u8)
    }
}
