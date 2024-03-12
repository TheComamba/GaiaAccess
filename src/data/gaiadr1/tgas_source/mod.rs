// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct tgas_source;

impl Schema for tgas_source {
    fn string(&self) -> String {
        "tgas_source".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    hip,
    tycho2_id,
    solution_id,
    source_id,
    random_index,
    ref_epoch,
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
    ra_dec_corr,
    ra_parallax_corr,
    ra_pmra_corr,
    ra_pmdec_corr,
    dec_parallax_corr,
    dec_pmra_corr,
    dec_pmdec_corr,
    parallax_pmra_corr,
    parallax_pmdec_corr,
    pmra_pmdec_corr,
    astrometric_n_obs_al,
    astrometric_n_obs_ac,
    astrometric_n_good_obs_al,
    astrometric_n_good_obs_ac,
    astrometric_n_bad_obs_al,
    astrometric_n_bad_obs_ac,
    astrometric_delta_q,
    astrometric_excess_noise,
    astrometric_excess_noise_sig,
    astrometric_primary_flag,
    astrometric_relegation_factor,
    astrometric_weight_al,
    astrometric_weight_ac,
    astrometric_priors_used,
    matched_observations,
    duplicated_source,
    scan_direction_strength_k1,
    scan_direction_strength_k2,
    scan_direction_strength_k3,
    scan_direction_strength_k4,
    scan_direction_mean_k1,
    scan_direction_mean_k2,
    scan_direction_mean_k3,
    scan_direction_mean_k4,
    phot_g_n_obs,
    phot_g_mean_flux,
    phot_g_mean_flux_error,
    phot_g_mean_mag,
    phot_variable_flag,
    l,
    b,
    ecl_lon,
    ecl_lat,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::hip.to_string());
    col_strings.push(Col::tycho2_id.to_string());
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::random_index.to_string());
    col_strings.push(Col::ref_epoch.to_string());
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
    col_strings.push(Col::ra_dec_corr.to_string());
    col_strings.push(Col::ra_parallax_corr.to_string());
    col_strings.push(Col::ra_pmra_corr.to_string());
    col_strings.push(Col::ra_pmdec_corr.to_string());
    col_strings.push(Col::dec_parallax_corr.to_string());
    col_strings.push(Col::dec_pmra_corr.to_string());
    col_strings.push(Col::dec_pmdec_corr.to_string());
    col_strings.push(Col::parallax_pmra_corr.to_string());
    col_strings.push(Col::parallax_pmdec_corr.to_string());
    col_strings.push(Col::pmra_pmdec_corr.to_string());
    col_strings.push(Col::astrometric_n_obs_al.to_string());
    col_strings.push(Col::astrometric_n_obs_ac.to_string());
    col_strings.push(Col::astrometric_n_good_obs_al.to_string());
    col_strings.push(Col::astrometric_n_good_obs_ac.to_string());
    col_strings.push(Col::astrometric_n_bad_obs_al.to_string());
    col_strings.push(Col::astrometric_n_bad_obs_ac.to_string());
    col_strings.push(Col::astrometric_delta_q.to_string());
    col_strings.push(Col::astrometric_excess_noise.to_string());
    col_strings.push(Col::astrometric_excess_noise_sig.to_string());
    col_strings.push(Col::astrometric_primary_flag.to_string());
    col_strings.push(Col::astrometric_relegation_factor.to_string());
    col_strings.push(Col::astrometric_weight_al.to_string());
    col_strings.push(Col::astrometric_weight_ac.to_string());
    col_strings.push(Col::astrometric_priors_used.to_string());
    col_strings.push(Col::matched_observations.to_string());
    col_strings.push(Col::duplicated_source.to_string());
    col_strings.push(Col::scan_direction_strength_k1.to_string());
    col_strings.push(Col::scan_direction_strength_k2.to_string());
    col_strings.push(Col::scan_direction_strength_k3.to_string());
    col_strings.push(Col::scan_direction_strength_k4.to_string());
    col_strings.push(Col::scan_direction_mean_k1.to_string());
    col_strings.push(Col::scan_direction_mean_k2.to_string());
    col_strings.push(Col::scan_direction_mean_k3.to_string());
    col_strings.push(Col::scan_direction_mean_k4.to_string());
    col_strings.push(Col::phot_g_n_obs.to_string());
    col_strings.push(Col::phot_g_mean_flux.to_string());
    col_strings.push(Col::phot_g_mean_flux_error.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::phot_variable_flag.to_string());
    col_strings.push(Col::l.to_string());
    col_strings.push(Col::b.to_string());
    col_strings.push(Col::ecl_lon.to_string());
    col_strings.push(Col::ecl_lat.to_string());
    map.insert(tgas_source.string(), col_strings);
}
