
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct keys;

impl Schema for keys {
    fn string(&self) -> String {
        "keys".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    description,
from_table,
key_id,
target_table,
utype,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::description.to_string());
col_strings.push(Col::from_table.to_string());
col_strings.push(Col::key_id.to_string());
col_strings.push(Col::target_table.to_string());
col_strings.push(Col::utype.to_string());
    map.insert(keys.string(), col_strings);
}
