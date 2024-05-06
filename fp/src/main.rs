mod data_loader;
mod graph_analysis;
mod utils;

use std::error::Error;

/// Main function to run the graph analysis pipeline.
fn main() -> Result<(), Box<dyn Error>> {
    // Load and preprocess CSV data for graph construction.
    let graph = data_loader::build_and_preprocess_graph(
        "data/bio-pathways-network.csv",
        "data/bio-pathways-associations.csv",
    )?;

    // Perform centrality analysis on the graph.
    let centrality_results = graph_analysis::compute_centrality_measures(&graph);
    println!("Centrality Results: {:?}", centrality_results);

    // Perform graph clustering.
    let clusters = graph_analysis::perform_clustering(&graph);
    println!("Clusters: {:?}", clusters);

    // Export the results to CSV files.
    utils::export_centrality_to_csv(&centrality_results, "data/centrality.csv")?;
    utils::export_clustering_to_csv(&clusters, "data/clusters.csv")?;

    // Export graph data for visualization in Python.
    utils::export_graph_for_visualization(&graph, "data/graph_data.csv")?;

    Ok(())
}
