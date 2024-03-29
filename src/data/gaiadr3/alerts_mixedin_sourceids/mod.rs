// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct alerts_mixedin_sourceids;

impl Table for alerts_mixedin_sourceids {
    fn string(&self) -> String {
        "alerts_mixedin_sourceids".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    alert_source_id,
    mixed_in_source_id,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::alert_source_id.to_string());
    col_strings.push(Col::mixed_in_source_id.to_string());
    map.insert(alerts_mixedin_sourceids.string(), col_strings);
}
