// This code is generated by generate_code.py, do not modify it manually
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct schemas;

impl Table for schemas {
    fn string(&self) -> String {
        "schemas".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    db_schema_name,
    description,
    schema_index,
    schema_name,
    utype,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::db_schema_name.to_string());
    col_strings.push(Col::description.to_string());
    col_strings.push(Col::schema_index.to_string());
    col_strings.push(Col::schema_name.to_string());
    col_strings.push(Col::utype.to_string());
    map.insert(schemas.string(), col_strings);
}
