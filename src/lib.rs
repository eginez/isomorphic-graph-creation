use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Product;

use num::{self, PrimInt};
use num::{Integer, Unsigned};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn factorial<T>(n: T) -> T
where
    T: Integer + PrimInt + Product + Unsigned,
{
    if n.is_zero() {
        return T::one();
    }
    num::range_inclusive(T::one(), n).product()
}
pub fn binomial_coefficient<T>(n: T, r: T) -> T
where
    T: Integer + PrimInt + Unsigned + for<'a> Product<&'a T> + Hash
{
    let first_list : HashSet<T> = num::range_inclusive(r + T::one(), n).collect();
    let second_list : HashSet<T> = num::range_inclusive(T::one(), n - r).collect();
    let numerator = first_list.difference(&second_list);
    let denominator = second_list.difference(&first_list);
    numerator.product::<T>() / denominator.product::<T>()
}

pub fn unrank_combination_single(
    set: &Vec<u64>,
    mut subset_size: u64,
    mut rank: u64,
) -> Vec<u64> {
    let mut combination:Vec<u64> = vec![];
    let mut i: usize = 0;
    let n = set.len() as u64;
    loop {
        let c = binomial_coefficient(n - (i as u64 + 1), subset_size - 1);
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
    let num_combinations = binomial_coefficient(set.len() as u64, subset_size);
    (0..num_combinations)
        .into_iter()
        .map(|pos| unrank_combination_single(set, subset_size, pos))
        .collect()
}

pub fn unrank_parallel(set: &Vec<u64>, subset_size: u64) -> Vec<Vec<u64>> {
    let num_combinations = binomial_coefficient(set.len() as u64, subset_size);
    (0..num_combinations)
        .into_par_iter()
        .map(|pos| unrank_combination_single(set, subset_size, pos))
        .collect()
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, 1)]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 6)]
    fn test_factorial(#[case] number: u64, #[case] result: u64) {
        assert_eq!(factorial(number), result);
    }

    #[rstest]
    #[case(2, 1, 2)]
    #[case(3, 1, 3)]
    #[case(3, 2, 3)]
    #[case(2, 0, 1)]
    #[case(20, 5, 15504)]
    fn test_binomial_coefficient(#[case] n: u64, #[case] r: u64, #[case] result: u64) {
        assert_eq!(binomial_coefficient(n, r), result);
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
