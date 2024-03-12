
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct panstarrs1_neighbourhood;

impl Schema for panstarrs1_neighbourhood {
    fn string(&self) -> String {
        "panstarrs1_neighbourhood".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
clean_panstarrs1_oid,
original_ext_source_id,
angular_distance,
score,
xm_flag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::source_id.to_string());
col_strings.push(Col::clean_panstarrs1_oid.to_string());
col_strings.push(Col::original_ext_source_id.to_string());
col_strings.push(Col::angular_distance.to_string());
col_strings.push(Col::score.to_string());
col_strings.push(Col::xm_flag.to_string());
    map.insert(panstarrs1_neighbourhood.string(), col_strings);
}
