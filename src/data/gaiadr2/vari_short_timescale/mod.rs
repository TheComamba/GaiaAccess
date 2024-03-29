// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct vari_short_timescale;

impl Table for vari_short_timescale {
    fn string(&self) -> String {
        "vari_short_timescale".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    amplitude_estimate,
    number_of_fov_transits,
    mean_of_fov_abbe_values,
    variogram_num_points,
    variogram_char_timescales,
    variogram_values,
    frequency,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::amplitude_estimate.to_string());
    col_strings.push(Col::number_of_fov_transits.to_string());
    col_strings.push(Col::mean_of_fov_abbe_values.to_string());
    col_strings.push(Col::variogram_num_points.to_string());
    col_strings.push(Col::variogram_char_timescales.to_string());
    col_strings.push(Col::variogram_values.to_string());
    col_strings.push(Col::frequency.to_string());
    map.insert(vari_short_timescale.string(), col_strings);
}
