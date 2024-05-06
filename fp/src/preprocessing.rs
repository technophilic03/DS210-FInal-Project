use csv::{ReaderBuilder, WriterBuilder, Trim};
use std::error::Error;

// this module handles the preprocessing of association data to make it suitable for graph-based analysis.

pub fn preprocess_associations(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).trim(Trim::All).from_path(input_path)?;
    let mut wtr = WriterBuilder::new().from_path(output_path)?;

    for result in rdr.records() {
        let record = result?;
        let disease_id = record.get(0).unwrap();
        let disease_name = record.get(1).unwrap();
        let genes = record.get(2).unwrap().split(',');

        for gene in genes {
            wtr.write_record(&[disease_id, disease_name, gene.trim()])?;
        }
    }
    wtr.flush()?;
    Ok(())
}
