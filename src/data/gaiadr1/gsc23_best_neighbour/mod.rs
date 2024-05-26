// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gsc23_best_neighbour table.

use crate::traits::{Column, Table};

/// GSC2.3 implementation of BaseBestNeighbour
#[allow(non_camel_case_types)]
pub struct gsc23_best_neighbour;

impl Table for gsc23_best_neighbour {
    fn string(&self) -> String {
        "gsc23_best_neighbour".to_string()
    }
}

/// The columns in the gsc23_best_neighbour table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// The additional numeric unique source identifier of the External
    /// catalogue, increasing with Declination.
    gsc23_oid,
    /// Unique identifier of the Gaia source, the attribute corresponds to
    /// GaiaSource.sourceId
    source_id,
    /// The unique source identifier in the original External catalogue.
    original_ext_source_id,
    /// Angular distance between a Gaia source and its nearest neighbour in the
    /// External Catalogue
    angular_distance,
    /// Number of sources in the External Catalogue which match the Gaia source
    /// within position errors.  
    /// The identifiers of all the neighbours can be found in the Neighbourhood
    /// table.
    number_of_neighbours,
    /// Number of other Gaia sources that have as best-neighbour the same
    /// External Catalogue source.  
    /// In case there are no other Gaia sources with the same best-neighbour in
    /// the external catalogue, the number of mates is equal to zero.  
    /// Given the Gaia high angular resolution, it will happen that what appears
    /// as a single object in an external catalogue will be resolved by Gaia and
    /// as such will be the best-match of more than one Gaia object.
    number_of_mates,
    /// The best-match to a Gaia source in an external catalogue is the source
    /// in the external catalogue that has the highest probability to be the
    /// best-match.  
    /// As the probability is based on positional and density properties, it
    /// could happen that there is more than one source in the external
    /// catalogue with the same probability.  
    /// Even if a single best-match is always chosen, this field tells the user
    /// if there were more “best” neighbours. Those neighbours can be found in
    /// the Neighbourhood table.
    best_neighbour_multiplicity,
    /// This flag is set to 0 if Gaia proper motions were not available and were
    /// thus not used in the XMatch.  
    /// This flag is set to 1 if Gaia proper motions were available and were
    /// thus used in the XMatch (for the First Gaia release the TGASS
    /// sub-sample.
    proper_motion_flag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::gsc23_oid.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::number_of_neighbours.to_string());
    col_strings.push(Col::number_of_mates.to_string());
    col_strings.push(Col::best_neighbour_multiplicity.to_string());
    col_strings.push(Col::proper_motion_flag.to_string());
    map.insert(gsc23_best_neighbour.string(), col_strings);
}
