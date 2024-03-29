// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct vari_agn;

impl Table for vari_agn {
    fn string(&self) -> String {
        "vari_agn".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    fractional_variability_g,
    structure_function_index,
    structure_function_index_scatter,
    qso_variability,
    non_qso_variability,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::fractional_variability_g.to_string());
    col_strings.push(Col::structure_function_index.to_string());
    col_strings.push(Col::structure_function_index_scatter.to_string());
    col_strings.push(Col::qso_variability.to_string());
    col_strings.push(Col::non_qso_variability.to_string());
    map.insert(vari_agn.string(), col_strings);
}
