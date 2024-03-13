// This code is generated by generate_code.py, do not modify it manually
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct gaiafpr;

impl Schema for gaiafpr {
    fn string(&self) -> String {
        "gaiafpr".to_string()
    }
}

#[cfg(any(feature = "gaiafpr_crowded_field_source", test))]
pub mod crowded_field_source;
#[cfg(any(feature = "gaiafpr_interstellar_medium_params", test))]
pub mod interstellar_medium_params;
#[cfg(any(feature = "gaiafpr_interstellar_medium_spectra", test))]
pub mod interstellar_medium_spectra;
#[cfg(any(feature = "gaiafpr_lens_candidates", test))]
pub mod lens_candidates;
#[cfg(any(feature = "gaiafpr_lens_catalogue_name", test))]
pub mod lens_catalogue_name;
#[cfg(any(feature = "gaiafpr_lens_observation", test))]
pub mod lens_observation;
#[cfg(any(feature = "gaiafpr_lens_outlier", test))]
pub mod lens_outlier;
#[cfg(any(feature = "gaiafpr_sso_observation", test))]
pub mod sso_observation;
#[cfg(any(feature = "gaiafpr_sso_source", test))]
pub mod sso_source;
#[cfg(any(feature = "gaiafpr_vari_epoch_radial_velocity", test))]
pub mod vari_epoch_radial_velocity;
#[cfg(any(feature = "gaiafpr_vari_long_period_variable", test))]
pub mod vari_long_period_variable;
#[cfg(any(feature = "gaiafpr_vari_rad_vel_statistics", test))]
pub mod vari_rad_vel_statistics;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    // Some tables do not have any columns. Disabling compiler warnings for these cases
    #[allow(unused_mut)]
    let mut tables = std::collections::HashMap::new();
    crowded_field_source::collect_known(&mut tables);
    lens_candidates::collect_known(&mut tables);
    lens_catalogue_name::collect_known(&mut tables);
    lens_observation::collect_known(&mut tables);
    lens_outlier::collect_known(&mut tables);
    sso_observation::collect_known(&mut tables);
    sso_source::collect_known(&mut tables);
    interstellar_medium_params::collect_known(&mut tables);
    interstellar_medium_spectra::collect_known(&mut tables);
    vari_epoch_radial_velocity::collect_known(&mut tables);
    vari_long_period_variable::collect_known(&mut tables);
    vari_rad_vel_statistics::collect_known(&mut tables);
    map.insert(gaiafpr.string(), tables);
}
