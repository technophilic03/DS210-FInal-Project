use csv::{ReaderBuilder, WriterBuilder};
// https://docs.rs/csv/latest/csv/struct.ReaderBuilder.html
use std::error::Error;
use std::fs::File;

use petgraph::graph::UnGraph;
// https://docs.rs/petgraph/0.6.4/petgraph/graph/type.UnGraph.html

/// Builds and preprocesses the graph data from the network and associations CSV files.
pub fn build_and_preprocess_graph(
    network_path: &str,
    associations_path: &str,
) -> Result<UnGraph<u32, ()>, Box<dyn Error>> {
    // Preprocess the associations CSV to make it suitable for graph construction.
    preprocess_associations(associations_path, "data/preprocessed_associations.csv")?;

    // Build the graph from the network CSV.
    let graph = load_network_data(network_path)?;
    Ok(graph)
}

/// Preprocesses the associations CSV file to make it suitable for graph construction.
fn preprocess_associations(
    input_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Specify the return type as Result<(), Box<dyn std::error::Error>>
    let mut rdr = ReaderBuilder::new() // Use ReaderBuilder to configure the CSV reader
        .has_headers(true)
        .from_path(input_path)?;
    let mut wtr = WriterBuilder::new().from_path(output_path)?; // Use WriterBuilder to configure the CSV writer

    for result in rdr.records() {
        // Use the records method to iterate over the CSV records
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

fn load_network_data(file_path: &str) -> Result<UnGraph<u32, ()>, Box<dyn std::error::Error>> {
    // Specify the return type as Result<UnGraph<u32, ()>, Box<dyn std::error::Error>>
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut graph = UnGraph::<u32, ()>::new_undirected(); // Create an undirected graph
    let mut index_map = std::collections::HashMap::new(); // Create a HashMap to store node indices

    for result in rdr.deserialize() {
        // Use the deserialize method to iterate over the CSV records
        let record: (u32, u32) = result?; // Deserialize the CSV record into a tuple of two u32 values
        let index1 = *index_map // Use the entry method to insert a new node index if it does not exist
            .entry(record.0)
            .or_insert_with(|| graph.add_node(record.0));
        let index2 = *index_map // Use the entry method to insert a new node index if it does not exist
            .entry(record.1)
            .or_insert_with(|| graph.add_node(record.1));
        graph.add_edge(index1, index2, ()); // Add an edge between the two nodes
    }
    Ok(graph)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    // Helper function to create a temp file with content
    fn temp_file_with_content(content: &str) -> String {
        let path = "temp_test_file.csv";
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        path.to_string()
    }

    #[test] // Test the preprocess_associations function
    fn test_empty_input_build_and_preprocess_graph() {
        let result = build_and_preprocess_graph("", "");
        assert!(result.is_err());
    }

    #[test] // Test the preprocess_associations function
    fn test_non_existent_file_preprocess_associations() {
        let result = preprocess_associations("non_existent.csv", "output.csv");
        assert!(result.is_err());
    }
}
