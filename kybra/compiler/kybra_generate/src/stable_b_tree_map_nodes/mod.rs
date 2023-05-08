pub mod errors;
pub mod rust;

use crate::{
    errors::{CollectResults, Unreachable},
    py_ast::PyAst,
    source_map::SourceMapped,
    Error,
};
use cdk_framework::act::node::CandidType;
use num_bigint::{BigInt, Sign};
use rustpython_parser::ast::{Constant, ExprKind, KeywordData, Located, StmtKind};

use self::errors::{
    InvalidMemoryId, MaxKeySizeMissing, MaxSizeMustBeInteger, MaxSizeMustBeNonNegative,
    MaxSizeTooBig, MaxValueSizeMissing, MemoryIdMustBeAnInteger, MemoryIdMustBeIntegerConstant,
    MemoryIdMustBeNonNegative, MemoryIdTooBig, MissingMemoryId, StableBTreeMapNodeFormat,
};

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
    pub fn build_stable_b_tree_map_nodes(&self) -> Result<Vec<StableBTreeMapNode>, Vec<Error>> {
        Ok(self
            .get_stmt_kinds()
            .iter()
            .map(|source_mapped_stmt_kind| source_mapped_stmt_kind.as_stable_b_tree_map_node())
            .collect_results()?
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

    pub fn get_value_type(&self) -> Result<SourceMapped<&Located<ExprKind>>, Error> {
        match &self.node {
            ExprKind::Subscript { slice, .. } => match &slice.node {
                ExprKind::Tuple { elts, .. } => {
                    Ok(SourceMapped::new(&elts[1], self.source_map.clone()))
                }
                _ => Err(StableBTreeMapNodeFormat::err_from_expr(self)),
            },
            _ => Err(Unreachable::new_err()),
        }
    }

    pub fn get_key_type(&self) -> Result<SourceMapped<&Located<ExprKind>>, Error> {
        match &self.node {
            ExprKind::Subscript { slice, .. } => match &slice.node {
                ExprKind::Tuple { elts, .. } => {
                    Ok(SourceMapped::new(&elts[0], self.source_map.clone()))
                }
                _ => Err(StableBTreeMapNodeFormat::err_from_expr(self).into()),
            },
            _ => Err(Unreachable::new_err()),
        }
    }
}

impl SourceMapped<&Located<StmtKind>> {
    pub fn is_stable_b_tree_map_node(&self) -> bool {
        match &self.node {
            StmtKind::Assign { value, .. }
            | StmtKind::AnnAssign {
                value: Some(value), ..
            } => SourceMapped::new(value.as_ref(), self.source_map.clone())
                .is_stable_b_tree_map_node(),
            _ => false,
        }
    }

    pub fn as_stable_b_tree_map_node(&self) -> Result<Option<StableBTreeMapNode>, Vec<Error>> {
        if !self.is_stable_b_tree_map_node() {
            return Ok(None);
        }
        let memory_id = match self.get_memory_id() {
            Ok(memory_id) => memory_id,
            Err(err) => return Err(err.into()),
        };
        let key_type = self.get_key_type()?;
        let value_type = self.get_value_type()?;
        let max_key_size = match self.get_max_key_size() {
            Ok(max_key_size) => max_key_size,
            Err(err) => return Err(err.into()),
        };
        let max_value_size = match self.get_max_value_size() {
            Ok(max_value_size) => max_value_size,
            Err(err) => return Err(err.into()),
        };
        Ok(Some(StableBTreeMapNode {
            memory_id,
            key_type,
            value_type,
            max_key_size,
            max_value_size,
        }))
    }

    fn get_assign_value(&self) -> Result<&Located<ExprKind>, Error> {
        match &self.node {
            StmtKind::Assign { value, .. }
            | StmtKind::AnnAssign {
                value: Some(value), ..
            } => Ok(value),
            _ => Err(Unreachable::new_err()),
        }
    }

    fn get_memory_id(&self) -> Result<u8, Error> {
        match &self.get_assign_value()?.node {
            ExprKind::Call { args, keywords, .. } => {
                if args.len() >= 1 {
                    return match &args[0].node {
                        ExprKind::Constant { value, .. } => match value {
                            Constant::Int(integer) => self.big_int_to_memory_id(integer),
                            _ => Err(MemoryIdMustBeAnInteger::err_from_stmt(self)),
                        },
                        _ => Err(InvalidMemoryId::err_from_stmt(self)),
                    };
                }
                if let Some(memory_id) = self.get_memory_from_keywords(keywords) {
                    return memory_id;
                }
                Err(MissingMemoryId::err_from_stmt(self))
            }
            _ => Err(Unreachable::new_err()),
        }
    }

    fn get_key_type(&self) -> Result<CandidType, Vec<Error>> {
        let assign_value = match self.get_assign_value() {
            Ok(assign_value) => assign_value,
            Err(err) => return Err(err.into()),
        };
        match &assign_value.node {
            ExprKind::Call { func, .. } => {
                match SourceMapped::new(func.as_ref(), self.source_map.clone()).get_key_type() {
                    Ok(key_type) => key_type,
                    Err(err) => return Err(err.into()),
                }
                .to_candid_type()
            }
            _ => Err(Unreachable::new_err().into()),
        }
    }

    fn get_value_type(&self) -> Result<CandidType, Vec<Error>> {
        let assign_value = match self.get_assign_value() {
            Ok(assign_value) => assign_value,
            Err(err) => return Err(err.into()),
        };
        match &assign_value.node {
            ExprKind::Call { func, .. } => {
                match SourceMapped::new(func.as_ref(), self.source_map.clone()).get_value_type() {
                    Ok(value_type) => value_type,
                    Err(err) => return Err(err.into()),
                }
                .to_candid_type()
            }
            _ => Err(Unreachable::new_err().into()),
        }
    }

    fn get_max_key_size(&self) -> Result<u32, Error> {
        match &self.get_assign_value()?.node {
            ExprKind::Call { args, keywords, .. } => {
                if args.len() >= 2 {
                    return self.get_max_size_from_args(1, args);
                }
                if let Some(max_key_size) = self.get_max_size_from_keywords("key", keywords) {
                    return max_key_size;
                }
                Err(MaxKeySizeMissing::err_from_stmt(self))
            }
            _ => Err(Unreachable::new_err()),
        }
    }

    fn get_max_value_size(&self) -> Result<u32, Error> {
        match &self.get_assign_value()?.node {
            ExprKind::Call { args, keywords, .. } => {
                if args.len() >= 3 {
                    return self.get_max_size_from_args(2, args);
                }
                if let Some(max_key_size) = self.get_max_size_from_keywords("value", keywords) {
                    return max_key_size;
                }
                Err(MaxValueSizeMissing::err_from_stmt(self))
            }
            _ => Err(Unreachable::new_err()),
        }
    }

    // Helper method for get_max_key_size and get_max_value_size
    fn get_max_size_from_keywords(
        &self,
        name: &str,
        keywords: &Vec<Located<KeywordData>>,
    ) -> Option<Result<u32, Error>> {
        keywords.iter().fold(None, |act_key_type, keyword| {
            if let Some(arg_name) = &keyword.node.arg {
                if arg_name == format!("max_{}_size", name).as_str() {
                    Some(match &keyword.node.value.node {
                        ExprKind::Constant { value, .. } => match value {
                            Constant::Int(int) => self.big_int_to_max_size(int),
                            _ => Err(MaxSizeMustBeInteger::err_from_stmt(self)),
                        },
                        _ => Err(MaxSizeMustBeInteger::err_from_stmt(self)),
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
    ) -> Result<u32, Error> {
        match &keywords[name].node {
            ExprKind::Constant { value, .. } => match value {
                Constant::Int(integer) => self.big_int_to_max_size(integer),
                _ => Err(MaxSizeMustBeInteger::err_from_stmt(self)),
            },
            _ => Err(MaxSizeMustBeInteger::err_from_stmt(self)),
        }
    }

    // Helper method for get_memory_id
    fn get_memory_from_keywords(
        &self,
        keywords: &Vec<Located<KeywordData>>,
    ) -> Option<Result<u8, Error>> {
        keywords.iter().fold(None, |act_key_type, keyword| {
            let result = if let Some(arg_name) = &keyword.node.arg {
                if arg_name == "memory_id" {
                    Some(match &keyword.node.value.node {
                        ExprKind::Constant { value, .. } => match value {
                            Constant::Int(int) => self.big_int_to_memory_id(int),
                            _ => Err(MemoryIdMustBeIntegerConstant::err_from_stmt(self)),
                        },
                        _ => Err(MemoryIdMustBeIntegerConstant::err_from_stmt(self)),
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

    fn big_int_to_max_size(&self, num: &BigInt) -> Result<u32, Error> {
        let digits = num.to_u32_digits();
        if digits.0 == Sign::Minus {
            return Err(MaxSizeMustBeNonNegative::err_from_stmt(self));
        }
        if digits.1.len() > 1 {
            return Err(MaxSizeTooBig::err_from_stmt(self));
        }
        Ok(digits.1[0])
    }

    fn big_int_to_memory_id(&self, num: &BigInt) -> Result<u8, Error> {
        let digits = num.to_u32_digits();
        if digits.0 == Sign::Minus {
            return Err(MemoryIdMustBeNonNegative::err_from_stmt(self));
        }
        if digits.1.len() > 1 {
            return Err(MemoryIdTooBig::err_from_stmt(self));
        }
        if digits.1.len() == 0 {
            return Ok(0);
        }
        let value = digits.1[0];
        if value > u8::MAX as u32 {
            return Err(MemoryIdTooBig::err_from_stmt(self));
        }
        Ok(value as u8)
    }
}
