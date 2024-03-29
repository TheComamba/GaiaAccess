// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct coord_sys;

impl Table for coord_sys {
    fn string(&self) -> String {
        "coord_sys".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    epoch,
    equinox,
    id,
    system,
    xml_output,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::epoch.to_string());
    col_strings.push(Col::equinox.to_string());
    col_strings.push(Col::id.to_string());
    col_strings.push(Col::system.to_string());
    col_strings.push(Col::xml_output.to_string());
    map.insert(coord_sys.string(), col_strings);
}
