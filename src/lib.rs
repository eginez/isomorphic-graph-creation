use num::{self, One};
use num::{Integer, Unsigned};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rustworkx_core::generators::gnp_random_graph;
use rustworkx_core::petgraph::algo::is_isomorphic;
use rustworkx_core::petgraph::dot::{Config, Dot};
use rustworkx_core::petgraph::graph::UnGraph;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::iter::Product;
use std::ops::Add;
use std::process::Command;

fn _build_hashset<R: Hash + Eq + PartialOrd + Add<Output = R> + One + Clone>(
    start: R,
    end: R,
) -> HashSet<R> {
    let mut result = HashSet::new();
    let mut current = start.clone();
    while current <= end {
        result.insert(current.clone());
        current = current + R::one();
    }
    result
}
pub fn binomial_coefficient<T>(n: &T, r: &T) -> T
where
    T: Integer + Unsigned + for<'a> Product<&'a T> + Hash + Clone,
{
    let start = r.clone() + T::one();
    let first_list = _build_hashset(start, n.clone());
    let second_list = _build_hashset(T::one(), n.clone() - r.clone());
    let numerator = first_list.difference(&second_list);
    let denominator = second_list.difference(&first_list);
    numerator.product::<T>() / denominator.product::<T>()
}

pub fn unrank_combination_single(set: &Vec<u64>, mut subset_size: u64, mut rank: u64) -> Vec<u64> {
    let mut combination: Vec<u64> = vec![];
    let mut i: usize = 0;
    let n = set.len() as u64;
    loop {
        let c = binomial_coefficient(&(n - (i as u64 + 1)), &(subset_size - 1));
        if rank < c {
            combination.push(set[i]);
            subset_size -= 1;
        } else {
            // When the rank is greater or equal than the combinations, it means
            // that all combinations starting at set[i] are ranked before the requested *rank*
            // and we have to skip them.
            rank -= c;
        }
        i += 1;
        if subset_size == 0 {
            break;
        }
    }
    combination
}
pub fn unrank(set: &Vec<u64>, subset_size: u64) -> Vec<Vec<u64>> {
    let num_combinations = binomial_coefficient(&(set.len() as u64), &subset_size);
    (0..num_combinations)
        .into_iter()
        .map(|pos| unrank_combination_single(set, subset_size, pos))
        .collect()
}

pub fn unrank_parallel(set: &Vec<u64>, subset_size: u64) -> Vec<Vec<u64>> {
    let num_combinations = binomial_coefficient(&(set.len() as u64), &subset_size);
    (0..num_combinations)
        .into_par_iter()
        .map(|pos| unrank_combination_single(set, subset_size, pos))
        .collect()
}

pub fn create_random_graph(node_count: usize, seed: Option<u64>) -> UnGraph<(), ()> {
    gnp_random_graph(node_count, 0.5, seed, || (), || ()).unwrap()
}

pub fn dot_graph(graph: &UnGraph<(), ()>, config: &[Config], filename: &str) {
    let dot_repr = format!("{:?}", Dot::with_config(graph, config));
    let dot_filename = format!("{}.dot", filename);
    let png_filename = format!("{}.png", filename);
    std::fs::write(dot_filename.clone(), dot_repr).unwrap();
    let command = "dot";
    let arguments = ["-Tpng", &dot_filename, "-o", &png_filename];

    let output = Command::new(command)
        .args(&arguments)
        .output()
        .expect("Should have worked, do you have graphviz installed?");

    if !output.status.success() {
        print!("Failed to write png file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::{BigUint, FromPrimitive};
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use rstest::*;

    #[test]
    fn test_random_graph() {
        let g = create_random_graph(5, None);
        let g2 = create_random_graph(4, None);

        assert!(is_isomorphic(&g, &g2))
    }

    #[rstest]
    #[case(2, 1, 2)]
    #[case(3, 1, 3)]
    #[case(3, 2, 3)]
    #[case(2, 0, 1)]
    #[case(20, 5, 15504)]
    #[case(40, 2, 780)]
    fn test_binomial_coefficient(#[case] n: u64, #[case] r: u64, #[case] result: u64) {
        assert_eq!(binomial_coefficient(&n, &r), result);
    }

    #[rstest]
    #[case(100, 10, 17310309456440)]
    fn test_binomial_coefficient_bigint(#[case] n: u32, #[case] r: u32, #[case] result: u64) {
        let n_big = BigUint::from(n);
        let r_big = BigUint::from(r);
        assert_eq!(binomial_coefficient(&n_big, &r_big), BigUint::from(result));
    }

    #[rstest]
    #[case(vec![5,8,0], 2, 0, vec![5, 8])]
    #[case(vec![5,8,0], 2, 1, vec![5,0])]
    #[case(vec![5,8,0], 2, 2, vec![8, 0])]
    fn test_rank(
        #[case] set: Vec<u64>,
        #[case] subset_size: u64,
        #[case] r: u64,
        #[case] res: Vec<u64>,
    ) {
        assert_eq!(unrank_combination_single(&set, subset_size, r), res);
    }

    #[test]
    fn test_rank_complete() {
        let input = vec![5, 8, 0, 1];
        let all_results: Vec<Vec<u64>> = unrank(&input, 2);
        assert_eq!(all_results[0], vec![5, 8]);
        assert_eq!(all_results[1], vec![5, 0]);
        assert_eq!(all_results[2], vec![5, 1]);
        assert_eq!(all_results[3], vec![8, 0]);
        assert_eq!(all_results[4], vec![8, 1]);
        assert_eq!(all_results[5], vec![0, 1]);
    }
    #[test]
    fn test_rank_complete_parallel() {
        let input = vec![5, 8, 0, 1];
        let all_results: Vec<Vec<u64>> = unrank_parallel(&input, 2);
        assert_eq!(all_results[0], vec![5, 8]);
        assert_eq!(all_results[1], vec![5, 0]);
        assert_eq!(all_results[2], vec![5, 1]);
        assert_eq!(all_results[3], vec![8, 0]);
        assert_eq!(all_results[4], vec![8, 1]);
        assert_eq!(all_results[5], vec![0, 1]);
    }
    fn _create_random_input(size: u32) -> Vec<u64> {
        let mut rng = StdRng::seed_from_u64(32);
        (0..size)
            .into_iter()
            .map(|_| rng.gen_range(0..100))
            .collect()
    }

    #[test]
    fn test_rank_big() {
        let input = _create_random_input(40);
        let all_results: Vec<Vec<u64>> = unrank_parallel(&input, 2);
        assert!(all_results.len() > 0)
    }
}
