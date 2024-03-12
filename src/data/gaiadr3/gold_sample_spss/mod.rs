// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct gold_sample_spss;

impl Schema for gold_sample_spss {
    fn string(&self) -> String {
        "gold_sample_spss".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    spss_id,
    spss_name,
    spectraltype,
    bin_flag,
    var_flag,
    teff,
    teff_error,
    logg,
    logg_error,
    feh,
    feh_error,
    alphafe,
    alphafe_error,
    radius,
    radius_error,
    lum,
    lum_error,
    mass,
    mass_error,
    age,
    age_error,
    azero,
    azero_error,
    ag,
    ag_error,
    ebpminrp,
    ebpminrp_error,
    distancepc,
    distancepc_error,
    radial_velocity,
    radial_velocity_error,
    vsini,
    vsin_error,
    notes,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::spss_id.to_string());
    col_strings.push(Col::spss_name.to_string());
    col_strings.push(Col::spectraltype.to_string());
    col_strings.push(Col::bin_flag.to_string());
    col_strings.push(Col::var_flag.to_string());
    col_strings.push(Col::teff.to_string());
    col_strings.push(Col::teff_error.to_string());
    col_strings.push(Col::logg.to_string());
    col_strings.push(Col::logg_error.to_string());
    col_strings.push(Col::feh.to_string());
    col_strings.push(Col::feh_error.to_string());
    col_strings.push(Col::alphafe.to_string());
    col_strings.push(Col::alphafe_error.to_string());
    col_strings.push(Col::radius.to_string());
    col_strings.push(Col::radius_error.to_string());
    col_strings.push(Col::lum.to_string());
    col_strings.push(Col::lum_error.to_string());
    col_strings.push(Col::mass.to_string());
    col_strings.push(Col::mass_error.to_string());
    col_strings.push(Col::age.to_string());
    col_strings.push(Col::age_error.to_string());
    col_strings.push(Col::azero.to_string());
    col_strings.push(Col::azero_error.to_string());
    col_strings.push(Col::ag.to_string());
    col_strings.push(Col::ag_error.to_string());
    col_strings.push(Col::ebpminrp.to_string());
    col_strings.push(Col::ebpminrp_error.to_string());
    col_strings.push(Col::distancepc.to_string());
    col_strings.push(Col::distancepc_error.to_string());
    col_strings.push(Col::radial_velocity.to_string());
    col_strings.push(Col::radial_velocity_error.to_string());
    col_strings.push(Col::vsini.to_string());
    col_strings.push(Col::vsin_error.to_string());
    col_strings.push(Col::notes.to_string());
    map.insert(gold_sample_spss.string(), col_strings);
}
