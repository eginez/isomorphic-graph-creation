use isomorphic_graph_creation::{create_random_graph, dot_graph, unrank};
use rustworkx_core::petgraph::dot::{Config, Dot};

fn main() {
    //let all_combinations = unrank(&vec![5, 8, 0, 1], 2);
    for i in 0..10 {
        let graph = create_random_graph(4, None);
        if i == 0 {
            dot_graph(
                &graph,
                &[Config::EdgeNoLabel, Config::NodeIndexLabel],
                "graph1",
            );
        }
        println!("generated {:?}", graph);
    }
}
