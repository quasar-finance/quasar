use crate::hash;

/// TODO: spec and tests
pub fn build_leaf_level<T: AsRef<[u8]>>(items: &[T]) -> Vec<hash::Hash> {
    let mut nodes: Vec<hash::Hash> = Vec::new();
    for item in items.iter() {
        let item = item.as_ref();
        let hash = hash::leaf(item);
        nodes.push(hash)
    }

    // sort items so that we can binary search them
    // when finding proofs.
    pdqsort::sort_by(&mut nodes, |a, b| a.cmp(b));

    nodes
}

// build_branch_levels builds branch levels from the give leaf nodes.
// mutates the parameter by pushing the new nodes onto it.
// CONTRACT: nodes are sorted in incrasing order.
pub fn build_branch_levels(nodes: &mut Vec<hash::Hash>) {
    let mut previous_level_length = nodes.len() as u128;
    let mut current_level_length = get_next_level_length(previous_level_length);
    let mut previous_level_start = 0;
    while current_level_length > 0 {
        for i in 0..current_level_length {
            let previous_level_index = 2 * i;
            let nodes_index: u128 = previous_level_start + previous_level_index;
            let left_sibling = &nodes[nodes_index as usize];

            let right_sibling = if previous_level_index + 1 >= previous_level_length {
                &nodes[nodes_index as usize] // For the case where the number of nodes at a level is odd.
            } else {
                &nodes[(nodes_index + 1) as usize]
            };

            let hash = hash::branch(left_sibling, right_sibling);
            nodes.push(hash);
        }
        previous_level_start += previous_level_length;
        previous_level_length = current_level_length;
        current_level_length = get_next_level_length(current_level_length);
    }
}

/// TODO: spec
#[inline]
pub fn get_next_level_length(level_len: u128) -> u128 {
    if level_len == 1 {
        0
    } else {
        (level_len + 1) / 2
    }
}

/// TODO: spec
fn calculate_tree_capacity<T>(items: &[T]) -> u128 {
    let leaves_count = items.len() as u128;
    let branch_node_count = round_up_power_of_two(items.len() as u128);
    leaves_count + branch_node_count
}

/// round_up_power_of_two returns the next power of two
/// https://graphics.stanford.edu/~seander/bithacks.html#RoundUpPowerOf2
fn round_up_power_of_two(n: u128) -> u128 {
    let mut v = n;
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v += 1;
    v
}

#[cfg(test)]
mod tests {
    use crate::test_util;
    use std::{collections::HashMap, vec};

    use super::*;

    #[test]
    fn build_branch_level_one_node() {
        let items: Vec<&[u8]> = vec![test_util::OSMO];

        let mut actual_nodes: Vec<hash::Hash> = prepare_leaf_nodes(&items);
        let expected_nodes: Vec<hash::Hash> = actual_nodes.clone();

        build_branch_levels(&mut actual_nodes);

        validate_nodes(&expected_nodes, &actual_nodes);
    }

    #[test]
    fn round_up_two() {
        let tests = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1000];
        let sols = vec![2, 2, 4, 4, 8, 8, 8, 8, 16, 1024];
        for i in 1..tests.len() {
            assert_eq!(round_up_power_of_two(tests[i]), sols[i]);
        }
    }

    #[test]
    fn tree_capacity() {
        let mut tests: HashMap<u128, Vec<&str>> = HashMap::new();
        tests.insert(8, vec!["node", "node", "node", "node"]);
        tests.insert(7, vec!["node", "node", "node"]);
        tests.insert(13, vec!["node", "node", "node", "node", "node"]);
        tests.insert(2024, vec!["node"; 1000]);

        for tc in tests {
            let expected = (tc.1.len() as u128) + round_up_power_of_two(tc.1.len() as u128);
            assert_eq!(tc.0, expected);
        }
    }
    #[test]
    fn next_level_length() {
        let tests = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for tc in tests {
            let next_level = get_next_level_length(tc);
            if tc != 1 {
                assert_eq!(next_level, (tc + 1) / 2)
            } else {
                assert_eq!(next_level, 0)
            }
        }
    }
    #[test]
    fn build_branch_level_two_nodes() {
        let items: Vec<&[u8]> = vec![test_util::OSMO, test_util::ION];

        let mut actual_nodes: Vec<hash::Hash> = prepare_leaf_nodes(&items);

        let mut expected_nodes: Vec<hash::Hash> = actual_nodes.clone();
        expected_nodes.push(hash::branch(&expected_nodes[0], &expected_nodes[1]));

        build_branch_levels(&mut actual_nodes);

        validate_nodes(&expected_nodes, &actual_nodes);
    }

    #[test]
    fn build_branch_level_three_nodes() {
        let items: Vec<&[u8]> = vec![test_util::OSMO, test_util::ION, test_util::WETH];

        let mut actual_nodes: Vec<hash::Hash> = prepare_leaf_nodes(&items);

        let mut expected_nodes: Vec<hash::Hash> = actual_nodes.clone();
        expected_nodes.push(hash::branch(&expected_nodes[0], &expected_nodes[1]));
        expected_nodes.push(hash::branch(&expected_nodes[2], &expected_nodes[2]));
        expected_nodes.push(hash::branch(&expected_nodes[3], &expected_nodes[4]));

        build_branch_levels(&mut actual_nodes);

        validate_nodes(&expected_nodes, &actual_nodes);
    }

    #[test]
    fn build_branch_level_five_nodes() {
        let items: Vec<&[u8]> = vec![
            test_util::OSMO,
            test_util::ION,
            test_util::WETH,
            test_util::USDC,
            test_util::AKT,
        ];

        let mut actual_nodes: Vec<hash::Hash> = prepare_leaf_nodes(&items);

        let mut expected_nodes: Vec<hash::Hash> = actual_nodes.clone();
        // level 3
        expected_nodes.push(hash::branch(&expected_nodes[0], &expected_nodes[1]));
        expected_nodes.push(hash::branch(&expected_nodes[2], &expected_nodes[3]));
        expected_nodes.push(hash::branch(&expected_nodes[4], &expected_nodes[4]));

        // level 2
        expected_nodes.push(hash::branch(&expected_nodes[5], &expected_nodes[6]));
        expected_nodes.push(hash::branch(&expected_nodes[7], &expected_nodes[7]));

        // level 1
        expected_nodes.push(hash::branch(&expected_nodes[8], &expected_nodes[9]));

        build_branch_levels(&mut actual_nodes);

        validate_nodes(&expected_nodes, &actual_nodes);
    }

    fn prepare_leaf_nodes(items: &Vec<&[u8]>) -> Vec<hash::Hash> {
        let mut actual_nodes: Vec<hash::Hash> =
            items.into_iter().map(|i| hash::leaf(i)).rev().collect();

        test_util::sort(&mut actual_nodes);
        return actual_nodes;
    }

    fn validate_nodes(expected_nodes: &Vec<hash::Hash>, actual_nodes: &Vec<hash::Hash>) {
        assert_eq!(expected_nodes.len(), actual_nodes.len());
        for i in 0..actual_nodes.len() {
            assert_eq!(expected_nodes[i], actual_nodes[i], "index {}", i);
        }
    }
}
