// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the urat1_best_neighbour table.

use crate::traits::{Column, Table};

/// URAT-1 BestNeighbour table lists each matched Gaia object with its best
/// neighbour in the external catalogue.
#[allow(non_camel_case_types)]
pub struct urat1_best_neighbour;

impl Table for urat1_best_neighbour {
    fn string(&self) -> String {
        "urat1_best_neighbour".to_string()
    }
}

/// The columns in the urat1_best_neighbour table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Unique Gaia source identifier
    source_id,
    /// Original External Catalogue source identifier
    original_ext_source_id,
    /// Angular Distance between the two sources
    angular_distance,
    /// Number of Gaia astrometric params used
    gaia_astrometric_params,
    /// External Catalogue source identifier
    urat1_oid,
    /// Number of neighbours in External Catalogue
    number_of_neighbours,
    /// Number of mates in Gaia Catalogue
    number_of_mates,
    /// Number of neighbours with same probability as best neighbour
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
    col_strings.push(Col::urat1_oid.to_string());
    col_strings.push(Col::number_of_neighbours.to_string());
    col_strings.push(Col::number_of_mates.to_string());
    col_strings.push(Col::best_neighbour_multiplicity.to_string());
    map.insert(urat1_best_neighbour.string(), col_strings);
}
