use std::hash::Hash;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

pub trait Column: ToString + Eq + Hash + Clone + Copy {}

#[allow(non_camel_case_types)]
#[derive(
    Debug, Clone, Copy, EnumIter, EnumString, Display, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum GaiaColumn {}

#[cfg(test)]
mod tests {
    use crate::{
        data::gaiadr3::{gaia_source::*, gaiadr3},
        query::GaiaQueryBuilder,
        result::{get_float, get_string},
    };

    #[test]
    fn request_a_string() {
        let col = Col::designation;
        let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
            .top(1)
            .select(vec![col])
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        assert!(result.data[0].contains_key(&col));
        assert!(get_string(result.data[0].get(&col).unwrap()).is_some());
    }

    #[test]
    fn request_a_float() {
        let col = Col::ecl_lon;
        let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
            .top(1)
            .select(vec![col])
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        assert!(result.data[0].contains_key(&col));
        assert!(get_float(result.data[0].get(&col).unwrap()).is_some());
    }
}
