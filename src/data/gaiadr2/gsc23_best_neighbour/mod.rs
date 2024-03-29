// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct gsc23_best_neighbour;

impl Table for gsc23_best_neighbour {
    fn string(&self) -> String {
        "gsc23_best_neighbour".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    original_ext_source_id,
    angular_distance,
    gaia_astrometric_params,
    gsc23_oid,
    number_of_neighbours,
    number_of_mates,
    best_neighbour_multiplicity,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::gaia_astrometric_params.to_string());
    col_strings.push(Col::gsc23_oid.to_string());
    col_strings.push(Col::number_of_neighbours.to_string());
    col_strings.push(Col::number_of_mates.to_string());
    col_strings.push(Col::best_neighbour_multiplicity.to_string());
    map.insert(gsc23_best_neighbour.string(), col_strings);
}
