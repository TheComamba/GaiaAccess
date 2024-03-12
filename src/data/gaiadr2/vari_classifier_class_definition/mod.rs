
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct vari_classifier_class_definition;

impl Schema for vari_classifier_class_definition {
    fn string(&self) -> String {
        "vari_classifier_class_definition".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
classifier_name,
class_name,
class_description,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::solution_id.to_string());
col_strings.push(Col::classifier_name.to_string());
col_strings.push(Col::class_name.to_string());
col_strings.push(Col::class_description.to_string());
    map.insert(vari_classifier_class_definition.string(), col_strings);
}
