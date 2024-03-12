// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct vari_rotation_modulation;

impl Table for vari_rotation_modulation {
    fn string(&self) -> String {
        "vari_rotation_modulation".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    num_segments,
    segments_start_time,
    segments_end_time,
    segments_colour_mag_intercept,
    segments_colour_mag_intercept_error,
    segments_colour_mag_slope,
    segments_colour_mag_slope_error,
    segments_correlation_coefficient,
    segments_correlation_significance,
    num_outliers,
    outliers_time,
    segments_rotation_period,
    segments_rotation_period_error,
    segments_rotation_period_fap,
    segments_cos_term,
    segments_cos_term_error,
    segments_sin_term,
    segments_sin_term_error,
    segments_a0_term,
    segments_a0_term_error,
    best_rotation_period,
    best_rotation_period_error,
    segments_activity_index,
    segments_activity_index_error,
    max_activity_index,
    max_activity_index_error,
    segments_g_unspotted,
    segments_g_unspotted_error,
    segments_bp_unspotted,
    segments_bp_unspotted_error,
    segments_rp_unspotted,
    segments_rp_unspotted_error,
    g_unspotted,
    g_unspotted_error,
    bp_unspotted,
    bp_unspotted_error,
    rp_unspotted,
    rp_unspotted_error,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::num_segments.to_string());
    col_strings.push(Col::segments_start_time.to_string());
    col_strings.push(Col::segments_end_time.to_string());
    col_strings.push(Col::segments_colour_mag_intercept.to_string());
    col_strings.push(Col::segments_colour_mag_intercept_error.to_string());
    col_strings.push(Col::segments_colour_mag_slope.to_string());
    col_strings.push(Col::segments_colour_mag_slope_error.to_string());
    col_strings.push(Col::segments_correlation_coefficient.to_string());
    col_strings.push(Col::segments_correlation_significance.to_string());
    col_strings.push(Col::num_outliers.to_string());
    col_strings.push(Col::outliers_time.to_string());
    col_strings.push(Col::segments_rotation_period.to_string());
    col_strings.push(Col::segments_rotation_period_error.to_string());
    col_strings.push(Col::segments_rotation_period_fap.to_string());
    col_strings.push(Col::segments_cos_term.to_string());
    col_strings.push(Col::segments_cos_term_error.to_string());
    col_strings.push(Col::segments_sin_term.to_string());
    col_strings.push(Col::segments_sin_term_error.to_string());
    col_strings.push(Col::segments_a0_term.to_string());
    col_strings.push(Col::segments_a0_term_error.to_string());
    col_strings.push(Col::best_rotation_period.to_string());
    col_strings.push(Col::best_rotation_period_error.to_string());
    col_strings.push(Col::segments_activity_index.to_string());
    col_strings.push(Col::segments_activity_index_error.to_string());
    col_strings.push(Col::max_activity_index.to_string());
    col_strings.push(Col::max_activity_index_error.to_string());
    col_strings.push(Col::segments_g_unspotted.to_string());
    col_strings.push(Col::segments_g_unspotted_error.to_string());
    col_strings.push(Col::segments_bp_unspotted.to_string());
    col_strings.push(Col::segments_bp_unspotted_error.to_string());
    col_strings.push(Col::segments_rp_unspotted.to_string());
    col_strings.push(Col::segments_rp_unspotted_error.to_string());
    col_strings.push(Col::g_unspotted.to_string());
    col_strings.push(Col::g_unspotted_error.to_string());
    col_strings.push(Col::bp_unspotted.to_string());
    col_strings.push(Col::bp_unspotted_error.to_string());
    col_strings.push(Col::rp_unspotted.to_string());
    col_strings.push(Col::rp_unspotted_error.to_string());
    map.insert(vari_rotation_modulation.string(), col_strings);
}