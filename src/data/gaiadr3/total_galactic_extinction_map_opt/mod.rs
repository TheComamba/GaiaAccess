// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the total_galactic_extinction_map_opt table.

use crate::traits::{Column, Table};

/// This table provides an optimum version of the Total Galactic Extinction map, derived from the table {{TotalGalacticExtinctionMap}} at a single HEALPix level 9. For this, the mean effective total Galactic extinction and related uncertainties have been selected from the optimal HEALPix level of the four offered in {{TotalGalacticExtinctionMap}}. For further details see Section \ref{ssec:cu8par_apsis_tge} of the online documentation.
#[allow(non_camel_case_types)]
pub struct total_galactic_extinction_map_opt;

impl Table for total_galactic_extinction_map_opt {
    fn string(&self) -> String {
        "total_galactic_extinction_map_opt".to_string()
    }
}

/// The columns in the total_galactic_extinction_map_opt table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// HEALPix identification
    healpix_id,
    /// Median A$_0$ extinction parameter
    a0,
    /// Uncertainty for the mean A$_0$
    a0_uncertainty,
    /// Number of tracers used
    num_tracers_used,
    /// Exit status for TGE
    status,
    /// Number indicating which HEALPix level was chosen to populate this HEALPix
    optimum_hpx_level,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::healpix_id.to_string());
    col_strings.push(Col::a0.to_string());
    col_strings.push(Col::a0_uncertainty.to_string());
    col_strings.push(Col::num_tracers_used.to_string());
    col_strings.push(Col::status.to_string());
    col_strings.push(Col::optimum_hpx_level.to_string());
    map.insert(total_galactic_extinction_map_opt.string(), col_strings);
}
