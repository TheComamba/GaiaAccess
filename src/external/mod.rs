use crate::schema::Schema;

pub struct External;

impl Schema for External {
    fn string(&self) -> String {
        "external".to_string()
    }
}

#[cfg(any(apassdr9, test))]
pub mod apassdr9;
#[cfg(any(catwise2020, test))]
pub mod catwise2020;
#[cfg(any(gaia_eso_survey, test))]
pub mod gaia_eso_survey;
#[cfg(any(gaiadr2_astrophysical_parameters, test))]
pub mod gaiadr2_astrophysical_parameters;
#[cfg(any(gaiadr2_geometric_distance, test))]
pub mod gaiadr2_geometric_distance;
#[cfg(any(gaiaedr3_distance, test))]
pub mod gaiaedr3_distance;
#[cfg(any(gaiaedr3_gcns_main_1, test))]
pub mod gaiaedr3_gcns_main_1;
#[cfg(any(gaiaedr3_gcns_rejected_1, test))]
pub mod gaiaedr3_gcns_rejected_1;
#[cfg(any(gaiaedr3_spurious, test))]
pub mod gaiaedr3_spurious;
#[cfg(any(galex_ais, test))]
pub mod galex_ais;
#[cfg(any(ravedr5_com, test))]
pub mod ravedr5_com;
#[cfg(any(ravedr5_dr5, test))]
pub mod ravedr5_dr5;
#[cfg(any(ravedr5_gra, test))]
pub mod ravedr5_gra;
#[cfg(any(ravedr5_on, test))]
pub mod ravedr5_on;
#[cfg(any(ravedr6, test))]
pub mod ravedr6;
#[cfg(any(sdssdr13_photoprimary, test))]
pub mod sdssdr13_photoprimary;
#[cfg(any(skymapperdr1_master, test))]
pub mod skymapperdr1_master;
#[cfg(any(skymapperdr2_master, test))]
pub mod skymapperdr2_master;
#[cfg(any(tmass_xsc, test))]
pub mod tmass_xsc;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    apassdr9::collect_known(&mut tables);
    catwise2020::collect_known(&mut tables);
    gaia_eso_survey::collect_known(&mut tables);
    gaiadr2_astrophysical_parameters::collect_known(&mut tables);
    gaiadr2_geometric_distance::collect_known(&mut tables);
    gaiaedr3_distance::collect_known(&mut tables);
    gaiaedr3_gcns_main_1::collect_known(&mut tables);
    gaiaedr3_gcns_rejected_1::collect_known(&mut tables);
    gaiaedr3_spurious::collect_known(&mut tables);
    galex_ais::collect_known(&mut tables);
    ravedr5_com::collect_known(&mut tables);
    ravedr5_dr5::collect_known(&mut tables);
    ravedr5_gra::collect_known(&mut tables);
    ravedr5_on::collect_known(&mut tables);
    ravedr6::collect_known(&mut tables);
    sdssdr13_photoprimary::collect_known(&mut tables);
    skymapperdr1_master::collect_known(&mut tables);
    skymapperdr2_master::collect_known(&mut tables);
    tmass_xsc::collect_known(&mut tables);
    map.insert(External.string(), tables);
}