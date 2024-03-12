
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct nss_acceleration_astro;

impl Schema for nss_acceleration_astro {
    fn string(&self) -> String {
        "nss_acceleration_astro".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
source_id,
nss_solution_type,
ra,
ra_error,
dec,
dec_error,
parallax,
parallax_error,
pmra,
pmra_error,
pmdec,
pmdec_error,
accel_ra,
accel_ra_error,
accel_dec,
accel_dec_error,
deriv_accel_ra,
deriv_accel_ra_error,
deriv_accel_dec,
deriv_accel_dec_error,
astrometric_n_obs_al,
astrometric_n_good_obs_al,
bit_index,
corr_vec,
obj_func,
goodness_of_fit,
significance,
flags,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::solution_id.to_string());
col_strings.push(Col::source_id.to_string());
col_strings.push(Col::nss_solution_type.to_string());
col_strings.push(Col::ra.to_string());
col_strings.push(Col::ra_error.to_string());
col_strings.push(Col::dec.to_string());
col_strings.push(Col::dec_error.to_string());
col_strings.push(Col::parallax.to_string());
col_strings.push(Col::parallax_error.to_string());
col_strings.push(Col::pmra.to_string());
col_strings.push(Col::pmra_error.to_string());
col_strings.push(Col::pmdec.to_string());
col_strings.push(Col::pmdec_error.to_string());
col_strings.push(Col::accel_ra.to_string());
col_strings.push(Col::accel_ra_error.to_string());
col_strings.push(Col::accel_dec.to_string());
col_strings.push(Col::accel_dec_error.to_string());
col_strings.push(Col::deriv_accel_ra.to_string());
col_strings.push(Col::deriv_accel_ra_error.to_string());
col_strings.push(Col::deriv_accel_dec.to_string());
col_strings.push(Col::deriv_accel_dec_error.to_string());
col_strings.push(Col::astrometric_n_obs_al.to_string());
col_strings.push(Col::astrometric_n_good_obs_al.to_string());
col_strings.push(Col::bit_index.to_string());
col_strings.push(Col::corr_vec.to_string());
col_strings.push(Col::obj_func.to_string());
col_strings.push(Col::goodness_of_fit.to_string());
col_strings.push(Col::significance.to_string());
col_strings.push(Col::flags.to_string());
    map.insert(nss_acceleration_astro.string(), col_strings);
}
