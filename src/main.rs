use isomorphic_graph_creation::{
    create_random_graph, dot_graph, generate_subgraph_parallel, generate_subgraph_single, unrank,
};
use rustworkx_core::petgraph::dot::{Config, Dot};

fn main() {
    //let all_combinations = unrank(&vec![5, 8, 0, 1], 2);
    let graph_size = 10;
    let subgraph_size = 5;
    let rank = 1;
    let graph = create_random_graph(graph_size, Some(10));
    let subgraphs = generate_subgraph_parallel(&graph, subgraph_size, 5);
    dot_graph(
        &graph,
        &[Config::EdgeNoLabel, Config::NodeIndexLabel],
        "graph1",
    );
    for (index, subgraph) in subgraphs.iter().enumerate() {
        dot_graph(
            &subgraph,
            &[Config::EdgeNoLabel, Config::NodeIndexLabel],
            &format!("subgraph-{}", index).to_string(),
        );
    }
}
