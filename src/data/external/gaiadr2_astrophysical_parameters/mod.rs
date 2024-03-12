// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct gaiadr2_astrophysical_parameters;

impl Schema for gaiadr2_astrophysical_parameters {
    fn string(&self) -> String {
        "gaiadr2_astrophysical_parameters".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    a0_best,
    a0_p50,
    a0_dist,
    r0_best,
    r0_p50,
    r0_dist,
    loga_best,
    loga_p50,
    loga_dist,
    logl_best,
    logl_p50,
    logl_dist,
    logm_best,
    logm_p50,
    logm_dist,
    logt_best,
    logt_p50,
    logt_dist,
    logg_best,
    logg_p50,
    logg_dist,
    a_bp_best,
    a_bp_p50,
    a_bp_dist,
    a_g_best,
    a_g_p50,
    a_g_dist,
    a_rp_best,
    a_rp_p50,
    a_rp_dist,
    mag_bp,
    err_bp,
    bp_best,
    bp_p50,
    bp_dist,
    mag_g,
    err_g,
    g_best,
    g_p50,
    g_dist,
    mag_rp,
    err_rp,
    rp_best,
    rp_p50,
    rp_dist,
    mag_j,
    err_j,
    j_best,
    j_p50,
    j_dist,
    mag_h,
    err_h,
    h_best,
    h_p50,
    h_dist,
    mag_ks,
    err_ks,
    ks_best,
    ks_p50,
    ks_dist,
    mag_w1,
    err_w1,
    w1_best,
    w1_p50,
    w1_dist,
    mag_w2,
    err_w2,
    w2_best,
    w2_p50,
    w2_dist,
    dmod_best,
    dmod_p50,
    dmod_dist,
    lnlike_best,
    lnlike_p50,
    lnlike_dist,
    lnp_best,
    lnp_p50,
    lnp_dist,
    log10jitter_best,
    log10jitter_p50,
    log10jitter_dist,
    parallax,
    err_parallax,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::a0_best.to_string());
    col_strings.push(Col::a0_p50.to_string());
    col_strings.push(Col::a0_dist.to_string());
    col_strings.push(Col::r0_best.to_string());
    col_strings.push(Col::r0_p50.to_string());
    col_strings.push(Col::r0_dist.to_string());
    col_strings.push(Col::loga_best.to_string());
    col_strings.push(Col::loga_p50.to_string());
    col_strings.push(Col::loga_dist.to_string());
    col_strings.push(Col::logl_best.to_string());
    col_strings.push(Col::logl_p50.to_string());
    col_strings.push(Col::logl_dist.to_string());
    col_strings.push(Col::logm_best.to_string());
    col_strings.push(Col::logm_p50.to_string());
    col_strings.push(Col::logm_dist.to_string());
    col_strings.push(Col::logt_best.to_string());
    col_strings.push(Col::logt_p50.to_string());
    col_strings.push(Col::logt_dist.to_string());
    col_strings.push(Col::logg_best.to_string());
    col_strings.push(Col::logg_p50.to_string());
    col_strings.push(Col::logg_dist.to_string());
    col_strings.push(Col::a_bp_best.to_string());
    col_strings.push(Col::a_bp_p50.to_string());
    col_strings.push(Col::a_bp_dist.to_string());
    col_strings.push(Col::a_g_best.to_string());
    col_strings.push(Col::a_g_p50.to_string());
    col_strings.push(Col::a_g_dist.to_string());
    col_strings.push(Col::a_rp_best.to_string());
    col_strings.push(Col::a_rp_p50.to_string());
    col_strings.push(Col::a_rp_dist.to_string());
    col_strings.push(Col::mag_bp.to_string());
    col_strings.push(Col::err_bp.to_string());
    col_strings.push(Col::bp_best.to_string());
    col_strings.push(Col::bp_p50.to_string());
    col_strings.push(Col::bp_dist.to_string());
    col_strings.push(Col::mag_g.to_string());
    col_strings.push(Col::err_g.to_string());
    col_strings.push(Col::g_best.to_string());
    col_strings.push(Col::g_p50.to_string());
    col_strings.push(Col::g_dist.to_string());
    col_strings.push(Col::mag_rp.to_string());
    col_strings.push(Col::err_rp.to_string());
    col_strings.push(Col::rp_best.to_string());
    col_strings.push(Col::rp_p50.to_string());
    col_strings.push(Col::rp_dist.to_string());
    col_strings.push(Col::mag_j.to_string());
    col_strings.push(Col::err_j.to_string());
    col_strings.push(Col::j_best.to_string());
    col_strings.push(Col::j_p50.to_string());
    col_strings.push(Col::j_dist.to_string());
    col_strings.push(Col::mag_h.to_string());
    col_strings.push(Col::err_h.to_string());
    col_strings.push(Col::h_best.to_string());
    col_strings.push(Col::h_p50.to_string());
    col_strings.push(Col::h_dist.to_string());
    col_strings.push(Col::mag_ks.to_string());
    col_strings.push(Col::err_ks.to_string());
    col_strings.push(Col::ks_best.to_string());
    col_strings.push(Col::ks_p50.to_string());
    col_strings.push(Col::ks_dist.to_string());
    col_strings.push(Col::mag_w1.to_string());
    col_strings.push(Col::err_w1.to_string());
    col_strings.push(Col::w1_best.to_string());
    col_strings.push(Col::w1_p50.to_string());
    col_strings.push(Col::w1_dist.to_string());
    col_strings.push(Col::mag_w2.to_string());
    col_strings.push(Col::err_w2.to_string());
    col_strings.push(Col::w2_best.to_string());
    col_strings.push(Col::w2_p50.to_string());
    col_strings.push(Col::w2_dist.to_string());
    col_strings.push(Col::dmod_best.to_string());
    col_strings.push(Col::dmod_p50.to_string());
    col_strings.push(Col::dmod_dist.to_string());
    col_strings.push(Col::lnlike_best.to_string());
    col_strings.push(Col::lnlike_p50.to_string());
    col_strings.push(Col::lnlike_dist.to_string());
    col_strings.push(Col::lnp_best.to_string());
    col_strings.push(Col::lnp_p50.to_string());
    col_strings.push(Col::lnp_dist.to_string());
    col_strings.push(Col::log10jitter_best.to_string());
    col_strings.push(Col::log10jitter_p50.to_string());
    col_strings.push(Col::log10jitter_dist.to_string());
    col_strings.push(Col::parallax.to_string());
    col_strings.push(Col::err_parallax.to_string());
    map.insert(gaiadr2_astrophysical_parameters.string(), col_strings);
}
