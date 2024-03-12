// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct ravedr5_join;

impl Schema for ravedr5_join {
    fn string(&self) -> String {
        "ravedr5_join".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    original_ext_source_id,
    clean_ravedr5_oid,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::clean_ravedr5_oid.to_string());
    map.insert(ravedr5_join.string(), col_strings);
}
