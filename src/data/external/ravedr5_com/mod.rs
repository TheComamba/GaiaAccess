// This code is generated by generate_code.py, do not modify it manually
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct ravedr5_com;

impl Table for ravedr5_com {
    fn string(&self) -> String {
        "ravedr5_com".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    healpix32,
    cf00_0,
    cf00_1,
    cf00_2,
    cf00_3,
    cf00_4,
    cf00_5,
    cf00_6,
    cf00_7,
    cf00_8,
    cf00_9,
    cf01_0,
    cf01_1,
    cf01_2,
    cf01_3,
    cf01_4,
    cf01_5,
    cf01_6,
    cf01_7,
    cf01_8,
    cf01_9,
    cf02_0,
    cf02_1,
    cf02_2,
    cf02_3,
    cf02_4,
    cf02_5,
    cf02_6,
    cf02_7,
    cf02_8,
    cf02_9,
    cf03_0,
    cf03_1,
    cf03_2,
    cf03_3,
    cf03_4,
    cf03_5,
    cf03_6,
    cf03_7,
    cf03_8,
    cf03_9,
    cf04_0,
    cf04_1,
    cf04_2,
    cf04_3,
    cf04_4,
    cf04_5,
    cf04_6,
    cf04_7,
    cf04_8,
    cf04_9,
    cf05_0,
    cf05_1,
    cf05_2,
    cf05_3,
    cf05_4,
    cf05_5,
    cf05_6,
    cf05_7,
    cf05_8,
    cf05_9,
    cf06_0,
    cf06_1,
    cf06_2,
    cf06_3,
    cf06_4,
    cf06_5,
    cf06_6,
    cf06_7,
    cf06_8,
    cf06_9,
    cf07_0,
    cf07_1,
    cf07_2,
    cf07_3,
    cf07_4,
    cf07_5,
    cf07_6,
    cf07_7,
    cf07_8,
    cf07_9,
    cf08_0,
    cf08_1,
    cf08_2,
    cf08_3,
    cf08_4,
    cf08_5,
    cf08_6,
    cf08_7,
    cf08_8,
    cf08_9,
    cf09_0,
    cf09_1,
    cf09_2,
    cf09_3,
    cf09_4,
    cf09_5,
    cf09_6,
    cf09_7,
    cf09_8,
    cf09_9,
    cf10_0,
    cf10_1,
    cf10_2,
    cf10_3,
    cf10_4,
    cf10_5,
    cf10_6,
    cf10_7,
    cf10_8,
    cf10_9,
    cf11_0,
    cf11_1,
    cf11_2,
    cf11_3,
    cf11_4,
    cf11_5,
    cf11_6,
    cf11_7,
    cf11_8,
    cf11_9,
    cf12_0,
    cf12_1,
    cf12_2,
    cf12_3,
    cf12_4,
    cf12_5,
    cf12_6,
    cf12_7,
    cf12_8,
    cf12_9,
    cf13_0,
    cf13_1,
    cf13_2,
    cf13_3,
    cf13_4,
    cf13_5,
    cf13_6,
    cf13_7,
    cf13_8,
    cf13_9,
    cf14_0,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::healpix32.to_string());
    col_strings.push(Col::cf00_0.to_string());
    col_strings.push(Col::cf00_1.to_string());
    col_strings.push(Col::cf00_2.to_string());
    col_strings.push(Col::cf00_3.to_string());
    col_strings.push(Col::cf00_4.to_string());
    col_strings.push(Col::cf00_5.to_string());
    col_strings.push(Col::cf00_6.to_string());
    col_strings.push(Col::cf00_7.to_string());
    col_strings.push(Col::cf00_8.to_string());
    col_strings.push(Col::cf00_9.to_string());
    col_strings.push(Col::cf01_0.to_string());
    col_strings.push(Col::cf01_1.to_string());
    col_strings.push(Col::cf01_2.to_string());
    col_strings.push(Col::cf01_3.to_string());
    col_strings.push(Col::cf01_4.to_string());
    col_strings.push(Col::cf01_5.to_string());
    col_strings.push(Col::cf01_6.to_string());
    col_strings.push(Col::cf01_7.to_string());
    col_strings.push(Col::cf01_8.to_string());
    col_strings.push(Col::cf01_9.to_string());
    col_strings.push(Col::cf02_0.to_string());
    col_strings.push(Col::cf02_1.to_string());
    col_strings.push(Col::cf02_2.to_string());
    col_strings.push(Col::cf02_3.to_string());
    col_strings.push(Col::cf02_4.to_string());
    col_strings.push(Col::cf02_5.to_string());
    col_strings.push(Col::cf02_6.to_string());
    col_strings.push(Col::cf02_7.to_string());
    col_strings.push(Col::cf02_8.to_string());
    col_strings.push(Col::cf02_9.to_string());
    col_strings.push(Col::cf03_0.to_string());
    col_strings.push(Col::cf03_1.to_string());
    col_strings.push(Col::cf03_2.to_string());
    col_strings.push(Col::cf03_3.to_string());
    col_strings.push(Col::cf03_4.to_string());
    col_strings.push(Col::cf03_5.to_string());
    col_strings.push(Col::cf03_6.to_string());
    col_strings.push(Col::cf03_7.to_string());
    col_strings.push(Col::cf03_8.to_string());
    col_strings.push(Col::cf03_9.to_string());
    col_strings.push(Col::cf04_0.to_string());
    col_strings.push(Col::cf04_1.to_string());
    col_strings.push(Col::cf04_2.to_string());
    col_strings.push(Col::cf04_3.to_string());
    col_strings.push(Col::cf04_4.to_string());
    col_strings.push(Col::cf04_5.to_string());
    col_strings.push(Col::cf04_6.to_string());
    col_strings.push(Col::cf04_7.to_string());
    col_strings.push(Col::cf04_8.to_string());
    col_strings.push(Col::cf04_9.to_string());
    col_strings.push(Col::cf05_0.to_string());
    col_strings.push(Col::cf05_1.to_string());
    col_strings.push(Col::cf05_2.to_string());
    col_strings.push(Col::cf05_3.to_string());
    col_strings.push(Col::cf05_4.to_string());
    col_strings.push(Col::cf05_5.to_string());
    col_strings.push(Col::cf05_6.to_string());
    col_strings.push(Col::cf05_7.to_string());
    col_strings.push(Col::cf05_8.to_string());
    col_strings.push(Col::cf05_9.to_string());
    col_strings.push(Col::cf06_0.to_string());
    col_strings.push(Col::cf06_1.to_string());
    col_strings.push(Col::cf06_2.to_string());
    col_strings.push(Col::cf06_3.to_string());
    col_strings.push(Col::cf06_4.to_string());
    col_strings.push(Col::cf06_5.to_string());
    col_strings.push(Col::cf06_6.to_string());
    col_strings.push(Col::cf06_7.to_string());
    col_strings.push(Col::cf06_8.to_string());
    col_strings.push(Col::cf06_9.to_string());
    col_strings.push(Col::cf07_0.to_string());
    col_strings.push(Col::cf07_1.to_string());
    col_strings.push(Col::cf07_2.to_string());
    col_strings.push(Col::cf07_3.to_string());
    col_strings.push(Col::cf07_4.to_string());
    col_strings.push(Col::cf07_5.to_string());
    col_strings.push(Col::cf07_6.to_string());
    col_strings.push(Col::cf07_7.to_string());
    col_strings.push(Col::cf07_8.to_string());
    col_strings.push(Col::cf07_9.to_string());
    col_strings.push(Col::cf08_0.to_string());
    col_strings.push(Col::cf08_1.to_string());
    col_strings.push(Col::cf08_2.to_string());
    col_strings.push(Col::cf08_3.to_string());
    col_strings.push(Col::cf08_4.to_string());
    col_strings.push(Col::cf08_5.to_string());
    col_strings.push(Col::cf08_6.to_string());
    col_strings.push(Col::cf08_7.to_string());
    col_strings.push(Col::cf08_8.to_string());
    col_strings.push(Col::cf08_9.to_string());
    col_strings.push(Col::cf09_0.to_string());
    col_strings.push(Col::cf09_1.to_string());
    col_strings.push(Col::cf09_2.to_string());
    col_strings.push(Col::cf09_3.to_string());
    col_strings.push(Col::cf09_4.to_string());
    col_strings.push(Col::cf09_5.to_string());
    col_strings.push(Col::cf09_6.to_string());
    col_strings.push(Col::cf09_7.to_string());
    col_strings.push(Col::cf09_8.to_string());
    col_strings.push(Col::cf09_9.to_string());
    col_strings.push(Col::cf10_0.to_string());
    col_strings.push(Col::cf10_1.to_string());
    col_strings.push(Col::cf10_2.to_string());
    col_strings.push(Col::cf10_3.to_string());
    col_strings.push(Col::cf10_4.to_string());
    col_strings.push(Col::cf10_5.to_string());
    col_strings.push(Col::cf10_6.to_string());
    col_strings.push(Col::cf10_7.to_string());
    col_strings.push(Col::cf10_8.to_string());
    col_strings.push(Col::cf10_9.to_string());
    col_strings.push(Col::cf11_0.to_string());
    col_strings.push(Col::cf11_1.to_string());
    col_strings.push(Col::cf11_2.to_string());
    col_strings.push(Col::cf11_3.to_string());
    col_strings.push(Col::cf11_4.to_string());
    col_strings.push(Col::cf11_5.to_string());
    col_strings.push(Col::cf11_6.to_string());
    col_strings.push(Col::cf11_7.to_string());
    col_strings.push(Col::cf11_8.to_string());
    col_strings.push(Col::cf11_9.to_string());
    col_strings.push(Col::cf12_0.to_string());
    col_strings.push(Col::cf12_1.to_string());
    col_strings.push(Col::cf12_2.to_string());
    col_strings.push(Col::cf12_3.to_string());
    col_strings.push(Col::cf12_4.to_string());
    col_strings.push(Col::cf12_5.to_string());
    col_strings.push(Col::cf12_6.to_string());
    col_strings.push(Col::cf12_7.to_string());
    col_strings.push(Col::cf12_8.to_string());
    col_strings.push(Col::cf12_9.to_string());
    col_strings.push(Col::cf13_0.to_string());
    col_strings.push(Col::cf13_1.to_string());
    col_strings.push(Col::cf13_2.to_string());
    col_strings.push(Col::cf13_3.to_string());
    col_strings.push(Col::cf13_4.to_string());
    col_strings.push(Col::cf13_5.to_string());
    col_strings.push(Col::cf13_6.to_string());
    col_strings.push(Col::cf13_7.to_string());
    col_strings.push(Col::cf13_8.to_string());
    col_strings.push(Col::cf13_9.to_string());
    col_strings.push(Col::cf14_0.to_string());
    map.insert(ravedr5_com.string(), col_strings);
}
