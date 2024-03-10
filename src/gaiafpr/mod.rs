use crate::schema::Schema;

pub struct Gaiafpr;

impl Schema for Gaiafpr {
    fn string(&self) -> String {
        "gaiafpr".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(Gaiafpr.string(), tables);
}
