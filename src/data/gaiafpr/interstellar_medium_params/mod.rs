// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct interstellar_medium_params;

impl Table for interstellar_medium_params {
    fn string(&self) -> String {
        "interstellar_medium_params".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    healpix,
    lc,
    bc,
    dc,
    n_targets,
    snr,
    ew8620,
    ew8620_lower,
    ew8620_upper,
    flags8620,
    p08620,
    p08620_lower,
    p08620_upper,
    p18620,
    p18620_lower,
    p18620_upper,
    p28620,
    p28620_lower,
    p28620_upper,
    ew8648,
    ew8648_lower,
    ew8648_upper,
    flags8648,
    p08648,
    p08648_lower,
    p08648_upper,
    p18648,
    p18648_lower,
    p18648_upper,
    p28648,
    p28648_lower,
    p28648_upper,
    dibcont_a0,
    dibcont_a0_lower,
    dibcont_a0_upper,
    dibcont_a1,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::healpix.to_string());
    col_strings.push(Col::lc.to_string());
    col_strings.push(Col::bc.to_string());
    col_strings.push(Col::dc.to_string());
    col_strings.push(Col::n_targets.to_string());
    col_strings.push(Col::snr.to_string());
    col_strings.push(Col::ew8620.to_string());
    col_strings.push(Col::ew8620_lower.to_string());
    col_strings.push(Col::ew8620_upper.to_string());
    col_strings.push(Col::flags8620.to_string());
    col_strings.push(Col::p08620.to_string());
    col_strings.push(Col::p08620_lower.to_string());
    col_strings.push(Col::p08620_upper.to_string());
    col_strings.push(Col::p18620.to_string());
    col_strings.push(Col::p18620_lower.to_string());
    col_strings.push(Col::p18620_upper.to_string());
    col_strings.push(Col::p28620.to_string());
    col_strings.push(Col::p28620_lower.to_string());
    col_strings.push(Col::p28620_upper.to_string());
    col_strings.push(Col::ew8648.to_string());
    col_strings.push(Col::ew8648_lower.to_string());
    col_strings.push(Col::ew8648_upper.to_string());
    col_strings.push(Col::flags8648.to_string());
    col_strings.push(Col::p08648.to_string());
    col_strings.push(Col::p08648_lower.to_string());
    col_strings.push(Col::p08648_upper.to_string());
    col_strings.push(Col::p18648.to_string());
    col_strings.push(Col::p18648_lower.to_string());
    col_strings.push(Col::p18648_upper.to_string());
    col_strings.push(Col::p28648.to_string());
    col_strings.push(Col::p28648_lower.to_string());
    col_strings.push(Col::p28648_upper.to_string());
    col_strings.push(Col::dibcont_a0.to_string());
    col_strings.push(Col::dibcont_a0_lower.to_string());
    col_strings.push(Col::dibcont_a0_upper.to_string());
    col_strings.push(Col::dibcont_a1.to_string());
    map.insert(interstellar_medium_params.string(), col_strings);
}