// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct ravedr5_gra;

impl Schema for ravedr5_gra {
    fn string(&self) -> String {
        "ravedr5_gra".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    name,
    loggsc,
    e_loggsc,
    flag050,
    flag075,
    flagm,
    teffir,
    mg_h,
    o_mg_h,
    al_h,
    o_al_h,
    si_h,
    o_si_h,
    ti_h,
    o_ti_h,
    fe_h,
    o_fe_h,
    ni_h,
    o_ni_h,
    a_fe_c,
    chisqc,
    fracc,
    avschl,
    dist,
    e_dist,
    logav,
    e_logav,
    plx,
    e_plx,
    dm,
    e_dm,
    ffb,
    fqb,
    ngauss,
    gm1,
    gs1,
    gf1,
    gm2,
    gs2,
    gf2,
    gm3,
    gs3,
    gf3,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::name.to_string());
    col_strings.push(Col::loggsc.to_string());
    col_strings.push(Col::e_loggsc.to_string());
    col_strings.push(Col::flag050.to_string());
    col_strings.push(Col::flag075.to_string());
    col_strings.push(Col::flagm.to_string());
    col_strings.push(Col::teffir.to_string());
    col_strings.push(Col::mg_h.to_string());
    col_strings.push(Col::o_mg_h.to_string());
    col_strings.push(Col::al_h.to_string());
    col_strings.push(Col::o_al_h.to_string());
    col_strings.push(Col::si_h.to_string());
    col_strings.push(Col::o_si_h.to_string());
    col_strings.push(Col::ti_h.to_string());
    col_strings.push(Col::o_ti_h.to_string());
    col_strings.push(Col::fe_h.to_string());
    col_strings.push(Col::o_fe_h.to_string());
    col_strings.push(Col::ni_h.to_string());
    col_strings.push(Col::o_ni_h.to_string());
    col_strings.push(Col::a_fe_c.to_string());
    col_strings.push(Col::chisqc.to_string());
    col_strings.push(Col::fracc.to_string());
    col_strings.push(Col::avschl.to_string());
    col_strings.push(Col::dist.to_string());
    col_strings.push(Col::e_dist.to_string());
    col_strings.push(Col::logav.to_string());
    col_strings.push(Col::e_logav.to_string());
    col_strings.push(Col::plx.to_string());
    col_strings.push(Col::e_plx.to_string());
    col_strings.push(Col::dm.to_string());
    col_strings.push(Col::e_dm.to_string());
    col_strings.push(Col::ffb.to_string());
    col_strings.push(Col::fqb.to_string());
    col_strings.push(Col::ngauss.to_string());
    col_strings.push(Col::gm1.to_string());
    col_strings.push(Col::gs1.to_string());
    col_strings.push(Col::gf1.to_string());
    col_strings.push(Col::gm2.to_string());
    col_strings.push(Col::gs2.to_string());
    col_strings.push(Col::gf2.to_string());
    col_strings.push(Col::gm3.to_string());
    col_strings.push(Col::gs3.to_string());
    col_strings.push(Col::gf3.to_string());
    map.insert(ravedr5_gra.string(), col_strings);
}
