// This code is generated by generate_code.py, do not modify it manually
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct lens_candidates;

impl Table for lens_candidates {
    fn string(&self) -> String {
        "lens_candidates".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    name,
    flag,
    n_components,
    component_id,
    n_obs_component,
    component_flag,
    ra_component,
    ra_std_component,
    dec_component,
    dec_std_component,
    g_flux_component,
    g_flux_component_error,
    g_mag_component,
    g_mag_std_component,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::name.to_string());
    col_strings.push(Col::flag.to_string());
    col_strings.push(Col::n_components.to_string());
    col_strings.push(Col::component_id.to_string());
    col_strings.push(Col::n_obs_component.to_string());
    col_strings.push(Col::component_flag.to_string());
    col_strings.push(Col::ra_component.to_string());
    col_strings.push(Col::ra_std_component.to_string());
    col_strings.push(Col::dec_component.to_string());
    col_strings.push(Col::dec_std_component.to_string());
    col_strings.push(Col::g_flux_component.to_string());
    col_strings.push(Col::g_flux_component_error.to_string());
    col_strings.push(Col::g_mag_component.to_string());
    col_strings.push(Col::g_mag_std_component.to_string());
    map.insert(lens_candidates.string(), col_strings);
}
