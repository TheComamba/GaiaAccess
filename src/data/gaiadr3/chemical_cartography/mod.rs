// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the chemical_cartography table.

use crate::traits::{Column, Table};

/// Orbital parameters, actions, Galactocentric Cartesian coordinates and velocities for the sample of stars used in the DR3 chemical cartographic study of the Milky Way.
///
/// For further details see \cite{DR3-DPACP-104}.
#[allow(non_camel_case_types)]
pub struct chemical_cartography;

impl Table for chemical_cartography {
    fn string(&self) -> String {
        "chemical_cartography".to_string()
    }
}

/// The columns in the chemical_cartography table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Source Identifier
    source_id,
    /// Median radial action per unit mass in units of 1967.3865 km/s kpc computed assuming the Staeckel approximation.
    jr_med,
    /// Median vertical action per unit mass in units of 1967.3865 km/s kpc computed assuming the Staeckel approximation.
    jz_med,
    /// Median azimuthal action per unit mass (positive clockwise) in units of 1967.3865 km/s kpc. Equivalent to the median vertical component of the angular momentum.
    jphi_med,
    /// Median in-plane projection of the Galactocentric distance.
    rplane_med,
    /// Median Galactocentric radial velocity.
    vrplane_med,
    /// Median vertical velocity.
    vz_med,
    /// Median Galactocentric azimuthal velocity (positive clockwise).
    vphi_med,
    /// Median maximum Galactic height (in absolute value) computed assuming the Staeckel approximation.
    zmax_med,
    /// Median of the maximum Galactocentric distance computed assuming the Staeckel approximation.
    rapo_med,
    /// Median of the minimum Galactocentric distance computed assuming the Staeckel approximation.
    rperi_med,
    /// Median orbital eccentricity computed assuming the Staeckel approximation.
    ecc_med,
    /// Median of the Cartesian coordinate X.
    x_med,
    /// Median of the Cartesian coordinate Y.
    y_med,
    /// Median of the Cartesian coordinate Z.
    z_med,
    /// Total energy per unit mass assuming median coordinates and velocities.
    energy_med,
    /// Upper confidence limit of the radial action per unit mass in units of 1967.3865 km/s kpc computed assuming the Staeckel approximation.
    jr_hi,
    /// Upper confidence limit of the vertical action per unit mass in units of 1967.3865 km/s kpc computed assuming the Staeckel approximation.
    jz_hi,
    /// Upper confidence limit of the azimuthal action per unit mass (positive clockwise) in units of 1967.3865 km/s kpc.
    jphi_hi,
    /// Upper confidence limit of the in-plane projection of the Galactocentric distance.
    rplane_hi,
    /// Upper confidence limit of the Galactocentric radial velocity.
    vrplane_hi,
    /// Upper confidence limit of the vertical velocity.
    vz_hi,
    /// Upper confidence limit of the Galactocentric azimuthal velocity (positive clockwise). Equivalent to the upper confidence limit of the vertical component of the angular momentum.
    vphi_hi,
    /// Upper confidence limit of the maximum Galactic height (in absolute value) computed assuming the Staeckel approximation.
    zmax_hi,
    /// Upper confidence limit of the of the maximum Galactocentric distance computed assuming the Staeckel approximation.
    rapo_hi,
    /// Upper confidence limit of the of the minimum Galactocentric distance computed assuming the Staeckel approximation.
    rperi_hi,
    /// Upper confidence limit of the orbital eccentricity computed assuming the Staeckel approximation.
    ecc_hi,
    /// Upper confidence limit of the Cartesian coordinate X.
    x_hi,
    /// Upper confidence limit of the Cartesian coordinate Y.
    y_hi,
    /// Upper confidence limit of the Cartesian coordinate Z.
    z_hi,
    /// Lower confidence limit of the radial action per unit mass in units of 1967.3865 km/s kpc computed assuming the Staeckel approximation.
    jr_lo,
    /// Lower confidence limit of the vertical action per unit mass in units of 1967.3865 km/s kpc computed assuming the Staeckel approximation.
    jz_lo,
    /// Lower confidence limit of the azimuthal action per unit mass (positive clockwise) in units of 1967.3865 km/s kpc. Equivalent to the lower confidence limit of the vertical component of the angular momentum.
    jphi_lo,
    /// Lower confidence limit of the in-plane projection of the Galactocentric distance.
    rplane_lo,
    /// Lower confidence limit of the Galactocentric radial velocity.
    vrplane_lo,
    /// Lower confidence limit of the vertical velocity.
    vz_lo,
    /// Lower confidence limit of the Galactocentric azimuthal velocity (positive clockwise).
    vphi_lo,
    /// Lower confidence limit of the maximum Galactic height (in absolute value) computed assuming the Staeckel approximation.
    zmax_lo,
    /// Lower confidence limit of the of the maximum Galactocentric distance computed assuming the Staeckel approximation.
    rapo_lo,
    /// Lower confidence limit of the of the minimum Galactocentric distance computed assuming the Staeckel approximation.
    rperi_lo,
    /// Lower confidence limit of the orbital eccentricity computed assuming the Staeckel approximation.
    ecc_lo,
    /// Lower confidence limit of the Cartesian coordinate X.
    x_lo,
    /// Lower confidence limit of the Cartesian coordinate Y.
    y_lo,
    /// Lower confidence limit of the Cartesian coordinate Z.
    z_lo,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::jr_med.to_string());
    col_strings.push(Col::jz_med.to_string());
    col_strings.push(Col::jphi_med.to_string());
    col_strings.push(Col::rplane_med.to_string());
    col_strings.push(Col::vrplane_med.to_string());
    col_strings.push(Col::vz_med.to_string());
    col_strings.push(Col::vphi_med.to_string());
    col_strings.push(Col::zmax_med.to_string());
    col_strings.push(Col::rapo_med.to_string());
    col_strings.push(Col::rperi_med.to_string());
    col_strings.push(Col::ecc_med.to_string());
    col_strings.push(Col::x_med.to_string());
    col_strings.push(Col::y_med.to_string());
    col_strings.push(Col::z_med.to_string());
    col_strings.push(Col::energy_med.to_string());
    col_strings.push(Col::jr_hi.to_string());
    col_strings.push(Col::jz_hi.to_string());
    col_strings.push(Col::jphi_hi.to_string());
    col_strings.push(Col::rplane_hi.to_string());
    col_strings.push(Col::vrplane_hi.to_string());
    col_strings.push(Col::vz_hi.to_string());
    col_strings.push(Col::vphi_hi.to_string());
    col_strings.push(Col::zmax_hi.to_string());
    col_strings.push(Col::rapo_hi.to_string());
    col_strings.push(Col::rperi_hi.to_string());
    col_strings.push(Col::ecc_hi.to_string());
    col_strings.push(Col::x_hi.to_string());
    col_strings.push(Col::y_hi.to_string());
    col_strings.push(Col::z_hi.to_string());
    col_strings.push(Col::jr_lo.to_string());
    col_strings.push(Col::jz_lo.to_string());
    col_strings.push(Col::jphi_lo.to_string());
    col_strings.push(Col::rplane_lo.to_string());
    col_strings.push(Col::vrplane_lo.to_string());
    col_strings.push(Col::vz_lo.to_string());
    col_strings.push(Col::vphi_lo.to_string());
    col_strings.push(Col::zmax_lo.to_string());
    col_strings.push(Col::rapo_lo.to_string());
    col_strings.push(Col::rperi_lo.to_string());
    col_strings.push(Col::ecc_lo.to_string());
    col_strings.push(Col::x_lo.to_string());
    col_strings.push(Col::y_lo.to_string());
    col_strings.push(Col::z_lo.to_string());
    map.insert(chemical_cartography.string(), col_strings);
}
