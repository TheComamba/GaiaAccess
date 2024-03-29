// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct sso_source;

impl Table for sso_source {
    fn string(&self) -> String {
        "sso_source".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    num_of_obs,
    number_mp,
    denomination,
    num_of_spectra,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::num_of_obs.to_string());
    col_strings.push(Col::number_mp.to_string());
    col_strings.push(Col::denomination.to_string());
    col_strings.push(Col::num_of_spectra.to_string());
    map.insert(sso_source.string(), col_strings);
}
