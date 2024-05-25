// This code is generated by generate_code.py, do not modify it manually
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct gaiaedr3_gcns_main_1;

impl Table for gaiaedr3_gcns_main_1 {
    fn string(&self) -> String {
        "gaiaedr3_gcns_main_1".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    ra,
    dec,
    ra_error,
    dec_error,
    parallax,
    parallax_error,
    pmra,
    pmra_error,
    pmdec,
    pmdec_error,
    phot_g_mean_mag,
    phot_g_mean_flux_over_error,
    phot_bp_mean_mag,
    phot_bp_mean_flux_over_error,
    phot_rp_mean_mag,
    phot_rp_mean_flux_over_error,
    phot_bp_rp_excess_factor,
    ruwe,
    ipd_frac_multi_peak,
    adoptedrv,
    adoptedrv_error,
    adoptedrv_refname,
    radial_velocity_is_valid,
    gcns_prob,
    wd_prob,
    dist_1,
    dist_16,
    dist_50,
    dist_84,
    xcoord_50,
    xcoord_16,
    xcoord_84,
    ycoord_50,
    ycoord_16,
    ycoord_84,
    zcoord_50,
    zcoord_16,
    zcoord_84,
    uvel_50,
    uvel_16,
    uvel_84,
    vvel_50,
    vvel_16,
    vvel_84,
    wvel_50,
    wvel_16,
    wvel_84,
    name_gunn,
    refname_gunn,
    gmag_gunn,
    e_gmag_gunn,
    rmag_gunn,
    e_rmag_gunn,
    imag_gunn,
    e_imag_gunn,
    zmag_gunn,
    e_zmag_gunn,
    name_2mass,
    j_m_2mass,
    j_msig_2mass,
    h_m_2mass,
    h_msig_2mass,
    k_m_2mass,
    k_msig_2mass,
    name_wise,
    w1mpro_pm_wise,
    w1sigmpro_pm_wise,
    w2mpro_pm_wise,
    w2sigmpro_pm_wise,
    w3mpro_wise,
    w3sigmpro_wise,
    w4mpro_wise,
    w4sigmpro_wise,
    gncs_main_oid,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::parallax.to_string());
    col_strings.push(Col::parallax_error.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmra_error.to_string());
    col_strings.push(Col::pmdec.to_string());
    col_strings.push(Col::pmdec_error.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::phot_g_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_bp_mean_mag.to_string());
    col_strings.push(Col::phot_bp_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_rp_mean_mag.to_string());
    col_strings.push(Col::phot_rp_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_bp_rp_excess_factor.to_string());
    col_strings.push(Col::ruwe.to_string());
    col_strings.push(Col::ipd_frac_multi_peak.to_string());
    col_strings.push(Col::adoptedrv.to_string());
    col_strings.push(Col::adoptedrv_error.to_string());
    col_strings.push(Col::adoptedrv_refname.to_string());
    col_strings.push(Col::radial_velocity_is_valid.to_string());
    col_strings.push(Col::gcns_prob.to_string());
    col_strings.push(Col::wd_prob.to_string());
    col_strings.push(Col::dist_1.to_string());
    col_strings.push(Col::dist_16.to_string());
    col_strings.push(Col::dist_50.to_string());
    col_strings.push(Col::dist_84.to_string());
    col_strings.push(Col::xcoord_50.to_string());
    col_strings.push(Col::xcoord_16.to_string());
    col_strings.push(Col::xcoord_84.to_string());
    col_strings.push(Col::ycoord_50.to_string());
    col_strings.push(Col::ycoord_16.to_string());
    col_strings.push(Col::ycoord_84.to_string());
    col_strings.push(Col::zcoord_50.to_string());
    col_strings.push(Col::zcoord_16.to_string());
    col_strings.push(Col::zcoord_84.to_string());
    col_strings.push(Col::uvel_50.to_string());
    col_strings.push(Col::uvel_16.to_string());
    col_strings.push(Col::uvel_84.to_string());
    col_strings.push(Col::vvel_50.to_string());
    col_strings.push(Col::vvel_16.to_string());
    col_strings.push(Col::vvel_84.to_string());
    col_strings.push(Col::wvel_50.to_string());
    col_strings.push(Col::wvel_16.to_string());
    col_strings.push(Col::wvel_84.to_string());
    col_strings.push(Col::name_gunn.to_string());
    col_strings.push(Col::refname_gunn.to_string());
    col_strings.push(Col::gmag_gunn.to_string());
    col_strings.push(Col::e_gmag_gunn.to_string());
    col_strings.push(Col::rmag_gunn.to_string());
    col_strings.push(Col::e_rmag_gunn.to_string());
    col_strings.push(Col::imag_gunn.to_string());
    col_strings.push(Col::e_imag_gunn.to_string());
    col_strings.push(Col::zmag_gunn.to_string());
    col_strings.push(Col::e_zmag_gunn.to_string());
    col_strings.push(Col::name_2mass.to_string());
    col_strings.push(Col::j_m_2mass.to_string());
    col_strings.push(Col::j_msig_2mass.to_string());
    col_strings.push(Col::h_m_2mass.to_string());
    col_strings.push(Col::h_msig_2mass.to_string());
    col_strings.push(Col::k_m_2mass.to_string());
    col_strings.push(Col::k_msig_2mass.to_string());
    col_strings.push(Col::name_wise.to_string());
    col_strings.push(Col::w1mpro_pm_wise.to_string());
    col_strings.push(Col::w1sigmpro_pm_wise.to_string());
    col_strings.push(Col::w2mpro_pm_wise.to_string());
    col_strings.push(Col::w2sigmpro_pm_wise.to_string());
    col_strings.push(Col::w3mpro_wise.to_string());
    col_strings.push(Col::w3sigmpro_wise.to_string());
    col_strings.push(Col::w4mpro_wise.to_string());
    col_strings.push(Col::w4sigmpro_wise.to_string());
    col_strings.push(Col::gncs_main_oid.to_string());
    map.insert(gaiaedr3_gcns_main_1.string(), col_strings);
}
