use csv::{Writer, WriterBuilder};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

/// Exports the graph data for visualization in Python.
pub fn export_graph_for_visualization(
    graph: &UnGraph<u32, ()>,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    // Check if the file exists, and create it if it does not.
    let path = Path::new(file_path);
    if !path.exists() {
        File::create(path)?;
    }

    // Create a new CSV writer that writes to the specified file path.
    let mut writer = WriterBuilder::new().from_path(file_path)?;

    // Iterate over all edges in the graph.
    for edge in graph.edge_references() {
        // Retrieve the source and target node indices of the current edge.
        let source = edge.source().index();
        let target = edge.target().index();

        // Write the source and target node indices to the CSV file as a new row.
        writer.write_record(&[source.to_string(), target.to_string()])?;
    }

    // Flush the writer to ensure all data is written to the file.
    writer.flush()?;
    Ok(())
}

/// Exports the centrality results to a CSV file.
pub fn export_centrality_to_csv(
    centrality: &HashMap<u32, f64>,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    // Check if the file exists, and create it if it does not.
    let path = Path::new(file_path);
    if !path.exists() {
        File::create(path)?;
    }
    let mut writer = Writer::from_path(file_path)?;
    writer.write_record(&["node", "centrality"])?;
    // Write the centrality scores to the CSV file.
    for (node, score) in centrality {
        writer.write_record(&[node.to_string(), score.to_string()])?;
    }
    writer.flush()?;
    Ok(())
}

/// Exports clustering data to a CSV file.
pub fn export_clustering_to_csv(
    clusters: &Vec<Vec<u32>>,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    // Check if the file exists, and create it if it does not.
    let path = Path::new(file_path);
    if !path.exists() {
        File::create(path)?;
    }

    let mut writer = Writer::from_path(file_path)?;
    writer.write_record(&["cluster_id", "node"])?;

    for (index, cluster) in clusters.iter().enumerate() {
        for node in cluster {
            writer.write_record(&[index.to_string(), node.to_string()])?;
        }
    }
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::UnGraph;
    use std::fs;
    use std::io::{self, BufReader, Read};

    // Helper function to check file content
    fn read_file_content(file_path: &str) -> io::Result<String> {
        let file = fs::File::open(file_path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[test] // Test for empty graph
    fn test_empty_graph_export_graph_for_visualization() {
        let graph: UnGraph<u32, ()> = UnGraph::new_undirected();
        let file_path = "temp_empty_graph.csv";
        export_graph_for_visualization(&graph, file_path).unwrap();
        let content = read_file_content(file_path).unwrap();
        assert!(
            content.is_empty(),
            "The file should be empty for an empty graph"
        );
        fs::remove_file(file_path).unwrap();
    }

    #[test] // Test for file creation
    fn test_file_creation_export_graph_for_visualization() {
        let graph: UnGraph<u32, ()> = UnGraph::new_undirected();
        let file_path = "temp_new_file.csv";
        export_graph_for_visualization(&graph, file_path).unwrap();
        assert!(
            fs::metadata(file_path).is_ok(),
            "The file should be created"
        );
        fs::remove_file(file_path).unwrap();
    }
}
