use petgraph::graph::UnGraph;
use csv::Reader;
use std::collections::HashMap;
use std::fs::File;

// Loads data from a CSV file and constructs an undirected graph.

pub fn load_and_construct_graph(file_path: &str) -> Result<UnGraph<u32, ()>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(file_path)?;
	// Create a new CSV reader from the file
    let mut graph = UnGraph::<u32, ()>::new_undirected();
	// Create a hashmap to keep track of indices of nodes in the graph
    let mut index_map = HashMap::new();

	// Iterate through each record in the CSV reader
    for result in reader.deserialize() {
        let record: (u32, u32) = result?;
		// create a node index for the first integer in the tuple, and add it to the graph
        let index1 = *index_map.entry(record.0).or_insert_with(|| graph.add_node(record.0));
		// create a node index for the second integer in the tuple, and add it to the graph
        let index2 = *index_map.entry(record.1).or_insert_with(|| graph.add_node(record.1));
        graph.add_edge(index1, index2, ());
    }
}