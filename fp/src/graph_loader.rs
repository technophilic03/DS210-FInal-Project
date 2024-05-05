use petgraph::graph::UnGraph;
use csv::Reader;
use std::collections::HashMap;

pub fn load_and_construct_graph(file_path: &str) -> Result<UnGraph<u32, ()>, Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path(file_path)?;
    let mut graph = UnGraph::<u32, ()>::new_undirected();
    let mut index_map = HashMap::new();

    for result in reader.deserialize() {
        let record: (u32, u32) = result?;
        let index1 = *index_map.entry(record.0).or_insert_with(|| graph.add_node(record.0));
        let index2 = *index_map.entry(record.1).or_insert_with(|| graph.add_node(record.1));
        graph.add_edge(index1, index2, ());
    }
}
