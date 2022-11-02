use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use super::kybra_types::KybraStmt;

pub trait GenerateInlineName
where
    Self: Hash,
{
    fn generate_inline_name(&self) -> String;
    fn calculate_hash(&self) -> String {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        format!("{}", s.finish()).to_string()
    }
}

pub trait GetDependencies {
    fn get_dependent_types(
        &self,
        type_alias_lookup: &HashMap<String, KybraStmt>,
        found_type_names: &HashSet<String>,
    ) -> HashSet<String>;
}
