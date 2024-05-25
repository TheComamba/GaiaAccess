// This code is generated by generate_code.py, do not modify it manually
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct ravedr6_neighbourhood;

impl Table for ravedr6_neighbourhood {
    fn string(&self) -> String {
        "ravedr6_neighbourhood".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    original_ext_source_id,
    angular_distance,
    score,
    xm_flag,
    ravedr6_oid,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::score.to_string());
    col_strings.push(Col::xm_flag.to_string());
    col_strings.push(Col::ravedr6_oid.to_string());
    map.insert(ravedr6_neighbourhood.string(), col_strings);
}
