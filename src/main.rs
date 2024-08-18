use isomorphic_graph_creation::unrank;

fn main() {
    let all_combinations = unrank(&vec![5, 8, 0, 1], 2);
    println!("generated {} combinations", all_combinations.len());
}
