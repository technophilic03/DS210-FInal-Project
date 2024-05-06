use petgraph::algo::{connected_components, dijkstra};
use petgraph::graph::UnGraph;
use petgraph::visit::{IntoNodeIdentifiers, NodeIndexable};
use std::collections::HashMap;
use std::convert::TryInto;

/// Computes the betweenness centrality for each node in the graph.
pub fn compute_centrality_measures(graph: &UnGraph<u32, ()>) -> HashMap<u32, f64> {
    let mut centrality = HashMap::new();
    for start in graph.node_identifiers() {
        let paths = dijkstra(graph, start, None, |_| 1); // Using 1 as the weight for each edge
        for (end, _) in &paths {
            if start != *end {
                let index: u32 = graph.to_index(*end).try_into().expect("Index conversion failed");
                *centrality.entry(index).or_insert(0.0) += 1.0;
            }
        }
    }

    // Normalize the centrality values
    let n = graph.node_count() as f64;
    for value in centrality.values_mut() {
        *value /= (n - 1.0) * (n - 2.0);
    }

    centrality
}

pub fn perform_clustering(graph: &UnGraph<u32, ()>) -> Vec<Vec<u32>> {
    // Compute connected components
    let components = connected_components(graph);
    let mut clusters: HashMap<usize, Vec<u32>> = HashMap::new();

    // Each node gets a component index assigned, the output is a vec of indices.
    let mut component_indices = vec![0; graph.node_count()];  // Pre-allocate space for component indices

    // Temporary mapping of node index to component
    let mut index = 0;
    while index < graph.node_count() {
        let _node = graph.from_index(index);
        component_indices[index] = components;
        index += 1;
    }

    // Assign nodes to their corresponding component clusters
    for (node_index, &component_index) in component_indices.iter().enumerate() {
        clusters.entry(component_index).or_default().push(node_index as u32);
    }

    // Collect all clusters into a vector
    clusters.into_values().collect()
}