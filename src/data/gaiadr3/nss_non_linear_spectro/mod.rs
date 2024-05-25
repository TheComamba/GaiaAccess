// This code is generated by generate_code.py, do not modify it manually
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct nss_non_linear_spectro;

impl Table for nss_non_linear_spectro {
    fn string(&self) -> String {
        "nss_non_linear_spectro".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    nss_solution_type,
    mean_velocity,
    mean_velocity_error,
    first_deriv_velocity,
    first_deriv_velocity_error,
    second_deriv_velocity,
    second_deriv_velocity_error,
    rv_n_obs_primary,
    rv_n_good_obs_primary,
    bit_index,
    corr_vec,
    obj_func,
    goodness_of_fit,
    flags,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::nss_solution_type.to_string());
    col_strings.push(Col::mean_velocity.to_string());
    col_strings.push(Col::mean_velocity_error.to_string());
    col_strings.push(Col::first_deriv_velocity.to_string());
    col_strings.push(Col::first_deriv_velocity_error.to_string());
    col_strings.push(Col::second_deriv_velocity.to_string());
    col_strings.push(Col::second_deriv_velocity_error.to_string());
    col_strings.push(Col::rv_n_obs_primary.to_string());
    col_strings.push(Col::rv_n_good_obs_primary.to_string());
    col_strings.push(Col::bit_index.to_string());
    col_strings.push(Col::corr_vec.to_string());
    col_strings.push(Col::obj_func.to_string());
    col_strings.push(Col::goodness_of_fit.to_string());
    col_strings.push(Col::flags.to_string());
    map.insert(nss_non_linear_spectro.string(), col_strings);
}
