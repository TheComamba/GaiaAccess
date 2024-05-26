// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the lens_observation table.

use crate::traits::{Column, Table};

/// This table contains the observations associated with the components found in the lens candidates table.
#[allow(non_camel_case_types)]
pub struct lens_observation;

impl Table for lens_observation {
    fn string(&self) -> String {
        "lens_observation".to_string()
    }
}

/// The columns in the lens_observation table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Unique source identifier
    source_id,
    /// Index of the component for this sourceId
    component_id,
    /// Counter for the observations of each component
    observation_id,
    /// Right ascension of each individual observation belonging to this component, as decoded from the transitId
    ra_obs,
    /// Declination of each individual observation belonging to this component, as decoded from the transitId
    dec_obs,
    /// Flux value of each individual observation belonging to this component
    g_flux_obs,
    /// Flux error value of each individual observation belonging to this component
    g_flux_obs_error,
    /// Onboard G magnitudes of each individual observation belonging to this component
    g_mag_obs,
    /// Epoch of the individual observation belonging to this component
    epoch_obs,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::component_id.to_string());
    col_strings.push(Col::observation_id.to_string());
    col_strings.push(Col::ra_obs.to_string());
    col_strings.push(Col::dec_obs.to_string());
    col_strings.push(Col::g_flux_obs.to_string());
    col_strings.push(Col::g_flux_obs_error.to_string());
    col_strings.push(Col::g_mag_obs.to_string());
    col_strings.push(Col::epoch_obs.to_string());
    map.insert(lens_observation.string(), col_strings);
}
