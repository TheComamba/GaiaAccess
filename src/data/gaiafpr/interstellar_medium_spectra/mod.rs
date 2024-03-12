
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct interstellar_medium_spectra;

impl Schema for interstellar_medium_spectra {
    fn string(&self) -> String {
        "interstellar_medium_spectra".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
healpix,
lc,
bc,
dc,
lambda,
flux,
flux_uncertainty,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::solution_id.to_string());
col_strings.push(Col::healpix.to_string());
col_strings.push(Col::lc.to_string());
col_strings.push(Col::bc.to_string());
col_strings.push(Col::dc.to_string());
col_strings.push(Col::lambda.to_string());
col_strings.push(Col::flux.to_string());
col_strings.push(Col::flux_uncertainty.to_string());
    map.insert(interstellar_medium_spectra.string(), col_strings);
}
