use crate::schema::Schema;

pub struct HubbleSc;

impl Schema for HubbleSc {
    fn string(&self) -> String {
        "hubble_sc".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    abscorr,
    a_f435w,
    a_f435w_n,
    a_f435w_sigma,
    a_f475w,
    a_f475w_n,
    a_f475w_sigma,
    a_f502n,
    a_f502n_n,
    a_f502n_sigma,
    a_f550m,
    a_f550m_n,
    a_f550m_sigma,
    a_f555w,
    a_f555w_n,
    a_f555w_sigma,
    a_f606w,
    a_f606w_n,
    a_f606w_sigma,
    a_f625w,
    a_f625w_n,
    a_f625w_sigma,
    a_f658n,
    a_f658n_n,
    a_f658n_sigma,
    a_f660n,
    a_f660n_n,
    a_f660n_sigma,
    a_f775w,
    a_f775w_n,
    a_f775w_sigma,
    a_f814w,
    a_f814w_n,
    a_f814w_sigma,
    a_f850lp,
    a_f850lp_n,
    a_f850lp_sigma,
    ci,
    ci_sigma,
    dsigma,
    extinction,
    htmid,
    kronradius,
    kronradius_sigma,
    matchdec,
    match_id,
    matchra,
    numfilters,
    numimages,
    numvisits,
    startmjd,
    starttime,
    stopmjd,
    stoptime,
    targetname,
    w2_f1042m,
    w2_f1042m_n,
    w2_f1042m_sigma,
    w2_f122m,
    w2_f122m_n,
    w2_f122m_sigma,
    w2_f160bn15,
    w2_f160bn15_n,
    w2_f160bn15_sigma,
    w2_f160bw,
    w2_f160bw_n,
    w2_f160bw_sigma,
    w2_f170w,
    w2_f170w_n,
    w2_f170w_sigma,
    w2_f185w,
    w2_f185w_n,
    w2_f185w_sigma,
    w2_f218w,
    w2_f218w_n,
    w2_f218w_sigma,
    w2_f255w,
    w2_f255w_n,
    w2_f255w_sigma,
    w2_f300w,
    w2_f300w_n,
    w2_f300w_sigma,
    w2_f336w,
    w2_f336w_n,
    w2_f336w_sigma,
    w2_f343n,
    w2_f343n_n,
    w2_f343n_sigma,
    w2_f375n,
    w2_f375n_n,
    w2_f375n_sigma,
    w2_f380w,
    w2_f380w_n,
    w2_f380w_sigma,
    w2_f390n,
    w2_f390n_n,
    w2_f390n_sigma,
    w2_f410m,
    w2_f410m_n,
    w2_f410m_sigma,
    w2_f437n,
    w2_f437n_n,
    w2_f437n_sigma,
    w2_f439w,
    w2_f439w_n,
    w2_f439w_sigma,
    w2_f450w,
    w2_f450w_n,
    w2_f450w_sigma,
    w2_f467m,
    w2_f467m_n,
    w2_f467m_sigma,
    w2_f469n,
    w2_f469n_n,
    w2_f469n_sigma,
    w2_f487n,
    w2_f487n_n,
    w2_f487n_sigma,
    w2_f502n,
    w2_f502n_n,
    w2_f502n_sigma,
    w2_f547m,
    w2_f547m_n,
    w2_f547m_sigma,
    w2_f555w,
    w2_f555w_n,
    w2_f555w_sigma,
    w2_f569w,
    w2_f569w_n,
    w2_f569w_sigma,
    w2_f588n,
    w2_f588n_n,
    w2_f588n_sigma,
    w2_f606w,
    w2_f606w_n,
    w2_f606w_sigma,
    w2_f622w,
    w2_f622w_n,
    w2_f622w_sigma,
    w2_f631n,
    w2_f631n_n,
    w2_f631n_sigma,
    w2_f656n,
    w2_f656n_n,
    w2_f656n_sigma,
    w2_f658n,
    w2_f658n_n,
    w2_f658n_sigma,
    w2_f673n,
    w2_f673n_n,
    w2_f673n_sigma,
    w2_f675w,
    w2_f675w_n,
    w2_f675w_sigma,
    w2_f702w,
    w2_f702w_n,
    w2_f702w_sigma,
    w2_f785lp,
    w2_f785lp_n,
    w2_f785lp_sigma,
    w2_f791w,
    w2_f791w_n,
    w2_f791w_sigma,
    w2_f814w,
    w2_f814w_n,
    w2_f814w_sigma,
    w2_f850lp,
    w2_f850lp_n,
    w2_f850lp_sigma,
    w2_f953n,
    w2_f953n_n,
    w2_f953n_sigma,
    w3_blank,
    w3_blank_n,
    w3_blank_sigma,
    w3_f098m,
    w3_f098m_n,
    w3_f098m_sigma,
    w3_f105w,
    w3_f105w_n,
    w3_f105w_sigma,
    w3_f110w,
    w3_f110w_n,
    w3_f110w_sigma,
    w3_f125w,
    w3_f125w_n,
    w3_f125w_sigma,
    w3_f126n,
    w3_f126n_n,
    w3_f126n_sigma,
    w3_f127m,
    w3_f127m_n,
    w3_f127m_sigma,
    w3_f128n,
    w3_f128n_n,
    w3_f128n_sigma,
    w3_f130n,
    w3_f130n_n,
    w3_f130n_sigma,
    w3_f132n,
    w3_f132n_n,
    w3_f132n_sigma,
    w3_f139m,
    w3_f139m_n,
    w3_f139m_sigma,
    w3_f140w,
    w3_f140w_n,
    w3_f140w_sigma,
    w3_f153m,
    w3_f153m_n,
    w3_f153m_sigma,
    w3_f160w,
    w3_f160w_n,
    w3_f160w_sigma,
    w3_f164n,
    w3_f164n_n,
    w3_f164n_sigma,
    w3_f167n,
    w3_f167n_n,
    w3_f167n_sigma,
    w3_f200lp,
    w3_f200lp_n,
    w3_f200lp_sigma,
    w3_f218w,
    w3_f218w_n,
    w3_f218w_sigma,
    w3_f225w,
    w3_f225w_n,
    w3_f225w_sigma,
    w3_f275w,
    w3_f275w_n,
    w3_f275w_sigma,
    w3_f280n,
    w3_f280n_n,
    w3_f280n_sigma,
    w3_f300x,
    w3_f300x_n,
    w3_f300x_sigma,
    w3_f336w,
    w3_f336w_n,
    w3_f336w_sigma,
    w3_f343n,
    w3_f343n_n,
    w3_f343n_sigma,
    w3_f350lp,
    w3_f350lp_n,
    w3_f350lp_sigma,
    w3_f373n,
    w3_f373n_n,
    w3_f373n_sigma,
    w3_f390m,
    w3_f390m_n,
    w3_f390m_sigma,
    w3_f390w,
    w3_f390w_n,
    w3_f390w_sigma,
    w3_f395n,
    w3_f395n_n,
    w3_f395n_sigma,
    w3_f410m,
    w3_f410m_n,
    w3_f410m_sigma,
    w3_f438w,
    w3_f438w_n,
    w3_f438w_sigma,
    w3_f467m,
    w3_f467m_n,
    w3_f467m_sigma,
    w3_f469n,
    w3_f469n_n,
    w3_f469n_sigma,
    w3_f475w,
    w3_f475w_n,
    w3_f475w_sigma,
    w3_f475x,
    w3_f475x_n,
    w3_f475x_sigma,
    w3_f487n,
    w3_f487n_n,
    w3_f487n_sigma,
    w3_f502n,
    w3_f502n_n,
    w3_f502n_sigma,
    w3_f547m,
    w3_f547m_n,
    w3_f547m_sigma,
    w3_f555w,
    w3_f555w_n,
    w3_f555w_sigma,
    w3_f600lp,
    w3_f600lp_n,
    w3_f600lp_sigma,
    w3_f606w,
    w3_f606w_n,
    w3_f606w_sigma,
    w3_f621m,
    w3_f621m_n,
    w3_f621m_sigma,
    w3_f625w,
    w3_f625w_n,
    w3_f625w_sigma,
    w3_f631n,
    w3_f631n_n,
    w3_f631n_sigma,
    w3_f645n,
    w3_f645n_n,
    w3_f645n_sigma,
    w3_f656n,
    w3_f656n_n,
    w3_f656n_sigma,
    w3_f657n,
    w3_f657n_n,
    w3_f657n_sigma,
    w3_f658n,
    w3_f658n_n,
    w3_f658n_sigma,
    w3_f665n,
    w3_f665n_f6,
    w3_f665n_f6_n,
    w3_f665n_f6_sigma,
    w3_f665n_n,
    w3_f665n_sigma,
    w3_f673n,
    w3_f673n_n,
    w3_f673n_sigma,
    w3_f680n,
    w3_f680n_n,
    w3_f680n_sigma,
    w3_f689m,
    w3_f689m_n,
    w3_f689m_sigma,
    w3_f763m,
    w3_f763m_n,
    w3_f763m_sigma,
    w3_f775w,
    w3_f775w_n,
    w3_f775w_sigma,
    w3_f814w,
    w3_f814w_n,
    w3_f814w_sigma,
    w3_f845m,
    w3_f845m_n,
    w3_f845m_sigma,
    w3_f850lp,
    w3_f850lp_n,
    w3_f850lp_sigma,
    w3_f953n,
    w3_f953n_n,
    w3_f953n_sigma,
    w3_fq232n,
    w3_fq232n_n,
    w3_fq232n_sigma,
    w3_fq243n,
    w3_fq243n_n,
    w3_fq243n_sigma,
    w3_fq378n,
    w3_fq378n_n,
    w3_fq378n_sigma,
    w3_fq387n,
    w3_fq387n_n,
    w3_fq387n_sigma,
    w3_fq422m,
    w3_fq422m_n,
    w3_fq422m_sigma,
    w3_fq436n,
    w3_fq436n_n,
    w3_fq436n_sigma,
    w3_fq437n,
    w3_fq437n_n,
    w3_fq437n_sigma,
    w3_fq492n,
    w3_fq492n_n,
    w3_fq492n_sigma,
    w3_fq508n,
    w3_fq508n_n,
    w3_fq508n_sigma,
    w3_fq575n,
    w3_fq575n_n,
    w3_fq575n_sigma,
    w3_fq619n,
    w3_fq619n_n,
    w3_fq619n_sigma,
    w3_fq634n,
    w3_fq634n_n,
    w3_fq634n_sigma,
    w3_fq672n,
    w3_fq672n_n,
    w3_fq672n_sigma,
    w3_fq674n,
    w3_fq674n_n,
    w3_fq674n_sigma,
    w3_fq727n,
    w3_fq727n_n,
    w3_fq727n_sigma,
    w3_fq750n,
    w3_fq750n_n,
    w3_fq750n_sigma,
    w3_fq889n,
    w3_fq889n_n,
    w3_fq889n_sigma,
    w3_fq906n,
    w3_fq906n_n,
    w3_fq906n_sigma,
    w3_fq924n,
    w3_fq924n_n,
    w3_fq924n_sigma,
    w3_fq937n,
    w3_fq937n_n,
    w3_fq937n_sigma,
    w3_g102,
    w3_g102_n,
    w3_g102_sigma,
    w3_g141,
    w3_g141_n,
    w3_g141_sigma,
    w3_g280,
    w3_g280_n,
    w3_g280_sigma,
    x,
    y,
    z,
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(HubbleSc.string(), col_strings);
}