// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct galex_ais;

impl Table for galex_ais {
    fn string(&self) -> String {
        "galex_ais".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    raj2000,
    dej2000,
    name,
    objid,
    phid,
    cat,
    rafdeg,
    defdeg,
    fuvexp,
    nuvexp,
    glon,
    glat,
    tile,
    img,
    sv,
    r_fov,
    obs,
    b,
    e_b_v,
    sp_,
    chkf,
    fuvmag,
    e_fuvmag,
    nuvmag,
    e_nuvmag,
    fuv_a,
    e_fuv_a,
    nuv_a,
    e_nuv_a,
    fuv_4,
    e_fuv_4,
    nuv_4,
    e_nuv_4,
    fuv_6,
    e_fuv_6,
    nuv_6,
    e_nuv_6,
    fafl,
    nafl,
    fexf,
    nexf,
    fflux,
    e_fflux,
    nflux,
    e_nflux,
    fxpos,
    fypos,
    nxpos,
    nypos,
    fima,
    nima,
    fr,
    nr,
    ns_g,
    fs_g,
    nell,
    fell,
    npa,
    e_npa,
    fpa,
    e_fpa,
    fnr,
    f3r,
    nar,
    narms,
    nbrms,
    far,
    farms,
    fbrms,
    nuvw,
    fuvw,
    prob,
    sep,
    nerr,
    ferr,
    ierr,
    nperr,
    fperr,
    cv,
    g,
    n,
    primid,
    groupid,
    gd,
    nd,
    primidd,
    groupidd,
    grouptot,
    oname,
    #[strum(serialize = "\"size\"")]
    size,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::raj2000.to_string());
    col_strings.push(Col::dej2000.to_string());
    col_strings.push(Col::name.to_string());
    col_strings.push(Col::objid.to_string());
    col_strings.push(Col::phid.to_string());
    col_strings.push(Col::cat.to_string());
    col_strings.push(Col::rafdeg.to_string());
    col_strings.push(Col::defdeg.to_string());
    col_strings.push(Col::fuvexp.to_string());
    col_strings.push(Col::nuvexp.to_string());
    col_strings.push(Col::glon.to_string());
    col_strings.push(Col::glat.to_string());
    col_strings.push(Col::tile.to_string());
    col_strings.push(Col::img.to_string());
    col_strings.push(Col::sv.to_string());
    col_strings.push(Col::r_fov.to_string());
    col_strings.push(Col::obs.to_string());
    col_strings.push(Col::b.to_string());
    col_strings.push(Col::e_b_v.to_string());
    col_strings.push(Col::sp_.to_string());
    col_strings.push(Col::chkf.to_string());
    col_strings.push(Col::fuvmag.to_string());
    col_strings.push(Col::e_fuvmag.to_string());
    col_strings.push(Col::nuvmag.to_string());
    col_strings.push(Col::e_nuvmag.to_string());
    col_strings.push(Col::fuv_a.to_string());
    col_strings.push(Col::e_fuv_a.to_string());
    col_strings.push(Col::nuv_a.to_string());
    col_strings.push(Col::e_nuv_a.to_string());
    col_strings.push(Col::fuv_4.to_string());
    col_strings.push(Col::e_fuv_4.to_string());
    col_strings.push(Col::nuv_4.to_string());
    col_strings.push(Col::e_nuv_4.to_string());
    col_strings.push(Col::fuv_6.to_string());
    col_strings.push(Col::e_fuv_6.to_string());
    col_strings.push(Col::nuv_6.to_string());
    col_strings.push(Col::e_nuv_6.to_string());
    col_strings.push(Col::fafl.to_string());
    col_strings.push(Col::nafl.to_string());
    col_strings.push(Col::fexf.to_string());
    col_strings.push(Col::nexf.to_string());
    col_strings.push(Col::fflux.to_string());
    col_strings.push(Col::e_fflux.to_string());
    col_strings.push(Col::nflux.to_string());
    col_strings.push(Col::e_nflux.to_string());
    col_strings.push(Col::fxpos.to_string());
    col_strings.push(Col::fypos.to_string());
    col_strings.push(Col::nxpos.to_string());
    col_strings.push(Col::nypos.to_string());
    col_strings.push(Col::fima.to_string());
    col_strings.push(Col::nima.to_string());
    col_strings.push(Col::fr.to_string());
    col_strings.push(Col::nr.to_string());
    col_strings.push(Col::ns_g.to_string());
    col_strings.push(Col::fs_g.to_string());
    col_strings.push(Col::nell.to_string());
    col_strings.push(Col::fell.to_string());
    col_strings.push(Col::npa.to_string());
    col_strings.push(Col::e_npa.to_string());
    col_strings.push(Col::fpa.to_string());
    col_strings.push(Col::e_fpa.to_string());
    col_strings.push(Col::fnr.to_string());
    col_strings.push(Col::f3r.to_string());
    col_strings.push(Col::nar.to_string());
    col_strings.push(Col::narms.to_string());
    col_strings.push(Col::nbrms.to_string());
    col_strings.push(Col::far.to_string());
    col_strings.push(Col::farms.to_string());
    col_strings.push(Col::fbrms.to_string());
    col_strings.push(Col::nuvw.to_string());
    col_strings.push(Col::fuvw.to_string());
    col_strings.push(Col::prob.to_string());
    col_strings.push(Col::sep.to_string());
    col_strings.push(Col::nerr.to_string());
    col_strings.push(Col::ferr.to_string());
    col_strings.push(Col::ierr.to_string());
    col_strings.push(Col::nperr.to_string());
    col_strings.push(Col::fperr.to_string());
    col_strings.push(Col::cv.to_string());
    col_strings.push(Col::g.to_string());
    col_strings.push(Col::n.to_string());
    col_strings.push(Col::primid.to_string());
    col_strings.push(Col::groupid.to_string());
    col_strings.push(Col::gd.to_string());
    col_strings.push(Col::nd.to_string());
    col_strings.push(Col::primidd.to_string());
    col_strings.push(Col::groupidd.to_string());
    col_strings.push(Col::grouptot.to_string());
    col_strings.push(Col::oname.to_string());
    col_strings.push(Col::size.to_string());
    map.insert(galex_ais.string(), col_strings);
}
