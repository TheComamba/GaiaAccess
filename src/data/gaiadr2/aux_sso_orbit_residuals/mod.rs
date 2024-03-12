
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct aux_sso_orbit_residuals;

impl Schema for aux_sso_orbit_residuals {
    fn string(&self) -> String {
        "aux_sso_orbit_residuals".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
transit_id,
observation_id,
number_mp,
epoch,
residual_ra,
residual_dec,
residual_al,
residual_ac,
selected,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::source_id.to_string());
col_strings.push(Col::transit_id.to_string());
col_strings.push(Col::observation_id.to_string());
col_strings.push(Col::number_mp.to_string());
col_strings.push(Col::epoch.to_string());
col_strings.push(Col::residual_ra.to_string());
col_strings.push(Col::residual_dec.to_string());
col_strings.push(Col::residual_al.to_string());
col_strings.push(Col::residual_ac.to_string());
col_strings.push(Col::selected.to_string());
    map.insert(aux_sso_orbit_residuals.string(), col_strings);
}
