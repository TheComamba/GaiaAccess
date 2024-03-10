use crate::schema::Schema;

pub struct JobUpload;

impl Schema for JobUpload {
    fn string(&self) -> String {
        "job_upload".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(JobUpload.string(), tables);
}