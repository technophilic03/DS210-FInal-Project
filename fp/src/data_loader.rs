// Module to handle data loading and preprocessing.

use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use std::fs::File;

use petgraph::graph::{UnGraph};


pub fn build_and_preprocess_graph(network_path: &str, associations_path: &str) -> Result<UnGraph<u32, ()>, Box<dyn Error>> {
    // Preprocess the associations CSV to make it suitable for graph construction.
    preprocess_associations(associations_path, "data/preprocessed_associations.csv")?;

    // Build the graph from the network CSV.
    let graph = load_network_data(network_path)?;
    Ok(graph)
}

fn preprocess_associations(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(input_path)?;
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
    Ok(()) // Explicitly return Ok to indicate success
}



fn load_network_data(file_path: &str) -> Result<UnGraph<u32, ()>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut graph = UnGraph::<u32, ()>::new_undirected();
    let mut index_map = std::collections::HashMap::new();

    for result in rdr.deserialize() {
        let record: (u32, u32) = result?;
        let index1 = *index_map.entry(record.0).or_insert_with(|| graph.add_node(record.0));
        let index2 = *index_map.entry(record.1).or_insert_with(|| graph.add_node(record.1));
        graph.add_edge(index1, index2, ());
    }
    Ok(graph) // Ensure to return the graph in a Result
}

