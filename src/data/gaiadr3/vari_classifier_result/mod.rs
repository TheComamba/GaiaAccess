// This code is generated by generate_code.py, do not modify it manually
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct vari_classifier_result;

impl Table for vari_classifier_result {
    fn string(&self) -> String {
        "vari_classifier_result".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    classifier_name,
    best_class_name,
    best_class_score,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::classifier_name.to_string());
    col_strings.push(Col::best_class_name.to_string());
    col_strings.push(Col::best_class_score.to_string());
    map.insert(vari_classifier_result.string(), col_strings);
}
