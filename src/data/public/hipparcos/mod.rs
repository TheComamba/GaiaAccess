// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct hipparcos;

impl Schema for hipparcos {
    fn string(&self) -> String {
        "hipparcos".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    hip,
    proxy,
    rahms,
    dedms,
    vmag,
    varflag,
    r_vmag,
    ra,
    de,
    astroref,
    plx,
    pmra,
    pmde,
    e_radeg,
    e_dedeg,
    e_plx,
    e_pmra,
    e_pmde,
    dera,
    plxra,
    plxde,
    pmrara,
    pmrade,
    pmraplx,
    pmdera,
    pmdede,
    pmdeplx,
    pmdepmra,
    f1,
    f2,
    btmag,
    e_btmag,
    vtmag,
    e_vtmag,
    m_btmag,
    b_v,
    e_b_v,
    r_b_v,
    v_i,
    e_v_i,
    r_v_i,
    combmag,
    hpmag,
    e_hpmag,
    hpscat,
    o_hpmag,
    m_hpmag,
    hpmax,
    hpmin,
    period,
    hvartype,
    morevar,
    morephoto,
    ccdm,
    n_ccdm,
    nsys,
    ncomp,
    multflag,
    source,
    qual,
    m_hip,
    theta,
    rho,
    e_rho,
    dhp,
    e_dhp,
    survey,
    chart,
    notes,
    hd,
    bd,
    cod,
    cpd,
    v_ired,
    sptype,
    r_sptype,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::hip.to_string());
    col_strings.push(Col::proxy.to_string());
    col_strings.push(Col::rahms.to_string());
    col_strings.push(Col::dedms.to_string());
    col_strings.push(Col::vmag.to_string());
    col_strings.push(Col::varflag.to_string());
    col_strings.push(Col::r_vmag.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::de.to_string());
    col_strings.push(Col::astroref.to_string());
    col_strings.push(Col::plx.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmde.to_string());
    col_strings.push(Col::e_radeg.to_string());
    col_strings.push(Col::e_dedeg.to_string());
    col_strings.push(Col::e_plx.to_string());
    col_strings.push(Col::e_pmra.to_string());
    col_strings.push(Col::e_pmde.to_string());
    col_strings.push(Col::dera.to_string());
    col_strings.push(Col::plxra.to_string());
    col_strings.push(Col::plxde.to_string());
    col_strings.push(Col::pmrara.to_string());
    col_strings.push(Col::pmrade.to_string());
    col_strings.push(Col::pmraplx.to_string());
    col_strings.push(Col::pmdera.to_string());
    col_strings.push(Col::pmdede.to_string());
    col_strings.push(Col::pmdeplx.to_string());
    col_strings.push(Col::pmdepmra.to_string());
    col_strings.push(Col::f1.to_string());
    col_strings.push(Col::f2.to_string());
    col_strings.push(Col::btmag.to_string());
    col_strings.push(Col::e_btmag.to_string());
    col_strings.push(Col::vtmag.to_string());
    col_strings.push(Col::e_vtmag.to_string());
    col_strings.push(Col::m_btmag.to_string());
    col_strings.push(Col::b_v.to_string());
    col_strings.push(Col::e_b_v.to_string());
    col_strings.push(Col::r_b_v.to_string());
    col_strings.push(Col::v_i.to_string());
    col_strings.push(Col::e_v_i.to_string());
    col_strings.push(Col::r_v_i.to_string());
    col_strings.push(Col::combmag.to_string());
    col_strings.push(Col::hpmag.to_string());
    col_strings.push(Col::e_hpmag.to_string());
    col_strings.push(Col::hpscat.to_string());
    col_strings.push(Col::o_hpmag.to_string());
    col_strings.push(Col::m_hpmag.to_string());
    col_strings.push(Col::hpmax.to_string());
    col_strings.push(Col::hpmin.to_string());
    col_strings.push(Col::period.to_string());
    col_strings.push(Col::hvartype.to_string());
    col_strings.push(Col::morevar.to_string());
    col_strings.push(Col::morephoto.to_string());
    col_strings.push(Col::ccdm.to_string());
    col_strings.push(Col::n_ccdm.to_string());
    col_strings.push(Col::nsys.to_string());
    col_strings.push(Col::ncomp.to_string());
    col_strings.push(Col::multflag.to_string());
    col_strings.push(Col::source.to_string());
    col_strings.push(Col::qual.to_string());
    col_strings.push(Col::m_hip.to_string());
    col_strings.push(Col::theta.to_string());
    col_strings.push(Col::rho.to_string());
    col_strings.push(Col::e_rho.to_string());
    col_strings.push(Col::dhp.to_string());
    col_strings.push(Col::e_dhp.to_string());
    col_strings.push(Col::survey.to_string());
    col_strings.push(Col::chart.to_string());
    col_strings.push(Col::notes.to_string());
    col_strings.push(Col::hd.to_string());
    col_strings.push(Col::bd.to_string());
    col_strings.push(Col::cod.to_string());
    col_strings.push(Col::cpd.to_string());
    col_strings.push(Col::v_ired.to_string());
    col_strings.push(Col::sptype.to_string());
    col_strings.push(Col::r_sptype.to_string());
    map.insert(hipparcos.string(), col_strings);
}
