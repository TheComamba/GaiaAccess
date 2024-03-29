// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct ext_phot_zero_point;

impl Table for ext_phot_zero_point {
    fn string(&self) -> String {
        "ext_phot_zero_point".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    g_mag_zero_point,
    g_mag_zero_point_error,
    bp_mag_zero_point,
    bp_mag_zero_point_error,
    rp_mag_zero_point,
    rp_mag_zero_point_error,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::g_mag_zero_point.to_string());
    col_strings.push(Col::g_mag_zero_point_error.to_string());
    col_strings.push(Col::bp_mag_zero_point.to_string());
    col_strings.push(Col::bp_mag_zero_point_error.to_string());
    col_strings.push(Col::rp_mag_zero_point.to_string());
    col_strings.push(Col::rp_mag_zero_point_error.to_string());
    map.insert(ext_phot_zero_point.string(), col_strings);
}
