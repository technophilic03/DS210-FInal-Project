use petgraph::algo::{connected_components, dijkstra};
// https://docs.rs/petgraph/0.6.4/petgraph/algo/index.html
use petgraph::graph::UnGraph;
use petgraph::visit::{IntoNodeIdentifiers, NodeIndexable};
// https://docs.rs/petgraph/0.6.4/petgraph/visit/trait.IntoNodeIdentifiers.html
// https://docs.rs/petgraph/0.6.4/petgraph/visit/trait.NodeIndexable.html
use std::collections::HashMap;

use std::convert::TryInto;
// https://doc.rust-lang.org/std/convert/trait.TryInto.html
// added from error suggestion

/// Computes the betweenness centrality for each node in the graph.
pub fn compute_centrality_measures(graph: &UnGraph<u32, ()>) -> HashMap<u32, f64> {
    let mut centrality = HashMap::new(); // Create a new HashMap to store the centrality values
    for start in graph.node_identifiers() {
        // Iterate over all nodes in the graph
        let paths = dijkstra(graph, start, None, |_| 1); // Using 1 as the weight for each edge
        for (end, _) in &paths {
            // Iterate over all nodes in the graph
            if start != *end {
                // Skip the current node
                let index: u32 = graph // Convert the node index to a u32 value
                    .to_index(*end)
                    .try_into()
                    .expect("Index conversion failed");
                *centrality.entry(index).or_insert(0.0) += 1.0; // Increment the centrality value
            }
        }
    }

    // Normalize the centrality values
    let n = graph.node_count() as f64;
    for value in centrality.values_mut() {
        // Iterate over all centrality values
        *value /= (n - 1.0) * (n - 2.0);
    }

    centrality
}

/// Performs clustering on the graph using the connected components algorithm.
pub fn perform_clustering(graph: &UnGraph<u32, ()>) -> Vec<Vec<u32>> {
    // Compute connected components
    let components = connected_components(graph);
    let mut clusters: HashMap<usize, Vec<u32>> = HashMap::new();

    // Each node gets a component index assigned, the output is a vec of indices.
    let mut component_indices = vec![0; graph.node_count()];

    // Temporary mapping of node index to component
    let mut index = 0;
    while index < graph.node_count() {
        let _node = graph.from_index(index); // Get the node index
        component_indices[index] = components;
        index += 1;
    }

    // Assign nodes to their corresponding component clusters
    for (node_index, &component_index) in component_indices.iter().enumerate() {
        clusters // Add the node to the corresponding cluster
            .entry(component_index)
            .or_default()
            .push(node_index as u32);
    }

    // Collect all clusters into a vector
    clusters.into_values().collect()
}
