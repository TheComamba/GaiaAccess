
// This code is generated by generate_code.py, do not modify it manually
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct gaiadr3;

impl Schema for gaiadr3 {
    fn string(&self) -> String {
        "gaiadr3".to_string()
    }
}

#[cfg(any(gaia_source, test))] pub mod gaia_source;
#[cfg(any(gaia_source_lite, test))] pub mod gaia_source_lite;
#[cfg(any(astrophysical_parameters, test))] pub mod astrophysical_parameters;
#[cfg(any(astrophysical_parameters_supp, test))] pub mod astrophysical_parameters_supp;
#[cfg(any(oa_neuron_information, test))] pub mod oa_neuron_information;
#[cfg(any(oa_neuron_xp_spectra, test))] pub mod oa_neuron_xp_spectra;
#[cfg(any(total_galactic_extinction_map, test))] pub mod total_galactic_extinction_map;
#[cfg(any(total_galactic_extinction_map_opt, test))] pub mod total_galactic_extinction_map_opt;
#[cfg(any(commanded_scan_law, test))] pub mod commanded_scan_law;
#[cfg(any(allwise_best_neighbour, test))] pub mod allwise_best_neighbour;
#[cfg(any(allwise_neighbourhood, test))] pub mod allwise_neighbourhood;
#[cfg(any(apassdr9_best_neighbour, test))] pub mod apassdr9_best_neighbour;
#[cfg(any(apassdr9_join, test))] pub mod apassdr9_join;
#[cfg(any(apassdr9_neighbourhood, test))] pub mod apassdr9_neighbourhood;
#[cfg(any(dr2_neighbourhood, test))] pub mod dr2_neighbourhood;
#[cfg(any(gsc23_best_neighbour, test))] pub mod gsc23_best_neighbour;
#[cfg(any(gsc23_join, test))] pub mod gsc23_join;
#[cfg(any(gsc23_neighbourhood, test))] pub mod gsc23_neighbourhood;
#[cfg(any(hipparcos2_best_neighbour, test))] pub mod hipparcos2_best_neighbour;
#[cfg(any(hipparcos2_neighbourhood, test))] pub mod hipparcos2_neighbourhood;
#[cfg(any(panstarrs1_best_neighbour, test))] pub mod panstarrs1_best_neighbour;
#[cfg(any(panstarrs1_join, test))] pub mod panstarrs1_join;
#[cfg(any(panstarrs1_neighbourhood, test))] pub mod panstarrs1_neighbourhood;
#[cfg(any(ravedr5_best_neighbour, test))] pub mod ravedr5_best_neighbour;
#[cfg(any(ravedr5_join, test))] pub mod ravedr5_join;
#[cfg(any(ravedr5_neighbourhood, test))] pub mod ravedr5_neighbourhood;
#[cfg(any(ravedr6_best_neighbour, test))] pub mod ravedr6_best_neighbour;
#[cfg(any(ravedr6_join, test))] pub mod ravedr6_join;
#[cfg(any(ravedr6_neighbourhood, test))] pub mod ravedr6_neighbourhood;
#[cfg(any(sdssdr13_best_neighbour, test))] pub mod sdssdr13_best_neighbour;
#[cfg(any(sdssdr13_join, test))] pub mod sdssdr13_join;
#[cfg(any(sdssdr13_neighbourhood, test))] pub mod sdssdr13_neighbourhood;
#[cfg(any(skymapperdr2_best_neighbour, test))] pub mod skymapperdr2_best_neighbour;
#[cfg(any(skymapperdr2_join, test))] pub mod skymapperdr2_join;
#[cfg(any(skymapperdr2_neighbourhood, test))] pub mod skymapperdr2_neighbourhood;
#[cfg(any(tmass_psc_xsc_best_neighbour, test))] pub mod tmass_psc_xsc_best_neighbour;
#[cfg(any(tmass_psc_xsc_join, test))] pub mod tmass_psc_xsc_join;
#[cfg(any(tmass_psc_xsc_neighbourhood, test))] pub mod tmass_psc_xsc_neighbourhood;
#[cfg(any(tycho2tdsc_merge_best_neighbour, test))] pub mod tycho2tdsc_merge_best_neighbour;
#[cfg(any(tycho2tdsc_merge_neighbourhood, test))] pub mod tycho2tdsc_merge_neighbourhood;
#[cfg(any(urat1_best_neighbour, test))] pub mod urat1_best_neighbour;
#[cfg(any(urat1_neighbourhood, test))] pub mod urat1_neighbourhood;
#[cfg(any(galaxy_candidates, test))] pub mod galaxy_candidates;
#[cfg(any(galaxy_catalogue_name, test))] pub mod galaxy_catalogue_name;
#[cfg(any(qso_candidates, test))] pub mod qso_candidates;
#[cfg(any(qso_catalogue_name, test))] pub mod qso_catalogue_name;
#[cfg(any(nss_acceleration_astro, test))] pub mod nss_acceleration_astro;
#[cfg(any(nss_non_linear_spectro, test))] pub mod nss_non_linear_spectro;
#[cfg(any(nss_two_body_orbit, test))] pub mod nss_two_body_orbit;
#[cfg(any(nss_vim_fl, test))] pub mod nss_vim_fl;
#[cfg(any(binary_masses, test))] pub mod binary_masses;
#[cfg(any(chemical_cartography, test))] pub mod chemical_cartography;
#[cfg(any(gold_sample_carbon_stars, test))] pub mod gold_sample_carbon_stars;
#[cfg(any(gold_sample_fgkm_stars, test))] pub mod gold_sample_fgkm_stars;
#[cfg(any(gold_sample_oba_stars, test))] pub mod gold_sample_oba_stars;
#[cfg(any(gold_sample_solar_analogues, test))] pub mod gold_sample_solar_analogues;
#[cfg(any(gold_sample_spss, test))] pub mod gold_sample_spss;
#[cfg(any(gold_sample_ucd, test))] pub mod gold_sample_ucd;
#[cfg(any(sso_orbits, test))] pub mod sso_orbits;
#[cfg(any(synthetic_photometry_gspc, test))] pub mod synthetic_photometry_gspc;
#[cfg(any(vari_spurious_signals, test))] pub mod vari_spurious_signals;
#[cfg(any(agn_cross_id, test))] pub mod agn_cross_id;
#[cfg(any(frame_rotator_source, test))] pub mod frame_rotator_source;
#[cfg(any(gaia_crf3_xm, test))] pub mod gaia_crf3_xm;
#[cfg(any(alerts_mixedin_sourceids, test))] pub mod alerts_mixedin_sourceids;
#[cfg(any(science_alerts, test))] pub mod science_alerts;
#[cfg(any(gaia_source_simulation, test))] pub mod gaia_source_simulation;
#[cfg(any(gaia_universe_model, test))] pub mod gaia_universe_model;
#[cfg(any(sso_observation, test))] pub mod sso_observation;
#[cfg(any(sso_reflectance_spectrum, test))] pub mod sso_reflectance_spectrum;
#[cfg(any(sso_source, test))] pub mod sso_source;
#[cfg(any(xp_summary, test))] pub mod xp_summary;
#[cfg(any(vari_agn, test))] pub mod vari_agn;
#[cfg(any(vari_cepheid, test))] pub mod vari_cepheid;
#[cfg(any(vari_classifier_class_definition, test))] pub mod vari_classifier_class_definition;
#[cfg(any(vari_classifier_definition, test))] pub mod vari_classifier_definition;
#[cfg(any(vari_classifier_result, test))] pub mod vari_classifier_result;
#[cfg(any(vari_compact_companion, test))] pub mod vari_compact_companion;
#[cfg(any(vari_eclipsing_binary, test))] pub mod vari_eclipsing_binary;
#[cfg(any(vari_epoch_radial_velocity, test))] pub mod vari_epoch_radial_velocity;
#[cfg(any(vari_long_period_variable, test))] pub mod vari_long_period_variable;
#[cfg(any(vari_microlensing, test))] pub mod vari_microlensing;
#[cfg(any(vari_ms_oscillator, test))] pub mod vari_ms_oscillator;
#[cfg(any(vari_planetary_transit, test))] pub mod vari_planetary_transit;
#[cfg(any(vari_planetary_transit_13june2022, test))] pub mod vari_planetary_transit_13june2022;
#[cfg(any(vari_rad_vel_statistics, test))] pub mod vari_rad_vel_statistics;
#[cfg(any(vari_rotation_modulation, test))] pub mod vari_rotation_modulation;
#[cfg(any(vari_rrlyrae, test))] pub mod vari_rrlyrae;
#[cfg(any(vari_short_timescale, test))] pub mod vari_short_timescale;
#[cfg(any(vari_summary, test))] pub mod vari_summary;
#[cfg(any(tycho2tdsc_merge, test))] pub mod tycho2tdsc_merge;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    gaia_source::collect_known(&mut tables);
gaia_source_lite::collect_known(&mut tables);
astrophysical_parameters::collect_known(&mut tables);
astrophysical_parameters_supp::collect_known(&mut tables);
oa_neuron_information::collect_known(&mut tables);
oa_neuron_xp_spectra::collect_known(&mut tables);
total_galactic_extinction_map::collect_known(&mut tables);
total_galactic_extinction_map_opt::collect_known(&mut tables);
commanded_scan_law::collect_known(&mut tables);
allwise_best_neighbour::collect_known(&mut tables);
allwise_neighbourhood::collect_known(&mut tables);
apassdr9_best_neighbour::collect_known(&mut tables);
apassdr9_join::collect_known(&mut tables);
apassdr9_neighbourhood::collect_known(&mut tables);
dr2_neighbourhood::collect_known(&mut tables);
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
ravedr6_best_neighbour::collect_known(&mut tables);
ravedr6_join::collect_known(&mut tables);
ravedr6_neighbourhood::collect_known(&mut tables);
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
galaxy_candidates::collect_known(&mut tables);
galaxy_catalogue_name::collect_known(&mut tables);
qso_candidates::collect_known(&mut tables);
qso_catalogue_name::collect_known(&mut tables);
nss_acceleration_astro::collect_known(&mut tables);
nss_non_linear_spectro::collect_known(&mut tables);
nss_two_body_orbit::collect_known(&mut tables);
nss_vim_fl::collect_known(&mut tables);
binary_masses::collect_known(&mut tables);
chemical_cartography::collect_known(&mut tables);
gold_sample_carbon_stars::collect_known(&mut tables);
gold_sample_fgkm_stars::collect_known(&mut tables);
gold_sample_oba_stars::collect_known(&mut tables);
gold_sample_solar_analogues::collect_known(&mut tables);
gold_sample_spss::collect_known(&mut tables);
gold_sample_ucd::collect_known(&mut tables);
sso_orbits::collect_known(&mut tables);
synthetic_photometry_gspc::collect_known(&mut tables);
vari_spurious_signals::collect_known(&mut tables);
agn_cross_id::collect_known(&mut tables);
frame_rotator_source::collect_known(&mut tables);
gaia_crf3_xm::collect_known(&mut tables);
alerts_mixedin_sourceids::collect_known(&mut tables);
science_alerts::collect_known(&mut tables);
gaia_source_simulation::collect_known(&mut tables);
gaia_universe_model::collect_known(&mut tables);
sso_observation::collect_known(&mut tables);
sso_reflectance_spectrum::collect_known(&mut tables);
sso_source::collect_known(&mut tables);
xp_summary::collect_known(&mut tables);
vari_agn::collect_known(&mut tables);
vari_cepheid::collect_known(&mut tables);
vari_classifier_class_definition::collect_known(&mut tables);
vari_classifier_definition::collect_known(&mut tables);
vari_classifier_result::collect_known(&mut tables);
vari_compact_companion::collect_known(&mut tables);
vari_eclipsing_binary::collect_known(&mut tables);
vari_epoch_radial_velocity::collect_known(&mut tables);
vari_long_period_variable::collect_known(&mut tables);
vari_microlensing::collect_known(&mut tables);
vari_ms_oscillator::collect_known(&mut tables);
vari_planetary_transit::collect_known(&mut tables);
vari_planetary_transit_13june2022::collect_known(&mut tables);
vari_rad_vel_statistics::collect_known(&mut tables);
vari_rotation_modulation::collect_known(&mut tables);
vari_rrlyrae::collect_known(&mut tables);
vari_short_timescale::collect_known(&mut tables);
vari_summary::collect_known(&mut tables);
tycho2tdsc_merge::collect_known(&mut tables);
    map.insert(gaiadr3.string(), tables);
}
