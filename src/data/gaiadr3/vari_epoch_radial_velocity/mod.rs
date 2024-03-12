// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct vari_epoch_radial_velocity;

impl Schema for vari_epoch_radial_velocity {
    fn string(&self) -> String {
        "vari_epoch_radial_velocity".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    transit_id,
    rv_obs_time,
    radial_velocity,
    radial_velocity_error,
    rejected_by_variability,
    solution_id,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::transit_id.to_string());
    col_strings.push(Col::rv_obs_time.to_string());
    col_strings.push(Col::radial_velocity.to_string());
    col_strings.push(Col::radial_velocity_error.to_string());
    col_strings.push(Col::rejected_by_variability.to_string());
    col_strings.push(Col::solution_id.to_string());
    map.insert(vari_epoch_radial_velocity.string(), col_strings);
}
