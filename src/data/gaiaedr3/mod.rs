// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known tables in the gaiaedr3 schema.

use crate::traits::Schema;

/// Gaia Early Data Release 3
#[allow(non_camel_case_types)]
pub struct gaiaedr3;

impl Schema for gaiaedr3 {
    fn string(&self) -> String {
        "gaiaedr3".to_string()
    }
}

#[cfg(any(feature = "gaiaedr3_agn_cross_id", test))]
pub mod agn_cross_id;
#[cfg(any(feature = "gaiaedr3_allwise_best_neighbour", test))]
pub mod allwise_best_neighbour;
#[cfg(any(feature = "gaiaedr3_allwise_neighbourhood", test))]
pub mod allwise_neighbourhood;
#[cfg(any(feature = "gaiaedr3_apassdr9_best_neighbour", test))]
pub mod apassdr9_best_neighbour;
#[cfg(any(feature = "gaiaedr3_apassdr9_join", test))]
pub mod apassdr9_join;
#[cfg(any(feature = "gaiaedr3_apassdr9_neighbourhood", test))]
pub mod apassdr9_neighbourhood;
#[cfg(any(feature = "gaiaedr3_commanded_scan_law", test))]
pub mod commanded_scan_law;
#[cfg(any(feature = "gaiaedr3_dr2_neighbourhood", test))]
pub mod dr2_neighbourhood;
#[cfg(any(feature = "gaiaedr3_frame_rotator_source", test))]
pub mod frame_rotator_source;
#[cfg(any(feature = "gaiaedr3_gaia_source", test))]
pub mod gaia_source;
#[cfg(any(feature = "gaiaedr3_gaia_source_simulation", test))]
pub mod gaia_source_simulation;
#[cfg(any(feature = "gaiaedr3_gaia_universe_model", test))]
pub mod gaia_universe_model;
#[cfg(any(feature = "gaiaedr3_gsc23_best_neighbour", test))]
pub mod gsc23_best_neighbour;
#[cfg(any(feature = "gaiaedr3_gsc23_join", test))]
pub mod gsc23_join;
#[cfg(any(feature = "gaiaedr3_gsc23_neighbourhood", test))]
pub mod gsc23_neighbourhood;
#[cfg(any(feature = "gaiaedr3_hipparcos2_best_neighbour", test))]
pub mod hipparcos2_best_neighbour;
#[cfg(any(feature = "gaiaedr3_hipparcos2_neighbourhood", test))]
pub mod hipparcos2_neighbourhood;
#[cfg(any(feature = "gaiaedr3_panstarrs1_best_neighbour", test))]
pub mod panstarrs1_best_neighbour;
#[cfg(any(feature = "gaiaedr3_panstarrs1_join", test))]
pub mod panstarrs1_join;
#[cfg(any(feature = "gaiaedr3_panstarrs1_neighbourhood", test))]
pub mod panstarrs1_neighbourhood;
#[cfg(any(feature = "gaiaedr3_ravedr5_best_neighbour", test))]
pub mod ravedr5_best_neighbour;
#[cfg(any(feature = "gaiaedr3_ravedr5_join", test))]
pub mod ravedr5_join;
#[cfg(any(feature = "gaiaedr3_ravedr5_neighbourhood", test))]
pub mod ravedr5_neighbourhood;
#[cfg(any(feature = "gaiaedr3_sdssdr13_best_neighbour", test))]
pub mod sdssdr13_best_neighbour;
#[cfg(any(feature = "gaiaedr3_sdssdr13_join", test))]
pub mod sdssdr13_join;
#[cfg(any(feature = "gaiaedr3_sdssdr13_neighbourhood", test))]
pub mod sdssdr13_neighbourhood;
#[cfg(any(feature = "gaiaedr3_skymapperdr2_best_neighbour", test))]
pub mod skymapperdr2_best_neighbour;
#[cfg(any(feature = "gaiaedr3_skymapperdr2_join", test))]
pub mod skymapperdr2_join;
#[cfg(any(feature = "gaiaedr3_skymapperdr2_neighbourhood", test))]
pub mod skymapperdr2_neighbourhood;
#[cfg(any(feature = "gaiaedr3_tmass_psc_xsc_best_neighbour", test))]
pub mod tmass_psc_xsc_best_neighbour;
#[cfg(any(feature = "gaiaedr3_tmass_psc_xsc_join", test))]
pub mod tmass_psc_xsc_join;
#[cfg(any(feature = "gaiaedr3_tmass_psc_xsc_neighbourhood", test))]
pub mod tmass_psc_xsc_neighbourhood;
#[cfg(any(feature = "gaiaedr3_tycho2tdsc_merge", test))]
pub mod tycho2tdsc_merge;
#[cfg(any(feature = "gaiaedr3_tycho2tdsc_merge_best_neighbour", test))]
pub mod tycho2tdsc_merge_best_neighbour;
#[cfg(any(feature = "gaiaedr3_tycho2tdsc_merge_neighbourhood", test))]
pub mod tycho2tdsc_merge_neighbourhood;
#[cfg(any(feature = "gaiaedr3_urat1_best_neighbour", test))]
pub mod urat1_best_neighbour;
#[cfg(any(feature = "gaiaedr3_urat1_neighbourhood", test))]
pub mod urat1_neighbourhood;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    // Some tables do not have any columns. Disabling compiler warnings for these cases
    #[allow(unused_mut)]
    let mut tables = std::collections::HashMap::new();
    gaia_source::collect_known(&mut tables);
    agn_cross_id::collect_known(&mut tables);
    commanded_scan_law::collect_known(&mut tables);
    dr2_neighbourhood::collect_known(&mut tables);
    frame_rotator_source::collect_known(&mut tables);
    allwise_best_neighbour::collect_known(&mut tables);
    allwise_neighbourhood::collect_known(&mut tables);
    apassdr9_best_neighbour::collect_known(&mut tables);
    apassdr9_join::collect_known(&mut tables);
    apassdr9_neighbourhood::collect_known(&mut tables);
    gsc23_best_neighbour::collect_known(&mut tables);
    gsc23_join::collect_known(&mut tables);
    gsc23_neighbourhood::collect_known(&mut tables);
    hipparcos2_best_neighbour::collect_known(&mut tables);
    hipparcos2_neighbourhood::collect_known(&mut tables);
    panstarrs1_best_neighbour::collect_known(&mut tables);
    panstarrs1_join::collect_known(&mut tables);
    panstarrs1_neighbourhood::collect_known(&mut tables);
    ravedr5_best_neighbour::collect_known(&mut tables);
    ravedr5_join::collect_known(&mut tables);
    ravedr5_neighbourhood::collect_known(&mut tables);
    sdssdr13_best_neighbour::collect_known(&mut tables);
    sdssdr13_join::collect_known(&mut tables);
    sdssdr13_neighbourhood::collect_known(&mut tables);
    skymapperdr2_best_neighbour::collect_known(&mut tables);
    skymapperdr2_join::collect_known(&mut tables);
    skymapperdr2_neighbourhood::collect_known(&mut tables);
    tmass_psc_xsc_best_neighbour::collect_known(&mut tables);
    tmass_psc_xsc_join::collect_known(&mut tables);
    tmass_psc_xsc_neighbourhood::collect_known(&mut tables);
    tycho2tdsc_merge_best_neighbour::collect_known(&mut tables);
    tycho2tdsc_merge_neighbourhood::collect_known(&mut tables);
    urat1_best_neighbour::collect_known(&mut tables);
    urat1_neighbourhood::collect_known(&mut tables);
    gaia_source_simulation::collect_known(&mut tables);
    gaia_universe_model::collect_known(&mut tables);
    tycho2tdsc_merge::collect_known(&mut tables);
    map.insert(gaiaedr3.string(), tables);
}
