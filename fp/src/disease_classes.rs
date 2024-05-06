use std::collections::HashMap;
use csv::ReaderBuilder;
use std::error::Error;

pub fn load_disease_classes(file_path: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut disease_classes = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let disease_id = record.get(0).unwrap().to_string();
        let disease_class = record.get(2).unwrap().to_string();
        disease_classes.insert(disease_id, disease_class);
    }

    Ok(disease_classes)
}
