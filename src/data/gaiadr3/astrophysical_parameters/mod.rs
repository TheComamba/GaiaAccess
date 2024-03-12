
// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct astrophysical_parameters;

impl Schema for astrophysical_parameters {
    fn string(&self) -> String {
        "astrophysical_parameters".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
source_id,
classprob_dsc_combmod_quasar,
classprob_dsc_combmod_galaxy,
classprob_dsc_combmod_star,
classprob_dsc_combmod_whitedwarf,
classprob_dsc_combmod_binarystar,
classprob_dsc_specmod_quasar,
classprob_dsc_specmod_galaxy,
classprob_dsc_specmod_star,
classprob_dsc_specmod_whitedwarf,
classprob_dsc_specmod_binarystar,
classprob_dsc_allosmod_quasar,
classprob_dsc_allosmod_galaxy,
classprob_dsc_allosmod_star,
teff_gspphot,
teff_gspphot_lower,
teff_gspphot_upper,
logg_gspphot,
logg_gspphot_lower,
logg_gspphot_upper,
mh_gspphot,
mh_gspphot_lower,
mh_gspphot_upper,
distance_gspphot,
distance_gspphot_lower,
distance_gspphot_upper,
azero_gspphot,
azero_gspphot_lower,
azero_gspphot_upper,
ag_gspphot,
ag_gspphot_lower,
ag_gspphot_upper,
abp_gspphot,
abp_gspphot_lower,
abp_gspphot_upper,
arp_gspphot,
arp_gspphot_lower,
arp_gspphot_upper,
ebpminrp_gspphot,
ebpminrp_gspphot_lower,
ebpminrp_gspphot_upper,
mg_gspphot,
mg_gspphot_lower,
mg_gspphot_upper,
radius_gspphot,
radius_gspphot_lower,
radius_gspphot_upper,
logposterior_gspphot,
mcmcaccept_gspphot,
libname_gspphot,
teff_gspspec,
teff_gspspec_lower,
teff_gspspec_upper,
logg_gspspec,
logg_gspspec_lower,
logg_gspspec_upper,
mh_gspspec,
mh_gspspec_lower,
mh_gspspec_upper,
alphafe_gspspec,
alphafe_gspspec_lower,
alphafe_gspspec_upper,
fem_gspspec,
fem_gspspec_lower,
fem_gspspec_upper,
fem_gspspec_nlines,
fem_gspspec_linescatter,
sife_gspspec,
sife_gspspec_lower,
sife_gspspec_upper,
sife_gspspec_nlines,
sife_gspspec_linescatter,
cafe_gspspec,
cafe_gspspec_lower,
cafe_gspspec_upper,
cafe_gspspec_nlines,
cafe_gspspec_linescatter,
tife_gspspec,
tife_gspspec_lower,
tife_gspspec_upper,
tife_gspspec_nlines,
tife_gspspec_linescatter,
mgfe_gspspec,
mgfe_gspspec_lower,
mgfe_gspspec_upper,
mgfe_gspspec_nlines,
mgfe_gspspec_linescatter,
ndfe_gspspec,
ndfe_gspspec_lower,
ndfe_gspspec_upper,
ndfe_gspspec_nlines,
ndfe_gspspec_linescatter,
feiim_gspspec,
feiim_gspspec_lower,
feiim_gspspec_upper,
feiim_gspspec_nlines,
feiim_gspspec_linescatter,
sfe_gspspec,
sfe_gspspec_lower,
sfe_gspspec_upper,
sfe_gspspec_nlines,
sfe_gspspec_linescatter,
zrfe_gspspec,
zrfe_gspspec_lower,
zrfe_gspspec_upper,
zrfe_gspspec_nlines,
zrfe_gspspec_linescatter,
nfe_gspspec,
nfe_gspspec_lower,
nfe_gspspec_upper,
nfe_gspspec_nlines,
nfe_gspspec_linescatter,
crfe_gspspec,
crfe_gspspec_lower,
crfe_gspspec_upper,
crfe_gspspec_nlines,
crfe_gspspec_linescatter,
cefe_gspspec,
cefe_gspspec_lower,
cefe_gspspec_upper,
cefe_gspspec_nlines,
cefe_gspspec_linescatter,
nife_gspspec,
nife_gspspec_lower,
nife_gspspec_upper,
nife_gspspec_nlines,
nife_gspspec_linescatter,
cn0ew_gspspec,
cn0ew_gspspec_uncertainty,
cn0_gspspec_centralline,
cn0_gspspec_width,
dib_gspspec_lambda,
dib_gspspec_lambda_uncertainty,
dibew_gspspec,
dibew_gspspec_uncertainty,
dibewnoise_gspspec_uncertainty,
dibp0_gspspec,
dibp2_gspspec,
dibp2_gspspec_uncertainty,
dibqf_gspspec,
flags_gspspec,
logchisq_gspspec,
ew_espels_halpha,
ew_espels_halpha_uncertainty,
ew_espels_halpha_flag,
ew_espels_halpha_model,
classlabel_espels,
classlabel_espels_flag,
classprob_espels_wcstar,
classprob_espels_wnstar,
classprob_espels_bestar,
classprob_espels_ttauristar,
classprob_espels_herbigstar,
classprob_espels_dmestar,
classprob_espels_pne,
azero_esphs,
azero_esphs_uncertainty,
ag_esphs,
ag_esphs_uncertainty,
ebpminrp_esphs,
ebpminrp_esphs_uncertainty,
teff_esphs,
teff_esphs_uncertainty,
logg_esphs,
logg_esphs_uncertainty,
vsini_esphs,
vsini_esphs_uncertainty,
flags_esphs,
spectraltype_esphs,
activityindex_espcs,
activityindex_espcs_uncertainty,
activityindex_espcs_input,
teff_espucd,
teff_espucd_uncertainty,
flags_espucd,
radius_flame,
radius_flame_lower,
radius_flame_upper,
lum_flame,
lum_flame_lower,
lum_flame_upper,
mass_flame,
mass_flame_lower,
mass_flame_upper,
age_flame,
age_flame_lower,
age_flame_upper,
flags_flame,
evolstage_flame,
gravredshift_flame,
gravredshift_flame_lower,
gravredshift_flame_upper,
bc_flame,
mh_msc,
mh_msc_upper,
mh_msc_lower,
azero_msc,
azero_msc_upper,
azero_msc_lower,
distance_msc,
distance_msc_upper,
distance_msc_lower,
teff_msc1,
teff_msc1_upper,
teff_msc1_lower,
teff_msc2,
teff_msc2_upper,
teff_msc2_lower,
logg_msc1,
logg_msc1_upper,
logg_msc1_lower,
logg_msc2,
logg_msc2_upper,
logg_msc2_lower,
ag_msc,
ag_msc_upper,
ag_msc_lower,
logposterior_msc,
mcmcaccept_msc,
mcmcdrift_msc,
flags_msc,
neuron_oa_id,
neuron_oa_dist,
neuron_oa_dist_percentile_rank,
flags_oa,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    col_strings.push(Col::solution_id.to_string());
col_strings.push(Col::source_id.to_string());
col_strings.push(Col::classprob_dsc_combmod_quasar.to_string());
col_strings.push(Col::classprob_dsc_combmod_galaxy.to_string());
col_strings.push(Col::classprob_dsc_combmod_star.to_string());
col_strings.push(Col::classprob_dsc_combmod_whitedwarf.to_string());
col_strings.push(Col::classprob_dsc_combmod_binarystar.to_string());
col_strings.push(Col::classprob_dsc_specmod_quasar.to_string());
col_strings.push(Col::classprob_dsc_specmod_galaxy.to_string());
col_strings.push(Col::classprob_dsc_specmod_star.to_string());
col_strings.push(Col::classprob_dsc_specmod_whitedwarf.to_string());
col_strings.push(Col::classprob_dsc_specmod_binarystar.to_string());
col_strings.push(Col::classprob_dsc_allosmod_quasar.to_string());
col_strings.push(Col::classprob_dsc_allosmod_galaxy.to_string());
col_strings.push(Col::classprob_dsc_allosmod_star.to_string());
col_strings.push(Col::teff_gspphot.to_string());
col_strings.push(Col::teff_gspphot_lower.to_string());
col_strings.push(Col::teff_gspphot_upper.to_string());
col_strings.push(Col::logg_gspphot.to_string());
col_strings.push(Col::logg_gspphot_lower.to_string());
col_strings.push(Col::logg_gspphot_upper.to_string());
col_strings.push(Col::mh_gspphot.to_string());
col_strings.push(Col::mh_gspphot_lower.to_string());
col_strings.push(Col::mh_gspphot_upper.to_string());
col_strings.push(Col::distance_gspphot.to_string());
col_strings.push(Col::distance_gspphot_lower.to_string());
col_strings.push(Col::distance_gspphot_upper.to_string());
col_strings.push(Col::azero_gspphot.to_string());
col_strings.push(Col::azero_gspphot_lower.to_string());
col_strings.push(Col::azero_gspphot_upper.to_string());
col_strings.push(Col::ag_gspphot.to_string());
col_strings.push(Col::ag_gspphot_lower.to_string());
col_strings.push(Col::ag_gspphot_upper.to_string());
col_strings.push(Col::abp_gspphot.to_string());
col_strings.push(Col::abp_gspphot_lower.to_string());
col_strings.push(Col::abp_gspphot_upper.to_string());
col_strings.push(Col::arp_gspphot.to_string());
col_strings.push(Col::arp_gspphot_lower.to_string());
col_strings.push(Col::arp_gspphot_upper.to_string());
col_strings.push(Col::ebpminrp_gspphot.to_string());
col_strings.push(Col::ebpminrp_gspphot_lower.to_string());
col_strings.push(Col::ebpminrp_gspphot_upper.to_string());
col_strings.push(Col::mg_gspphot.to_string());
col_strings.push(Col::mg_gspphot_lower.to_string());
col_strings.push(Col::mg_gspphot_upper.to_string());
col_strings.push(Col::radius_gspphot.to_string());
col_strings.push(Col::radius_gspphot_lower.to_string());
col_strings.push(Col::radius_gspphot_upper.to_string());
col_strings.push(Col::logposterior_gspphot.to_string());
col_strings.push(Col::mcmcaccept_gspphot.to_string());
col_strings.push(Col::libname_gspphot.to_string());
col_strings.push(Col::teff_gspspec.to_string());
col_strings.push(Col::teff_gspspec_lower.to_string());
col_strings.push(Col::teff_gspspec_upper.to_string());
col_strings.push(Col::logg_gspspec.to_string());
col_strings.push(Col::logg_gspspec_lower.to_string());
col_strings.push(Col::logg_gspspec_upper.to_string());
col_strings.push(Col::mh_gspspec.to_string());
col_strings.push(Col::mh_gspspec_lower.to_string());
col_strings.push(Col::mh_gspspec_upper.to_string());
col_strings.push(Col::alphafe_gspspec.to_string());
col_strings.push(Col::alphafe_gspspec_lower.to_string());
col_strings.push(Col::alphafe_gspspec_upper.to_string());
col_strings.push(Col::fem_gspspec.to_string());
col_strings.push(Col::fem_gspspec_lower.to_string());
col_strings.push(Col::fem_gspspec_upper.to_string());
col_strings.push(Col::fem_gspspec_nlines.to_string());
col_strings.push(Col::fem_gspspec_linescatter.to_string());
col_strings.push(Col::sife_gspspec.to_string());
col_strings.push(Col::sife_gspspec_lower.to_string());
col_strings.push(Col::sife_gspspec_upper.to_string());
col_strings.push(Col::sife_gspspec_nlines.to_string());
col_strings.push(Col::sife_gspspec_linescatter.to_string());
col_strings.push(Col::cafe_gspspec.to_string());
col_strings.push(Col::cafe_gspspec_lower.to_string());
col_strings.push(Col::cafe_gspspec_upper.to_string());
col_strings.push(Col::cafe_gspspec_nlines.to_string());
col_strings.push(Col::cafe_gspspec_linescatter.to_string());
col_strings.push(Col::tife_gspspec.to_string());
col_strings.push(Col::tife_gspspec_lower.to_string());
col_strings.push(Col::tife_gspspec_upper.to_string());
col_strings.push(Col::tife_gspspec_nlines.to_string());
col_strings.push(Col::tife_gspspec_linescatter.to_string());
col_strings.push(Col::mgfe_gspspec.to_string());
col_strings.push(Col::mgfe_gspspec_lower.to_string());
col_strings.push(Col::mgfe_gspspec_upper.to_string());
col_strings.push(Col::mgfe_gspspec_nlines.to_string());
col_strings.push(Col::mgfe_gspspec_linescatter.to_string());
col_strings.push(Col::ndfe_gspspec.to_string());
col_strings.push(Col::ndfe_gspspec_lower.to_string());
col_strings.push(Col::ndfe_gspspec_upper.to_string());
col_strings.push(Col::ndfe_gspspec_nlines.to_string());
col_strings.push(Col::ndfe_gspspec_linescatter.to_string());
col_strings.push(Col::feiim_gspspec.to_string());
col_strings.push(Col::feiim_gspspec_lower.to_string());
col_strings.push(Col::feiim_gspspec_upper.to_string());
col_strings.push(Col::feiim_gspspec_nlines.to_string());
col_strings.push(Col::feiim_gspspec_linescatter.to_string());
col_strings.push(Col::sfe_gspspec.to_string());
col_strings.push(Col::sfe_gspspec_lower.to_string());
col_strings.push(Col::sfe_gspspec_upper.to_string());
col_strings.push(Col::sfe_gspspec_nlines.to_string());
col_strings.push(Col::sfe_gspspec_linescatter.to_string());
col_strings.push(Col::zrfe_gspspec.to_string());
col_strings.push(Col::zrfe_gspspec_lower.to_string());
col_strings.push(Col::zrfe_gspspec_upper.to_string());
col_strings.push(Col::zrfe_gspspec_nlines.to_string());
col_strings.push(Col::zrfe_gspspec_linescatter.to_string());
col_strings.push(Col::nfe_gspspec.to_string());
col_strings.push(Col::nfe_gspspec_lower.to_string());
col_strings.push(Col::nfe_gspspec_upper.to_string());
col_strings.push(Col::nfe_gspspec_nlines.to_string());
col_strings.push(Col::nfe_gspspec_linescatter.to_string());
col_strings.push(Col::crfe_gspspec.to_string());
col_strings.push(Col::crfe_gspspec_lower.to_string());
col_strings.push(Col::crfe_gspspec_upper.to_string());
col_strings.push(Col::crfe_gspspec_nlines.to_string());
col_strings.push(Col::crfe_gspspec_linescatter.to_string());
col_strings.push(Col::cefe_gspspec.to_string());
col_strings.push(Col::cefe_gspspec_lower.to_string());
col_strings.push(Col::cefe_gspspec_upper.to_string());
col_strings.push(Col::cefe_gspspec_nlines.to_string());
col_strings.push(Col::cefe_gspspec_linescatter.to_string());
col_strings.push(Col::nife_gspspec.to_string());
col_strings.push(Col::nife_gspspec_lower.to_string());
col_strings.push(Col::nife_gspspec_upper.to_string());
col_strings.push(Col::nife_gspspec_nlines.to_string());
col_strings.push(Col::nife_gspspec_linescatter.to_string());
col_strings.push(Col::cn0ew_gspspec.to_string());
col_strings.push(Col::cn0ew_gspspec_uncertainty.to_string());
col_strings.push(Col::cn0_gspspec_centralline.to_string());
col_strings.push(Col::cn0_gspspec_width.to_string());
col_strings.push(Col::dib_gspspec_lambda.to_string());
col_strings.push(Col::dib_gspspec_lambda_uncertainty.to_string());
col_strings.push(Col::dibew_gspspec.to_string());
col_strings.push(Col::dibew_gspspec_uncertainty.to_string());
col_strings.push(Col::dibewnoise_gspspec_uncertainty.to_string());
col_strings.push(Col::dibp0_gspspec.to_string());
col_strings.push(Col::dibp2_gspspec.to_string());
col_strings.push(Col::dibp2_gspspec_uncertainty.to_string());
col_strings.push(Col::dibqf_gspspec.to_string());
col_strings.push(Col::flags_gspspec.to_string());
col_strings.push(Col::logchisq_gspspec.to_string());
col_strings.push(Col::ew_espels_halpha.to_string());
col_strings.push(Col::ew_espels_halpha_uncertainty.to_string());
col_strings.push(Col::ew_espels_halpha_flag.to_string());
col_strings.push(Col::ew_espels_halpha_model.to_string());
col_strings.push(Col::classlabel_espels.to_string());
col_strings.push(Col::classlabel_espels_flag.to_string());
col_strings.push(Col::classprob_espels_wcstar.to_string());
col_strings.push(Col::classprob_espels_wnstar.to_string());
col_strings.push(Col::classprob_espels_bestar.to_string());
col_strings.push(Col::classprob_espels_ttauristar.to_string());
col_strings.push(Col::classprob_espels_herbigstar.to_string());
col_strings.push(Col::classprob_espels_dmestar.to_string());
col_strings.push(Col::classprob_espels_pne.to_string());
col_strings.push(Col::azero_esphs.to_string());
col_strings.push(Col::azero_esphs_uncertainty.to_string());
col_strings.push(Col::ag_esphs.to_string());
col_strings.push(Col::ag_esphs_uncertainty.to_string());
col_strings.push(Col::ebpminrp_esphs.to_string());
col_strings.push(Col::ebpminrp_esphs_uncertainty.to_string());
col_strings.push(Col::teff_esphs.to_string());
col_strings.push(Col::teff_esphs_uncertainty.to_string());
col_strings.push(Col::logg_esphs.to_string());
col_strings.push(Col::logg_esphs_uncertainty.to_string());
col_strings.push(Col::vsini_esphs.to_string());
col_strings.push(Col::vsini_esphs_uncertainty.to_string());
col_strings.push(Col::flags_esphs.to_string());
col_strings.push(Col::spectraltype_esphs.to_string());
col_strings.push(Col::activityindex_espcs.to_string());
col_strings.push(Col::activityindex_espcs_uncertainty.to_string());
col_strings.push(Col::activityindex_espcs_input.to_string());
col_strings.push(Col::teff_espucd.to_string());
col_strings.push(Col::teff_espucd_uncertainty.to_string());
col_strings.push(Col::flags_espucd.to_string());
col_strings.push(Col::radius_flame.to_string());
col_strings.push(Col::radius_flame_lower.to_string());
col_strings.push(Col::radius_flame_upper.to_string());
col_strings.push(Col::lum_flame.to_string());
col_strings.push(Col::lum_flame_lower.to_string());
col_strings.push(Col::lum_flame_upper.to_string());
col_strings.push(Col::mass_flame.to_string());
col_strings.push(Col::mass_flame_lower.to_string());
col_strings.push(Col::mass_flame_upper.to_string());
col_strings.push(Col::age_flame.to_string());
col_strings.push(Col::age_flame_lower.to_string());
col_strings.push(Col::age_flame_upper.to_string());
col_strings.push(Col::flags_flame.to_string());
col_strings.push(Col::evolstage_flame.to_string());
col_strings.push(Col::gravredshift_flame.to_string());
col_strings.push(Col::gravredshift_flame_lower.to_string());
col_strings.push(Col::gravredshift_flame_upper.to_string());
col_strings.push(Col::bc_flame.to_string());
col_strings.push(Col::mh_msc.to_string());
col_strings.push(Col::mh_msc_upper.to_string());
col_strings.push(Col::mh_msc_lower.to_string());
col_strings.push(Col::azero_msc.to_string());
col_strings.push(Col::azero_msc_upper.to_string());
col_strings.push(Col::azero_msc_lower.to_string());
col_strings.push(Col::distance_msc.to_string());
col_strings.push(Col::distance_msc_upper.to_string());
col_strings.push(Col::distance_msc_lower.to_string());
col_strings.push(Col::teff_msc1.to_string());
col_strings.push(Col::teff_msc1_upper.to_string());
col_strings.push(Col::teff_msc1_lower.to_string());
col_strings.push(Col::teff_msc2.to_string());
col_strings.push(Col::teff_msc2_upper.to_string());
col_strings.push(Col::teff_msc2_lower.to_string());
col_strings.push(Col::logg_msc1.to_string());
col_strings.push(Col::logg_msc1_upper.to_string());
col_strings.push(Col::logg_msc1_lower.to_string());
col_strings.push(Col::logg_msc2.to_string());
col_strings.push(Col::logg_msc2_upper.to_string());
col_strings.push(Col::logg_msc2_lower.to_string());
col_strings.push(Col::ag_msc.to_string());
col_strings.push(Col::ag_msc_upper.to_string());
col_strings.push(Col::ag_msc_lower.to_string());
col_strings.push(Col::logposterior_msc.to_string());
col_strings.push(Col::mcmcaccept_msc.to_string());
col_strings.push(Col::mcmcdrift_msc.to_string());
col_strings.push(Col::flags_msc.to_string());
col_strings.push(Col::neuron_oa_id.to_string());
col_strings.push(Col::neuron_oa_dist.to_string());
col_strings.push(Col::neuron_oa_dist_percentile_rank.to_string());
col_strings.push(Col::flags_oa.to_string());
    map.insert(astrophysical_parameters.string(), col_strings);
}
