// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct panstarrs1_original_valid;

impl Table for panstarrs1_original_valid {
    fn string(&self) -> String {
        "panstarrs1_original_valid".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    obj_name,
    obj_id,
    ra,
    dec,
    ra_error,
    dec_error,
    epoch_mean,
    g_mean_psf_mag,
    g_mean_psf_mag_error,
    g_flags,
    r_mean_psf_mag,
    r_mean_psf_mag_error,
    r_flags,
    i_mean_psf_mag,
    i_mean_psf_mag_error,
    i_flags,
    z_mean_psf_mag,
    z_mean_psf_mag_error,
    z_flags,
    y_mean_psf_mag,
    y_mean_psf_mag_error,
    y_flags,
    n_detections,
    zone_id,
    obj_info_flag,
    quality_flag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::obj_name.to_string());
    col_strings.push(Col::obj_id.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::epoch_mean.to_string());
    col_strings.push(Col::g_mean_psf_mag.to_string());
    col_strings.push(Col::g_mean_psf_mag_error.to_string());
    col_strings.push(Col::g_flags.to_string());
    col_strings.push(Col::r_mean_psf_mag.to_string());
    col_strings.push(Col::r_mean_psf_mag_error.to_string());
    col_strings.push(Col::r_flags.to_string());
    col_strings.push(Col::i_mean_psf_mag.to_string());
    col_strings.push(Col::i_mean_psf_mag_error.to_string());
    col_strings.push(Col::i_flags.to_string());
    col_strings.push(Col::z_mean_psf_mag.to_string());
    col_strings.push(Col::z_mean_psf_mag_error.to_string());
    col_strings.push(Col::z_flags.to_string());
    col_strings.push(Col::y_mean_psf_mag.to_string());
    col_strings.push(Col::y_mean_psf_mag_error.to_string());
    col_strings.push(Col::y_flags.to_string());
    col_strings.push(Col::n_detections.to_string());
    col_strings.push(Col::zone_id.to_string());
    col_strings.push(Col::obj_info_flag.to_string());
    col_strings.push(Col::quality_flag.to_string());
    map.insert(panstarrs1_original_valid.string(), col_strings);
}
