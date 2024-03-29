// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct dr2_neighbourhood;

impl Table for dr2_neighbourhood {
    fn string(&self) -> String {
        "dr2_neighbourhood".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    dr2_source_id,
    dr3_source_id,
    angular_distance,
    magnitude_difference,
    proper_motion_propagation,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::dr2_source_id.to_string());
    col_strings.push(Col::dr3_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::magnitude_difference.to_string());
    col_strings.push(Col::proper_motion_propagation.to_string());
    map.insert(dr2_neighbourhood.string(), col_strings);
}
