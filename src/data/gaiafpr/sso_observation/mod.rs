
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct sso_observation;

impl Schema for sso_observation {
    fn string(&self) -> String {
        "sso_observation".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
source_id,
denomination,
transit_id,
observation_id,
number_mp,
epoch,
epoch_err,
epoch_utc,
ra,
dec,
ra_error_systematic,
dec_error_systematic,
ra_dec_correlation_systematic,
ra_error_random,
dec_error_random,
ra_dec_correlation_random,
x_gaia,
y_gaia,
z_gaia,
vx_gaia,
vy_gaia,
vz_gaia,
x_gaia_geocentric,
y_gaia_geocentric,
z_gaia_geocentric,
vx_gaia_geocentric,
vy_gaia_geocentric,
vz_gaia_geocentric,
position_angle_scan,
astrometric_outcome_ccd,
astrometric_outcome_transit,
fov,
is_rejected,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::solution_id.to_string());
col_strings.push(Col::source_id.to_string());
col_strings.push(Col::denomination.to_string());
col_strings.push(Col::transit_id.to_string());
col_strings.push(Col::observation_id.to_string());
col_strings.push(Col::number_mp.to_string());
col_strings.push(Col::epoch.to_string());
col_strings.push(Col::epoch_err.to_string());
col_strings.push(Col::epoch_utc.to_string());
col_strings.push(Col::ra.to_string());
col_strings.push(Col::dec.to_string());
col_strings.push(Col::ra_error_systematic.to_string());
col_strings.push(Col::dec_error_systematic.to_string());
col_strings.push(Col::ra_dec_correlation_systematic.to_string());
col_strings.push(Col::ra_error_random.to_string());
col_strings.push(Col::dec_error_random.to_string());
col_strings.push(Col::ra_dec_correlation_random.to_string());
col_strings.push(Col::x_gaia.to_string());
col_strings.push(Col::y_gaia.to_string());
col_strings.push(Col::z_gaia.to_string());
col_strings.push(Col::vx_gaia.to_string());
col_strings.push(Col::vy_gaia.to_string());
col_strings.push(Col::vz_gaia.to_string());
col_strings.push(Col::x_gaia_geocentric.to_string());
col_strings.push(Col::y_gaia_geocentric.to_string());
col_strings.push(Col::z_gaia_geocentric.to_string());
col_strings.push(Col::vx_gaia_geocentric.to_string());
col_strings.push(Col::vy_gaia_geocentric.to_string());
col_strings.push(Col::vz_gaia_geocentric.to_string());
col_strings.push(Col::position_angle_scan.to_string());
col_strings.push(Col::astrometric_outcome_ccd.to_string());
col_strings.push(Col::astrometric_outcome_transit.to_string());
col_strings.push(Col::fov.to_string());
col_strings.push(Col::is_rejected.to_string());
    map.insert(sso_observation.string(), col_strings);
}
