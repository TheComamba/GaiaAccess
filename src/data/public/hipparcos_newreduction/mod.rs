// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct hipparcos_newreduction;

impl Schema for hipparcos_newreduction {
    fn string(&self) -> String {
        "hipparcos_newreduction".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    hip,
    ic,
    ra,
    dec,
    ra_rad,
    de_rad,
    plx,
    pm_ra,
    pm_de,
    e_ra_rad,
    e_de_rad,
    e_plx,
    e_pm_ra,
    e_pm_de,
    f1,
    f2,
    nc,
    ntr,
    u3,
    u4,
    u5,
    u6,
    u7,
    u8,
    u9,
    sn,
    so,
    var,
    u1,
    u2,
    u10,
    u11,
    u12,
    u13,
    u14,
    u15,
    hp_mag,
    b_v,
    v_i,
    e_hp_mag,
    e_b_v,
    s_hp,
    va,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::hip.to_string());
    col_strings.push(Col::ic.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::ra_rad.to_string());
    col_strings.push(Col::de_rad.to_string());
    col_strings.push(Col::plx.to_string());
    col_strings.push(Col::pm_ra.to_string());
    col_strings.push(Col::pm_de.to_string());
    col_strings.push(Col::e_ra_rad.to_string());
    col_strings.push(Col::e_de_rad.to_string());
    col_strings.push(Col::e_plx.to_string());
    col_strings.push(Col::e_pm_ra.to_string());
    col_strings.push(Col::e_pm_de.to_string());
    col_strings.push(Col::f1.to_string());
    col_strings.push(Col::f2.to_string());
    col_strings.push(Col::nc.to_string());
    col_strings.push(Col::ntr.to_string());
    col_strings.push(Col::u3.to_string());
    col_strings.push(Col::u4.to_string());
    col_strings.push(Col::u5.to_string());
    col_strings.push(Col::u6.to_string());
    col_strings.push(Col::u7.to_string());
    col_strings.push(Col::u8.to_string());
    col_strings.push(Col::u9.to_string());
    col_strings.push(Col::sn.to_string());
    col_strings.push(Col::so.to_string());
    col_strings.push(Col::var.to_string());
    col_strings.push(Col::u1.to_string());
    col_strings.push(Col::u2.to_string());
    col_strings.push(Col::u10.to_string());
    col_strings.push(Col::u11.to_string());
    col_strings.push(Col::u12.to_string());
    col_strings.push(Col::u13.to_string());
    col_strings.push(Col::u14.to_string());
    col_strings.push(Col::u15.to_string());
    col_strings.push(Col::hp_mag.to_string());
    col_strings.push(Col::b_v.to_string());
    col_strings.push(Col::v_i.to_string());
    col_strings.push(Col::e_hp_mag.to_string());
    col_strings.push(Col::e_b_v.to_string());
    col_strings.push(Col::s_hp.to_string());
    col_strings.push(Col::va.to_string());
    map.insert(hipparcos_newreduction.string(), col_strings);
}
