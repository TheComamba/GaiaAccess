// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct commanded_scan_law;

impl Schema for commanded_scan_law {
    fn string(&self) -> String {
        "commanded_scan_law".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    jd_time,
    bjd_fov1,
    bjd_fov2,
    obmt_time,
    ra_fov1,
    dec_fov1,
    heal_pix_fov1,
    scan_angle_fov1,
    ra_fov2,
    dec_fov2,
    heal_pix_fov2,
    scan_angle_fov2,
    solution_id,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::jd_time.to_string());
    col_strings.push(Col::bjd_fov1.to_string());
    col_strings.push(Col::bjd_fov2.to_string());
    col_strings.push(Col::obmt_time.to_string());
    col_strings.push(Col::ra_fov1.to_string());
    col_strings.push(Col::dec_fov1.to_string());
    col_strings.push(Col::heal_pix_fov1.to_string());
    col_strings.push(Col::scan_angle_fov1.to_string());
    col_strings.push(Col::ra_fov2.to_string());
    col_strings.push(Col::dec_fov2.to_string());
    col_strings.push(Col::heal_pix_fov2.to_string());
    col_strings.push(Col::scan_angle_fov2.to_string());
    col_strings.push(Col::solution_id.to_string());
    map.insert(commanded_scan_law.string(), col_strings);
}
