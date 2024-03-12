// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct gaia_source;

impl Schema for gaia_source {
    fn string(&self) -> String {
        "gaia_source".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    designation,
    source_id,
    random_index,
    ref_epoch,
    ra,
    ra_error,
    dec,
    dec_error,
    parallax,
    parallax_error,
    parallax_over_error,
    pm,
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
    astrometric_n_bad_obs_al,
    astrometric_gof_al,
    astrometric_chi2_al,
    astrometric_excess_noise,
    astrometric_excess_noise_sig,
    astrometric_params_solved,
    astrometric_primary_flag,
    nu_eff_used_in_astrometry,
    pseudocolour,
    pseudocolour_error,
    ra_pseudocolour_corr,
    dec_pseudocolour_corr,
    parallax_pseudocolour_corr,
    pmra_pseudocolour_corr,
    pmdec_pseudocolour_corr,
    astrometric_matched_transits,
    visibility_periods_used,
    astrometric_sigma5d_max,
    matched_transits,
    new_matched_transits,
    matched_transits_removed,
    ipd_gof_harmonic_amplitude,
    ipd_gof_harmonic_phase,
    ipd_frac_multi_peak,
    ipd_frac_odd_win,
    ruwe,
    scan_direction_strength_k1,
    scan_direction_strength_k2,
    scan_direction_strength_k3,
    scan_direction_strength_k4,
    scan_direction_mean_k1,
    scan_direction_mean_k2,
    scan_direction_mean_k3,
    scan_direction_mean_k4,
    duplicated_source,
    phot_g_n_obs,
    phot_g_mean_flux,
    phot_g_mean_flux_error,
    phot_g_mean_flux_over_error,
    phot_g_mean_mag,
    phot_bp_n_obs,
    phot_bp_mean_flux,
    phot_bp_mean_flux_error,
    phot_bp_mean_flux_over_error,
    phot_bp_mean_mag,
    phot_rp_n_obs,
    phot_rp_mean_flux,
    phot_rp_mean_flux_error,
    phot_rp_mean_flux_over_error,
    phot_rp_mean_mag,
    phot_bp_n_contaminated_transits,
    phot_bp_n_blended_transits,
    phot_rp_n_contaminated_transits,
    phot_rp_n_blended_transits,
    phot_proc_mode,
    phot_bp_rp_excess_factor,
    bp_rp,
    bp_g,
    g_rp,
    dr2_radial_velocity,
    dr2_radial_velocity_error,
    dr2_rv_nb_transits,
    dr2_rv_template_teff,
    dr2_rv_template_logg,
    dr2_rv_template_fe_h,
    l,
    b,
    ecl_lon,
    ecl_lat,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::designation.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::random_index.to_string());
    col_strings.push(Col::ref_epoch.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::parallax.to_string());
    col_strings.push(Col::parallax_error.to_string());
    col_strings.push(Col::parallax_over_error.to_string());
    col_strings.push(Col::pm.to_string());
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
    col_strings.push(Col::astrometric_n_bad_obs_al.to_string());
    col_strings.push(Col::astrometric_gof_al.to_string());
    col_strings.push(Col::astrometric_chi2_al.to_string());
    col_strings.push(Col::astrometric_excess_noise.to_string());
    col_strings.push(Col::astrometric_excess_noise_sig.to_string());
    col_strings.push(Col::astrometric_params_solved.to_string());
    col_strings.push(Col::astrometric_primary_flag.to_string());
    col_strings.push(Col::nu_eff_used_in_astrometry.to_string());
    col_strings.push(Col::pseudocolour.to_string());
    col_strings.push(Col::pseudocolour_error.to_string());
    col_strings.push(Col::ra_pseudocolour_corr.to_string());
    col_strings.push(Col::dec_pseudocolour_corr.to_string());
    col_strings.push(Col::parallax_pseudocolour_corr.to_string());
    col_strings.push(Col::pmra_pseudocolour_corr.to_string());
    col_strings.push(Col::pmdec_pseudocolour_corr.to_string());
    col_strings.push(Col::astrometric_matched_transits.to_string());
    col_strings.push(Col::visibility_periods_used.to_string());
    col_strings.push(Col::astrometric_sigma5d_max.to_string());
    col_strings.push(Col::matched_transits.to_string());
    col_strings.push(Col::new_matched_transits.to_string());
    col_strings.push(Col::matched_transits_removed.to_string());
    col_strings.push(Col::ipd_gof_harmonic_amplitude.to_string());
    col_strings.push(Col::ipd_gof_harmonic_phase.to_string());
    col_strings.push(Col::ipd_frac_multi_peak.to_string());
    col_strings.push(Col::ipd_frac_odd_win.to_string());
    col_strings.push(Col::ruwe.to_string());
    col_strings.push(Col::scan_direction_strength_k1.to_string());
    col_strings.push(Col::scan_direction_strength_k2.to_string());
    col_strings.push(Col::scan_direction_strength_k3.to_string());
    col_strings.push(Col::scan_direction_strength_k4.to_string());
    col_strings.push(Col::scan_direction_mean_k1.to_string());
    col_strings.push(Col::scan_direction_mean_k2.to_string());
    col_strings.push(Col::scan_direction_mean_k3.to_string());
    col_strings.push(Col::scan_direction_mean_k4.to_string());
    col_strings.push(Col::duplicated_source.to_string());
    col_strings.push(Col::phot_g_n_obs.to_string());
    col_strings.push(Col::phot_g_mean_flux.to_string());
    col_strings.push(Col::phot_g_mean_flux_error.to_string());
    col_strings.push(Col::phot_g_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::phot_bp_n_obs.to_string());
    col_strings.push(Col::phot_bp_mean_flux.to_string());
    col_strings.push(Col::phot_bp_mean_flux_error.to_string());
    col_strings.push(Col::phot_bp_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_bp_mean_mag.to_string());
    col_strings.push(Col::phot_rp_n_obs.to_string());
    col_strings.push(Col::phot_rp_mean_flux.to_string());
    col_strings.push(Col::phot_rp_mean_flux_error.to_string());
    col_strings.push(Col::phot_rp_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_rp_mean_mag.to_string());
    col_strings.push(Col::phot_bp_n_contaminated_transits.to_string());
    col_strings.push(Col::phot_bp_n_blended_transits.to_string());
    col_strings.push(Col::phot_rp_n_contaminated_transits.to_string());
    col_strings.push(Col::phot_rp_n_blended_transits.to_string());
    col_strings.push(Col::phot_proc_mode.to_string());
    col_strings.push(Col::phot_bp_rp_excess_factor.to_string());
    col_strings.push(Col::bp_rp.to_string());
    col_strings.push(Col::bp_g.to_string());
    col_strings.push(Col::g_rp.to_string());
    col_strings.push(Col::dr2_radial_velocity.to_string());
    col_strings.push(Col::dr2_radial_velocity_error.to_string());
    col_strings.push(Col::dr2_rv_nb_transits.to_string());
    col_strings.push(Col::dr2_rv_template_teff.to_string());
    col_strings.push(Col::dr2_rv_template_logg.to_string());
    col_strings.push(Col::dr2_rv_template_fe_h.to_string());
    col_strings.push(Col::l.to_string());
    col_strings.push(Col::b.to_string());
    col_strings.push(Col::ecl_lon.to_string());
    col_strings.push(Col::ecl_lat.to_string());
    map.insert(gaia_source.string(), col_strings);
}
