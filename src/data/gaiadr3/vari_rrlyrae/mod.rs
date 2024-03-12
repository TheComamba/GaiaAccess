
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct vari_rrlyrae;

impl Schema for vari_rrlyrae {
    fn string(&self) -> String {
        "vari_rrlyrae".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
source_id,
pf,
pf_error,
p1_o,
p1_o_error,
epoch_g,
epoch_g_error,
epoch_bp,
epoch_bp_error,
epoch_rp,
epoch_rp_error,
epoch_rv,
epoch_rv_error,
int_average_g,
int_average_g_error,
int_average_bp,
int_average_bp_error,
int_average_rp,
int_average_rp_error,
average_rv,
average_rv_error,
peak_to_peak_g,
peak_to_peak_g_error,
peak_to_peak_bp,
peak_to_peak_bp_error,
peak_to_peak_rp,
peak_to_peak_rp_error,
peak_to_peak_rv,
peak_to_peak_rv_error,
metallicity,
metallicity_error,
r21_g,
r21_g_error,
r31_g,
r31_g_error,
phi21_g,
phi21_g_error,
phi31_g,
phi31_g_error,
num_clean_epochs_g,
num_clean_epochs_bp,
num_clean_epochs_rp,
num_clean_epochs_rv,
zp_mag_g,
zp_mag_bp,
zp_mag_rp,
num_harmonics_for_p1_g,
num_harmonics_for_p1_bp,
num_harmonics_for_p1_rp,
num_harmonics_for_p1_rv,
reference_time_g,
reference_time_bp,
reference_time_rp,
reference_time_rv,
fund_freq1,
fund_freq1_error,
fund_freq2,
fund_freq2_error,
fund_freq1_harmonic_ampl_g,
fund_freq1_harmonic_ampl_g_error,
fund_freq1_harmonic_phase_g,
fund_freq1_harmonic_phase_g_error,
fund_freq1_harmonic_ampl_bp,
fund_freq1_harmonic_ampl_bp_error,
fund_freq1_harmonic_phase_bp,
fund_freq1_harmonic_phase_bp_error,
fund_freq1_harmonic_ampl_rp,
fund_freq1_harmonic_ampl_rp_error,
fund_freq1_harmonic_phase_rp,
fund_freq1_harmonic_phase_rp_error,
fund_freq1_harmonic_ampl_rv,
fund_freq1_harmonic_ampl_rv_error,
fund_freq1_harmonic_phase_rv,
fund_freq1_harmonic_phase_rv_error,
best_classification,
g_absorption,
g_absorption_error,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::solution_id.to_string());
col_strings.push(Col::source_id.to_string());
col_strings.push(Col::pf.to_string());
col_strings.push(Col::pf_error.to_string());
col_strings.push(Col::p1_o.to_string());
col_strings.push(Col::p1_o_error.to_string());
col_strings.push(Col::epoch_g.to_string());
col_strings.push(Col::epoch_g_error.to_string());
col_strings.push(Col::epoch_bp.to_string());
col_strings.push(Col::epoch_bp_error.to_string());
col_strings.push(Col::epoch_rp.to_string());
col_strings.push(Col::epoch_rp_error.to_string());
col_strings.push(Col::epoch_rv.to_string());
col_strings.push(Col::epoch_rv_error.to_string());
col_strings.push(Col::int_average_g.to_string());
col_strings.push(Col::int_average_g_error.to_string());
col_strings.push(Col::int_average_bp.to_string());
col_strings.push(Col::int_average_bp_error.to_string());
col_strings.push(Col::int_average_rp.to_string());
col_strings.push(Col::int_average_rp_error.to_string());
col_strings.push(Col::average_rv.to_string());
col_strings.push(Col::average_rv_error.to_string());
col_strings.push(Col::peak_to_peak_g.to_string());
col_strings.push(Col::peak_to_peak_g_error.to_string());
col_strings.push(Col::peak_to_peak_bp.to_string());
col_strings.push(Col::peak_to_peak_bp_error.to_string());
col_strings.push(Col::peak_to_peak_rp.to_string());
col_strings.push(Col::peak_to_peak_rp_error.to_string());
col_strings.push(Col::peak_to_peak_rv.to_string());
col_strings.push(Col::peak_to_peak_rv_error.to_string());
col_strings.push(Col::metallicity.to_string());
col_strings.push(Col::metallicity_error.to_string());
col_strings.push(Col::r21_g.to_string());
col_strings.push(Col::r21_g_error.to_string());
col_strings.push(Col::r31_g.to_string());
col_strings.push(Col::r31_g_error.to_string());
col_strings.push(Col::phi21_g.to_string());
col_strings.push(Col::phi21_g_error.to_string());
col_strings.push(Col::phi31_g.to_string());
col_strings.push(Col::phi31_g_error.to_string());
col_strings.push(Col::num_clean_epochs_g.to_string());
col_strings.push(Col::num_clean_epochs_bp.to_string());
col_strings.push(Col::num_clean_epochs_rp.to_string());
col_strings.push(Col::num_clean_epochs_rv.to_string());
col_strings.push(Col::zp_mag_g.to_string());
col_strings.push(Col::zp_mag_bp.to_string());
col_strings.push(Col::zp_mag_rp.to_string());
col_strings.push(Col::num_harmonics_for_p1_g.to_string());
col_strings.push(Col::num_harmonics_for_p1_bp.to_string());
col_strings.push(Col::num_harmonics_for_p1_rp.to_string());
col_strings.push(Col::num_harmonics_for_p1_rv.to_string());
col_strings.push(Col::reference_time_g.to_string());
col_strings.push(Col::reference_time_bp.to_string());
col_strings.push(Col::reference_time_rp.to_string());
col_strings.push(Col::reference_time_rv.to_string());
col_strings.push(Col::fund_freq1.to_string());
col_strings.push(Col::fund_freq1_error.to_string());
col_strings.push(Col::fund_freq2.to_string());
col_strings.push(Col::fund_freq2_error.to_string());
col_strings.push(Col::fund_freq1_harmonic_ampl_g.to_string());
col_strings.push(Col::fund_freq1_harmonic_ampl_g_error.to_string());
col_strings.push(Col::fund_freq1_harmonic_phase_g.to_string());
col_strings.push(Col::fund_freq1_harmonic_phase_g_error.to_string());
col_strings.push(Col::fund_freq1_harmonic_ampl_bp.to_string());
col_strings.push(Col::fund_freq1_harmonic_ampl_bp_error.to_string());
col_strings.push(Col::fund_freq1_harmonic_phase_bp.to_string());
col_strings.push(Col::fund_freq1_harmonic_phase_bp_error.to_string());
col_strings.push(Col::fund_freq1_harmonic_ampl_rp.to_string());
col_strings.push(Col::fund_freq1_harmonic_ampl_rp_error.to_string());
col_strings.push(Col::fund_freq1_harmonic_phase_rp.to_string());
col_strings.push(Col::fund_freq1_harmonic_phase_rp_error.to_string());
col_strings.push(Col::fund_freq1_harmonic_ampl_rv.to_string());
col_strings.push(Col::fund_freq1_harmonic_ampl_rv_error.to_string());
col_strings.push(Col::fund_freq1_harmonic_phase_rv.to_string());
col_strings.push(Col::fund_freq1_harmonic_phase_rv_error.to_string());
col_strings.push(Col::best_classification.to_string());
col_strings.push(Col::g_absorption.to_string());
col_strings.push(Col::g_absorption_error.to_string());
    map.insert(vari_rrlyrae.string(), col_strings);
}
