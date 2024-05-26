// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gaia_source_simulation table.

use crate::traits::{Column, Table};

/// Table of sources realised according to the Gaia Object Generator (GOG)
/// simulation. Observed attributes are given with simulated observational
/// uncertainties.
///
/// This table contains the output of GOG. The values are obtained after
/// adding the corresponding uncertainty (based on the error models) to the
/// true values in table GaiaUniverseModel. Both the values and the
/// uncertainties are provided.
#[allow(non_camel_case_types)]
pub struct gaia_source_simulation;

impl Table for gaia_source_simulation {
    fn string(&self) -> String {
        "gaia_source_simulation".to_string()
    }
}

/// The columns in the gaia_source_simulation table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Long Identifier
    source_id,
    /// Right Ascension
    ra,
    /// Right Ascension error
    ra_error,
    /// Declination
    dec,
    /// Declination error
    dec_error,
    /// Parallax
    parallax,
    /// Parallax error
    parallax_error,
    /// Proper motion in RA
    pmra,
    /// Error in RA proper motion
    pmra_error,
    /// Proper motion in dec
    pmdec,
    /// Error in dec. proper motion
    pmdec_error,
    /// Number of AL observations
    n_obs_al,
    /// Number of outliers AL observations
    n_outliers_al,
    /// Mean G flux
    phot_g_mean_flux,
    /// Mean G flux error
    phot_g_mean_flux_error,
    /// Mean G magnitude
    phot_g_mean_mag,
    /// Mean BP flux
    phot_bp_mean_flux,
    /// Mean BP flux error
    phot_bp_mean_flux_error,
    /// Mean BP magnitude
    phot_bp_mean_mag,
    /// Mean RP flux
    phot_rp_mean_flux,
    /// Mean RP flux error
    phot_rp_mean_flux_error,
    /// Mean RP magnitude
    phot_rp_mean_mag,
    /// Mean RVS flux
    phot_rvs_mean_flux,
    /// Mean RVS flux error
    phot_rvs_mean_flux_error,
    /// Mean RVS magnitude
    phot_rvs_mean_mag,
    /// Radial velocity
    radial_velocity,
    /// Radial velocity error
    radial_velocity_error,
    /// Effective temperature
    teff,
    /// Effective temperature error
    teff_error,
    /// v sin i
    vsini,
    /// v sin i error
    vsini_error,
    /// Extinction at 550 nm
    a0,
    /// Extinction at 550 nm error
    a0_error,
    /// Iron abundance
    feh,
    /// Iron abundance error
    feh_error,
    /// Surface gravity
    logg,
    /// Surface gravity Error
    logg_error,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
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
    col_strings.push(Col::n_obs_al.to_string());
    col_strings.push(Col::n_outliers_al.to_string());
    col_strings.push(Col::phot_g_mean_flux.to_string());
    col_strings.push(Col::phot_g_mean_flux_error.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::phot_bp_mean_flux.to_string());
    col_strings.push(Col::phot_bp_mean_flux_error.to_string());
    col_strings.push(Col::phot_bp_mean_mag.to_string());
    col_strings.push(Col::phot_rp_mean_flux.to_string());
    col_strings.push(Col::phot_rp_mean_flux_error.to_string());
    col_strings.push(Col::phot_rp_mean_mag.to_string());
    col_strings.push(Col::phot_rvs_mean_flux.to_string());
    col_strings.push(Col::phot_rvs_mean_flux_error.to_string());
    col_strings.push(Col::phot_rvs_mean_mag.to_string());
    col_strings.push(Col::radial_velocity.to_string());
    col_strings.push(Col::radial_velocity_error.to_string());
    col_strings.push(Col::teff.to_string());
    col_strings.push(Col::teff_error.to_string());
    col_strings.push(Col::vsini.to_string());
    col_strings.push(Col::vsini_error.to_string());
    col_strings.push(Col::a0.to_string());
    col_strings.push(Col::a0_error.to_string());
    col_strings.push(Col::feh.to_string());
    col_strings.push(Col::feh_error.to_string());
    col_strings.push(Col::logg.to_string());
    col_strings.push(Col::logg_error.to_string());
    map.insert(gaia_source_simulation.string(), col_strings);
}
