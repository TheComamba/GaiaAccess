// This code is generated by generate_code.py, do not modify it manually
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct science_alerts;

impl Table for science_alerts {
    fn string(&self) -> String {
        "science_alerts".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    transit_id,
    name,
    solution_id,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::transit_id.to_string());
    col_strings.push(Col::name.to_string());
    col_strings.push(Col::solution_id.to_string());
    map.insert(science_alerts.string(), col_strings);
}
