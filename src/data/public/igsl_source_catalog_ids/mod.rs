
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct igsl_source_catalog_ids;

impl Schema for igsl_source_catalog_ids {
    fn string(&self) -> String {
        "igsl_source_catalog_ids".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    id_epc,
id_gsc23,
id_hip,
id_lqrf,
id_ogle,
id_ppmxl,
id_sdss,
id_tmass,
id_tycho,
id_ucac,
solution_id,
source_id,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::id_epc.to_string());
col_strings.push(Col::id_gsc23.to_string());
col_strings.push(Col::id_hip.to_string());
col_strings.push(Col::id_lqrf.to_string());
col_strings.push(Col::id_ogle.to_string());
col_strings.push(Col::id_ppmxl.to_string());
col_strings.push(Col::id_sdss.to_string());
col_strings.push(Col::id_tmass.to_string());
col_strings.push(Col::id_tycho.to_string());
col_strings.push(Col::id_ucac.to_string());
col_strings.push(Col::solution_id.to_string());
col_strings.push(Col::source_id.to_string());
    map.insert(igsl_source_catalog_ids.string(), col_strings);
}
