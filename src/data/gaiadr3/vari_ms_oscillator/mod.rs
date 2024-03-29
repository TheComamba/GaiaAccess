// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct vari_ms_oscillator;

impl Table for vari_ms_oscillator {
    fn string(&self) -> String {
        "vari_ms_oscillator".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    frequency1,
    fap_g_freq1,
    amplitude_g_freq1,
    phase_g_freq1,
    num_harmonics,
    amplitude_g_freq1_harm2,
    phase_g_freq1_harm2,
    amplitude_g_freq1_harm3,
    phase_g_freq1_harm3,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::frequency1.to_string());
    col_strings.push(Col::fap_g_freq1.to_string());
    col_strings.push(Col::amplitude_g_freq1.to_string());
    col_strings.push(Col::phase_g_freq1.to_string());
    col_strings.push(Col::num_harmonics.to_string());
    col_strings.push(Col::amplitude_g_freq1_harm2.to_string());
    col_strings.push(Col::phase_g_freq1_harm2.to_string());
    col_strings.push(Col::amplitude_g_freq1_harm3.to_string());
    col_strings.push(Col::phase_g_freq1_harm3.to_string());
    map.insert(vari_ms_oscillator.string(), col_strings);
}
