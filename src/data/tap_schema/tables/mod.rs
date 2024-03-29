// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct tables;

impl Table for tables {
    fn string(&self) -> String {
        "tables".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    db_table_name,
    description,
    schema_name,
    #[strum(serialize = "\"size\"")]
    size,
    size_bytes,
    table_index,
    table_name,
    table_type,
    utype,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::db_table_name.to_string());
    col_strings.push(Col::description.to_string());
    col_strings.push(Col::schema_name.to_string());
    col_strings.push(Col::size.to_string());
    col_strings.push(Col::size_bytes.to_string());
    col_strings.push(Col::table_index.to_string());
    col_strings.push(Col::table_name.to_string());
    col_strings.push(Col::table_type.to_string());
    col_strings.push(Col::utype.to_string());
    map.insert(tables.string(), col_strings);
}
