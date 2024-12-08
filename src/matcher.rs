use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use std::ops::Sub;

use itertools::Itertools;
use num::Zero;

pub fn match_charges<O, C>(orders: &[O], charges: &mut [C]) -> BinaryHeap<Vec<(O, Vec<C>)>>
where
    O: Clone + Ord + PartialOrd<C> + Sub<C, Output = O> + Zero,
    C: Clone + Eq + Hash + Ord,
{
    if charges.is_empty() {
        return BinaryHeap::new();
    }

    charges.sort();
    orders
        .into_iter()
        .map(|o| {
            // get all the possible exact fits for each order
            exhaustive(o.clone(), charges)
                .into_iter()
                // map them into pairs with their order
                .map(|m| (o.clone(), m))
                // the HashMap::Map here isn't Clone so we can't use `multi_cartesian_product`
                // on it.  but if we collect it into a vec and use that iterable, it'll work.
                .collect::<Vec<(O, Vec<C>)>>()
                .into_iter()
        })
        // now, we've got a vec of vecs where each vec is the order-match pairs grouped by the same
        // order.  if we take the n-dimensional product of that, we have potential candidates for
        // this solution.
        .multi_cartesian_product()
        // then, we need to filter those candidates down to those that don't use the same charge
        // for multiple orders
        .filter(|candidate: &Vec<(O, Vec<C>)>| {
            charges
                == &candidate
                    .iter()
                    .map(|(_, c)| c)
                    .fold(BinaryHeap::new(), |mut h, cs| {
                        let mut bhcs = cs.into_iter().map(C::clone).collect();
                        h.append(&mut bhcs);
                        h
                    })
                    .into_sorted_vec()
        })
        .collect()
}

fn exhaustive<O, C>(order: O, charges: &[C]) -> HashSet<Vec<C>>
where
    O: Clone + Zero + Sub<C, Output = O> + PartialOrd<C>,
    C: Clone + Eq + Hash + Ord,
{
    charges
        .to_owned()
        .into_iter()
        .powerset()
        .filter_map(|cs| knapsack(order.clone(), &cs))
        .map(BinaryHeap::into_sorted_vec)
        .collect()
}

fn knapsack<O, C>(order: O, charges: &[C]) -> Option<BinaryHeap<C>>
where
    O: Zero + Sub<C, Output = O> + PartialOrd<C>,
    C: Clone + Eq + Ord,
{
    if order.is_zero() {
        return Some(BinaryHeap::new());
    }
    if charges.is_empty() {
        return None;
    }

    let c = charges[0].clone();
    if order < c {
        return knapsack(order, &charges[1..]);
    }

    knapsack(order - c.clone(), &charges[1..]).map(|mut r| {
        r.push(c);
        r
    })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case([], [], vec![])]
    #[case([], [1], vec![])]
    #[case([2], [1], vec![])]
    #[case([1], [1], vec![vec![(1, vec![1])]])]
    #[case([2], [1, 1], vec![vec![(2, vec![1, 1])]])]
    #[case([1, 2], [1, 2], vec![vec![(1, vec![1]), (2, vec![2])]])]
    #[case([2, 3], [1, 1, 1, 2], vec![
        vec![(2, vec![1, 1]), (3, vec![1, 2])],
        vec![(2, vec![2]), (3, vec![1, 1, 1])],
    ])]
    fn match_charges_to_orders(
        #[case] orders: impl AsRef<[u32]>,
        #[case] mut charges: impl AsMut<[u32]>,
        #[case] matched: Vec<Vec<(u32, Vec<u32>)>>,
    ) {
        assert_eq!(
            match_charges(orders.as_ref(), charges.as_mut()).into_sorted_vec(),
            matched
        );
    }

    #[rstest]
    #[case(0, [], vec![vec![]])]
    #[case(1, [], vec![])]
    #[case(1, [1], vec![vec![1]])]
    #[case(1, [1, 1], vec![vec![1]])]
    #[case(2, [1, 2, 1], vec![vec![1, 1], vec![2]])]
    #[case(7, [1, 3, 1, 5, 2, 1], vec![
        vec![1, 1, 2, 3],
        vec![1, 1, 5],
        vec![2, 5],
    ])]
    fn exhaustive_finds_all_exact_fits(
        #[case] order: u32,
        #[case] charges: impl AsRef<[u32]>,
        #[case] expected: Vec<Vec<u32>>,
    ) {
        assert_eq!(
            exhaustive(order, charges.as_ref())
                .into_iter()
                .collect::<BinaryHeap<_>>()
                .into_sorted_vec(),
            expected,
        );
    }

    #[rstest]
    #[case(0, [], Some(vec![]))]
    #[case(0, [1], Some(vec![]))]
    #[case(1, [], None)]
    #[case(1, [1], Some(vec![1]))]
    #[case(1, [1, 2], Some(vec![1]))]
    #[case(1, [2], None)]
    #[case(7, [1, 3, 1, 5, 2, 1], Some(vec![1, 1, 2, 3]))]
    fn knapsack_finds_exact_fits_in_charges(
        #[case] order: u32,
        #[case] charges: impl AsRef<[u32]>,
        #[case] expected: Option<Vec<u32>>,
    ) {
        assert_eq!(
            knapsack(order, charges.as_ref()).map(BinaryHeap::into_sorted_vec),
            expected
        );
    }
}
