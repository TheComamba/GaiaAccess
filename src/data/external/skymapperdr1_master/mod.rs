// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct skymapperdr1_master;

impl Schema for skymapperdr1_master {
    fn string(&self) -> String {
        "skymapperdr1_master".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    object_id,
    raj2000,
    dej2000,
    e_raj2000,
    e_dej2000,
    smss_j,
    mean_epoch,
    rms_epoch,
    glon,
    glat,
    flags,
    nimaflags,
    ngood,
    ngood_min,
    nch_max,
    u_flags,
    u_nimaflags,
    u_ngood,
    u_nch,
    u_nvisit,
    v_flags,
    v_nimaflags,
    v_ngood,
    v_nch,
    v_nvisit,
    g_flags,
    g_nimaflags,
    g_ngood,
    g_nch,
    g_nvisit,
    r_flags,
    r_nimaflags,
    r_ngood,
    r_nch,
    r_nvisit,
    i_flags,
    i_nimaflags,
    i_ngood,
    i_nch,
    i_nvisit,
    z_flags,
    z_nimaflags,
    z_ngood,
    z_nch,
    z_nvisit,
    class_star,
    radius_petro,
    a,
    e_a,
    b,
    e_b,
    pa,
    e_pa,
    u_psf,
    e_u_psf,
    u_petro,
    e_u_petro,
    v_psf,
    e_v_psf,
    v_petro,
    e_v_petro,
    g_psf,
    e_g_psf,
    g_petro,
    e_g_petro,
    r_psf,
    e_r_psf,
    r_petro,
    e_r_petro,
    i_psf,
    e_i_psf,
    i_petro,
    e_i_petro,
    z_psf,
    e_z_psf,
    z_petro,
    e_z_petro,
    ebmv_sfd,
    prox,
    prox_id,
    edr_id,
    edr_dist,
    twomass_key1,
    twomass_dist1,
    twomass_cat1,
    twomass_key2,
    twomass_dist2,
    twomass_cat2,
    allwise_cntr,
    allwise_dist,
    ucac4_mpos,
    ucac4_dist,
    apass_recno,
    apass_dist,
    gaia_dr1_id,
    gaia_dr1_dist,
    ps1_dr1_id,
    ps1_dr1_dist,
    galex_bcs_id,
    galex_bcs_dist,
    gaia_dr2_id1,
    gaia_dr2_dist1,
    gaia_dr2_id2,
    gaia_dr2_dist2,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::object_id.to_string());
    col_strings.push(Col::raj2000.to_string());
    col_strings.push(Col::dej2000.to_string());
    col_strings.push(Col::e_raj2000.to_string());
    col_strings.push(Col::e_dej2000.to_string());
    col_strings.push(Col::smss_j.to_string());
    col_strings.push(Col::mean_epoch.to_string());
    col_strings.push(Col::rms_epoch.to_string());
    col_strings.push(Col::glon.to_string());
    col_strings.push(Col::glat.to_string());
    col_strings.push(Col::flags.to_string());
    col_strings.push(Col::nimaflags.to_string());
    col_strings.push(Col::ngood.to_string());
    col_strings.push(Col::ngood_min.to_string());
    col_strings.push(Col::nch_max.to_string());
    col_strings.push(Col::u_flags.to_string());
    col_strings.push(Col::u_nimaflags.to_string());
    col_strings.push(Col::u_ngood.to_string());
    col_strings.push(Col::u_nch.to_string());
    col_strings.push(Col::u_nvisit.to_string());
    col_strings.push(Col::v_flags.to_string());
    col_strings.push(Col::v_nimaflags.to_string());
    col_strings.push(Col::v_ngood.to_string());
    col_strings.push(Col::v_nch.to_string());
    col_strings.push(Col::v_nvisit.to_string());
    col_strings.push(Col::g_flags.to_string());
    col_strings.push(Col::g_nimaflags.to_string());
    col_strings.push(Col::g_ngood.to_string());
    col_strings.push(Col::g_nch.to_string());
    col_strings.push(Col::g_nvisit.to_string());
    col_strings.push(Col::r_flags.to_string());
    col_strings.push(Col::r_nimaflags.to_string());
    col_strings.push(Col::r_ngood.to_string());
    col_strings.push(Col::r_nch.to_string());
    col_strings.push(Col::r_nvisit.to_string());
    col_strings.push(Col::i_flags.to_string());
    col_strings.push(Col::i_nimaflags.to_string());
    col_strings.push(Col::i_ngood.to_string());
    col_strings.push(Col::i_nch.to_string());
    col_strings.push(Col::i_nvisit.to_string());
    col_strings.push(Col::z_flags.to_string());
    col_strings.push(Col::z_nimaflags.to_string());
    col_strings.push(Col::z_ngood.to_string());
    col_strings.push(Col::z_nch.to_string());
    col_strings.push(Col::z_nvisit.to_string());
    col_strings.push(Col::class_star.to_string());
    col_strings.push(Col::radius_petro.to_string());
    col_strings.push(Col::a.to_string());
    col_strings.push(Col::e_a.to_string());
    col_strings.push(Col::b.to_string());
    col_strings.push(Col::e_b.to_string());
    col_strings.push(Col::pa.to_string());
    col_strings.push(Col::e_pa.to_string());
    col_strings.push(Col::u_psf.to_string());
    col_strings.push(Col::e_u_psf.to_string());
    col_strings.push(Col::u_petro.to_string());
    col_strings.push(Col::e_u_petro.to_string());
    col_strings.push(Col::v_psf.to_string());
    col_strings.push(Col::e_v_psf.to_string());
    col_strings.push(Col::v_petro.to_string());
    col_strings.push(Col::e_v_petro.to_string());
    col_strings.push(Col::g_psf.to_string());
    col_strings.push(Col::e_g_psf.to_string());
    col_strings.push(Col::g_petro.to_string());
    col_strings.push(Col::e_g_petro.to_string());
    col_strings.push(Col::r_psf.to_string());
    col_strings.push(Col::e_r_psf.to_string());
    col_strings.push(Col::r_petro.to_string());
    col_strings.push(Col::e_r_petro.to_string());
    col_strings.push(Col::i_psf.to_string());
    col_strings.push(Col::e_i_psf.to_string());
    col_strings.push(Col::i_petro.to_string());
    col_strings.push(Col::e_i_petro.to_string());
    col_strings.push(Col::z_psf.to_string());
    col_strings.push(Col::e_z_psf.to_string());
    col_strings.push(Col::z_petro.to_string());
    col_strings.push(Col::e_z_petro.to_string());
    col_strings.push(Col::ebmv_sfd.to_string());
    col_strings.push(Col::prox.to_string());
    col_strings.push(Col::prox_id.to_string());
    col_strings.push(Col::edr_id.to_string());
    col_strings.push(Col::edr_dist.to_string());
    col_strings.push(Col::twomass_key1.to_string());
    col_strings.push(Col::twomass_dist1.to_string());
    col_strings.push(Col::twomass_cat1.to_string());
    col_strings.push(Col::twomass_key2.to_string());
    col_strings.push(Col::twomass_dist2.to_string());
    col_strings.push(Col::twomass_cat2.to_string());
    col_strings.push(Col::allwise_cntr.to_string());
    col_strings.push(Col::allwise_dist.to_string());
    col_strings.push(Col::ucac4_mpos.to_string());
    col_strings.push(Col::ucac4_dist.to_string());
    col_strings.push(Col::apass_recno.to_string());
    col_strings.push(Col::apass_dist.to_string());
    col_strings.push(Col::gaia_dr1_id.to_string());
    col_strings.push(Col::gaia_dr1_dist.to_string());
    col_strings.push(Col::ps1_dr1_id.to_string());
    col_strings.push(Col::ps1_dr1_dist.to_string());
    col_strings.push(Col::galex_bcs_id.to_string());
    col_strings.push(Col::galex_bcs_dist.to_string());
    col_strings.push(Col::gaia_dr2_id1.to_string());
    col_strings.push(Col::gaia_dr2_dist1.to_string());
    col_strings.push(Col::gaia_dr2_id2.to_string());
    col_strings.push(Col::gaia_dr2_dist2.to_string());
    map.insert(skymapperdr1_master.string(), col_strings);
}
