// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the vari_short_timescale table.

use crate::traits::{Column, Table};

/// This table describes the short-timescale sources.
#[allow(non_camel_case_types)]
pub struct vari_short_timescale;

impl Table for vari_short_timescale {
    fn string(&self) -> String {
        "vari_short_timescale".to_string()
    }
}

/// The columns in the vari_short_timescale table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Unique source identifier
    source_id,
    /// Amplitude estimate of all per-CCD G-band photometry (95th quantile - 5th quantile)
    amplitude_estimate,
    /// Number of FoV transits with more than 7 CCD measurements after time series cleaning
    number_of_fov_transits,
    /// Mean of per-FoV Abbe values derived from per-CCD G-band photometry
    mean_of_fov_abbe_values,
    /// Number of selected timescale(s) derived from the variogram
    variogram_num_points,
    /// Characteristic timescale(s) of variability
    variogram_char_timescales,
    /// Variogram values associated with the {\tt variogramCharTimescales}
    variogram_values,
    /// Frequency search result for either G CCD, G FoV, BP or RP photometry
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
