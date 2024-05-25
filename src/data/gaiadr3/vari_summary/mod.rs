// This code is generated by generate_code.py, do not modify it manually
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct vari_summary;

impl Table for vari_summary {
    fn string(&self) -> String {
        "vari_summary".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    num_selected_g_fov,
    mean_obs_time_g_fov,
    time_duration_g_fov,
    min_mag_g_fov,
    max_mag_g_fov,
    mean_mag_g_fov,
    median_mag_g_fov,
    range_mag_g_fov,
    trimmed_range_mag_g_fov,
    std_dev_mag_g_fov,
    skewness_mag_g_fov,
    kurtosis_mag_g_fov,
    mad_mag_g_fov,
    abbe_mag_g_fov,
    iqr_mag_g_fov,
    stetson_mag_g_fov,
    std_dev_over_rms_err_mag_g_fov,
    outlier_median_g_fov,
    num_selected_bp,
    mean_obs_time_bp,
    time_duration_bp,
    min_mag_bp,
    max_mag_bp,
    mean_mag_bp,
    median_mag_bp,
    range_mag_bp,
    trimmed_range_mag_bp,
    std_dev_mag_bp,
    skewness_mag_bp,
    kurtosis_mag_bp,
    mad_mag_bp,
    abbe_mag_bp,
    iqr_mag_bp,
    stetson_mag_bp,
    std_dev_over_rms_err_mag_bp,
    outlier_median_bp,
    num_selected_rp,
    mean_obs_time_rp,
    time_duration_rp,
    min_mag_rp,
    max_mag_rp,
    mean_mag_rp,
    median_mag_rp,
    range_mag_rp,
    trimmed_range_mag_rp,
    std_dev_mag_rp,
    skewness_mag_rp,
    kurtosis_mag_rp,
    mad_mag_rp,
    abbe_mag_rp,
    iqr_mag_rp,
    stetson_mag_rp,
    std_dev_over_rms_err_mag_rp,
    outlier_median_rp,
    in_vari_classification_result,
    in_vari_rrlyrae,
    in_vari_cepheid,
    in_vari_planetary_transit,
    in_vari_short_timescale,
    in_vari_long_period_variable,
    in_vari_eclipsing_binary,
    in_vari_rotation_modulation,
    in_vari_ms_oscillator,
    in_vari_agn,
    in_vari_microlensing,
    in_vari_compact_companion,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::num_selected_g_fov.to_string());
    col_strings.push(Col::mean_obs_time_g_fov.to_string());
    col_strings.push(Col::time_duration_g_fov.to_string());
    col_strings.push(Col::min_mag_g_fov.to_string());
    col_strings.push(Col::max_mag_g_fov.to_string());
    col_strings.push(Col::mean_mag_g_fov.to_string());
    col_strings.push(Col::median_mag_g_fov.to_string());
    col_strings.push(Col::range_mag_g_fov.to_string());
    col_strings.push(Col::trimmed_range_mag_g_fov.to_string());
    col_strings.push(Col::std_dev_mag_g_fov.to_string());
    col_strings.push(Col::skewness_mag_g_fov.to_string());
    col_strings.push(Col::kurtosis_mag_g_fov.to_string());
    col_strings.push(Col::mad_mag_g_fov.to_string());
    col_strings.push(Col::abbe_mag_g_fov.to_string());
    col_strings.push(Col::iqr_mag_g_fov.to_string());
    col_strings.push(Col::stetson_mag_g_fov.to_string());
    col_strings.push(Col::std_dev_over_rms_err_mag_g_fov.to_string());
    col_strings.push(Col::outlier_median_g_fov.to_string());
    col_strings.push(Col::num_selected_bp.to_string());
    col_strings.push(Col::mean_obs_time_bp.to_string());
    col_strings.push(Col::time_duration_bp.to_string());
    col_strings.push(Col::min_mag_bp.to_string());
    col_strings.push(Col::max_mag_bp.to_string());
    col_strings.push(Col::mean_mag_bp.to_string());
    col_strings.push(Col::median_mag_bp.to_string());
    col_strings.push(Col::range_mag_bp.to_string());
    col_strings.push(Col::trimmed_range_mag_bp.to_string());
    col_strings.push(Col::std_dev_mag_bp.to_string());
    col_strings.push(Col::skewness_mag_bp.to_string());
    col_strings.push(Col::kurtosis_mag_bp.to_string());
    col_strings.push(Col::mad_mag_bp.to_string());
    col_strings.push(Col::abbe_mag_bp.to_string());
    col_strings.push(Col::iqr_mag_bp.to_string());
    col_strings.push(Col::stetson_mag_bp.to_string());
    col_strings.push(Col::std_dev_over_rms_err_mag_bp.to_string());
    col_strings.push(Col::outlier_median_bp.to_string());
    col_strings.push(Col::num_selected_rp.to_string());
    col_strings.push(Col::mean_obs_time_rp.to_string());
    col_strings.push(Col::time_duration_rp.to_string());
    col_strings.push(Col::min_mag_rp.to_string());
    col_strings.push(Col::max_mag_rp.to_string());
    col_strings.push(Col::mean_mag_rp.to_string());
    col_strings.push(Col::median_mag_rp.to_string());
    col_strings.push(Col::range_mag_rp.to_string());
    col_strings.push(Col::trimmed_range_mag_rp.to_string());
    col_strings.push(Col::std_dev_mag_rp.to_string());
    col_strings.push(Col::skewness_mag_rp.to_string());
    col_strings.push(Col::kurtosis_mag_rp.to_string());
    col_strings.push(Col::mad_mag_rp.to_string());
    col_strings.push(Col::abbe_mag_rp.to_string());
    col_strings.push(Col::iqr_mag_rp.to_string());
    col_strings.push(Col::stetson_mag_rp.to_string());
    col_strings.push(Col::std_dev_over_rms_err_mag_rp.to_string());
    col_strings.push(Col::outlier_median_rp.to_string());
    col_strings.push(Col::in_vari_classification_result.to_string());
    col_strings.push(Col::in_vari_rrlyrae.to_string());
    col_strings.push(Col::in_vari_cepheid.to_string());
    col_strings.push(Col::in_vari_planetary_transit.to_string());
    col_strings.push(Col::in_vari_short_timescale.to_string());
    col_strings.push(Col::in_vari_long_period_variable.to_string());
    col_strings.push(Col::in_vari_eclipsing_binary.to_string());
    col_strings.push(Col::in_vari_rotation_modulation.to_string());
    col_strings.push(Col::in_vari_ms_oscillator.to_string());
    col_strings.push(Col::in_vari_agn.to_string());
    col_strings.push(Col::in_vari_microlensing.to_string());
    col_strings.push(Col::in_vari_compact_companion.to_string());
    map.insert(vari_summary.string(), col_strings);
}
