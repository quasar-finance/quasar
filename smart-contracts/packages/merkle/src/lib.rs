mod binary_search;
mod builder;
pub mod hash;
pub mod proof;

#[derive(Debug)]
pub struct Tree {
    leaf_count: u128,
    nodes: Vec<hash::Hash>,
}

impl Tree {
    pub fn new<T: AsRef<[u8]>>(items: &[T]) -> Self {
        if items.is_empty() {
            return Tree {
                leaf_count: 0,
                nodes: Vec::<hash::Hash>::new(),
            };
        }

        let mut nodes: Vec<hash::Hash> = builder::build_leaf_level(items);
        let leaf_count = nodes.len() as u128;

        builder::build_branch_levels(&mut nodes);

        Tree { leaf_count, nodes }
    }

    pub fn get_root(&self) -> Option<hash::Hash> {
        let node_count = self.nodes.len();
        if self.leaf_count == 0 {
            return None;
        }
        Some(self.nodes[node_count - 1])
    }

    pub fn find_proof<T: AsRef<[u8]>>(&self, item: &T) -> Option<proof::Proof> {
        if self.leaf_count <= 1 {
            return None;
        }

        let item_ref = item.as_ref();
        let hash_to_search_for = hash::leaf(item_ref);

        // binary search leaves
        let proof_index = binary_search::search(&self.nodes, self.leaf_count, &hash_to_search_for)?;

        let mut proof = proof::Proof::default();

        let mut level_length = self.leaf_count;
        let mut level_start = 0;
        let mut current_index = proof_index;

        let mut sibling_hash: hash::Hash;
        let mut is_left_sibling: bool;

        while level_length != 1 {
            is_left_sibling = current_index % 2 == 1;
            if is_left_sibling {
                // if current_index node is on the right, we need its left sibling
                sibling_hash = self.nodes[(level_start + current_index - 1) as usize];
            } else if current_index + 1 == level_length {
                // if current_index node is on the left but there is no right sibling
                // grab itself for proof.
                sibling_hash = self.nodes[(level_start + current_index) as usize];
            } else {
                // current_index node is on the left, grab its right sibling
                sibling_hash = self.nodes[(level_start + current_index + 1) as usize];
            }

            proof.push(is_left_sibling, sibling_hash);

            level_start += level_length;
            level_length = builder::get_next_level_length(level_length);
            current_index /= 2;
        }

        Some(proof)
    }

    #[allow(dead_code)]
    fn get_node_count(&self) -> u128 {
        self.nodes.len() as u128
    }

    #[allow(dead_code)]
    fn get_leaf_count(&self) -> Result<u128, String> {
        if self.leaf_count > self.get_node_count() {
            return Err(format!(
                "leaf count ({}) is greater than node count ({})",
                self.leaf_count,
                self.get_node_count()
            ));
        }
        Ok(self.leaf_count)
    }

    #[allow(dead_code)]
    fn get_node_at(&self, index: u128) -> Result<hash::Hash, String> {
        if index >= self.get_node_count() {
            return Err(format!(
                "requested index ({}) is greater than node count ({})",
                index,
                self.get_node_count()
            ));
        }
        Ok(self.nodes[index as usize])
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::proof::Entry;

    use super::*;

    #[test]
    fn new_merkle_tree_empty() {
        let items: Vec<String> = vec![];

        let mt = Tree::new(&items);

        let root = mt.get_root();

        assert_eq!(false, root.is_some());
        assert_eq!(0, mt.get_leaf_count().unwrap());
        assert_eq!(0, mt.get_node_count());

        match mt.get_node_at(0) {
            Ok(result) => {
                panic!("must have returned error but received {:?}", result)
            }
            Err(_error) => {
                // expected
            }
        }
    }

    #[test]
    fn new_merkle_tree_one_element() {
        let items: Vec<&[u8]> = vec![test_util::OSMO];

        let mt = Tree::new(&items);

        let root = mt.get_root();

        assert_eq!(true, root.is_some());
        assert_eq!(1, mt.get_leaf_count().unwrap());
        assert_eq!(1, mt.get_node_count());

        // TODO: extra this into helper and clean up tests
        match mt.get_node_at(0) {
            Ok(result) => {
                assert_eq!(hash::leaf(test_util::OSMO), result);
                assert_eq!(root.unwrap(), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }
    }

    #[test]
    fn new_merkle_tree_two_elements() {
        let mut items: Vec<&[u8]> = vec![test_util::OSMO, test_util::ION];

        let mt = Tree::new(&items);

        test_util::hash_and_sort(&mut items);

        let root = mt.get_root();

        assert_eq!(true, root.is_some());
        assert_eq!(2, mt.get_leaf_count().unwrap());
        assert_eq!(3, mt.get_node_count());

        match mt.get_node_at(0) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[0]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(1) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[1]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(2) {
            Ok(result) => {
                let left_hash: hash::Hash = hash::leaf(items[0]);
                let right_hash: hash::Hash = hash::leaf(items[1]);
                assert_eq!(hash::branch(&left_hash, &right_hash), result);
                assert_eq!(mt.get_root().unwrap(), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }
    }

    #[test]
    fn new_merkle_tree_three_elements() {
        let mut items: Vec<&[u8]> = vec![test_util::OSMO, test_util::WETH, test_util::ION];

        let mt = Tree::new(&items);

        test_util::hash_and_sort(&mut items);

        let root = mt.get_root();

        assert_eq!(true, root.is_some());
        assert_eq!(3, mt.get_leaf_count().unwrap());
        assert_eq!(6, mt.get_node_count());

        match mt.get_node_at(0) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[0]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(1) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[1]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(2) {
            Ok(result) => {
                assert_eq!(hash::leaf(items[2]), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(3) {
            Ok(result) => {
                let left_hash: hash::Hash = hash::leaf(items[0]);
                let right_hash: hash::Hash = hash::leaf(items[1]);
                assert_eq!(hash::branch(&left_hash, &right_hash), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(4) {
            Ok(result) => {
                let left_hash: hash::Hash = hash::leaf(items[2]);
                let right_hash: hash::Hash = hash::leaf(items[2]);
                assert_eq!(hash::branch(&left_hash, &right_hash), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }

        match mt.get_node_at(5) {
            Ok(result) => {
                assert_eq!(mt.get_root().unwrap(), result);

                let left_left_hash: hash::Hash = hash::leaf(items[0]);
                let left_right_hash: hash::Hash = hash::leaf(items[1]);

                let left_hash: hash::Hash = hash::branch(&left_left_hash, &left_right_hash);
                assert_eq!(hash::branch(&left_left_hash, &left_right_hash), left_hash);

                let right_left_hash: hash::Hash = hash::leaf(items[2]);
                let right_right_hash: hash::Hash = hash::leaf(items[2]);

                let right_hash: hash::Hash = hash::branch(&right_left_hash, &right_right_hash);
                assert_eq!(
                    hash::branch(&right_left_hash, &right_right_hash),
                    right_hash
                );

                assert_eq!(hash::branch(&left_hash, &right_hash), result);
            }
            Err(error) => {
                panic!("must have returned result but received error {:?}", error)
            }
        }
    }

    #[test]
    fn find_proof_one() {
        let items: Vec<&[u8]> = vec![test_util::OSMO];

        let mt = Tree::new(&items);

        let result = mt.find_proof(&test_util::OSMO);

        assert_eq!(true, result.is_none());
    }

    #[test]
    fn find_proof_two_items_found() {
        // N.B.: SHA3_256 lexicographical byte order is: hash(OSMO), hash(WETH).
        let items: Vec<&[u8]> = vec![test_util::OSMO, test_util::WETH];

        let mt = Tree::new(&items);

        let result = mt.find_proof(&test_util::WETH);

        assert_eq!(false, result.is_none());

        let actual_proof = result.unwrap();

        let actual_entry = actual_proof.get_entry_at(0);

        assert_eq!(1, actual_proof.get_num_entries());
        assert_eq!(Entry::new(true, hash::leaf(test_util::OSMO)), *actual_entry);
    }

    #[test]
    fn find_proof_two_items_not_found() {
        // N.B.: SHA3_256 lexicographical byte order is: hash(OSMO), hash(WETH).
        let items: Vec<&[u8]> = vec![test_util::OSMO, test_util::WETH];

        let mt = Tree::new(&items);

        let result = mt.find_proof(&test_util::ION);

        assert_eq!(true, result.is_none());
    }

    #[test]
    fn find_proof_five_items_right_found() {
        // N.B.: SHA3_256 lexicographical byte order is the following:
        // - hash(OSMO)
        // - hash(USDC)
        // - hash(ION)
        // - hash(AKT)
        // - hash(WETH)
        // Therefore the tree should be the following:
        //
        //                                                            root: hash(B_4, B_5)
        //
        //                               B_4: hash(B_0, B_1)                                          B_5: hash(B_3, B_3)
        //
        //   B_0: hash(hash(OSMO), hash(USDC))           B_1: hash(hash(ION), hash(AKT))      B_3: hash(hash(WETH), hash(WETH))    B_3
        //
        //     hash(OSMO) hash(USDC)                     hash(ION) hash(AKT)                      hash(WETH)
        let items: Vec<&[u8]> = vec![
            test_util::OSMO,
            test_util::ION,
            test_util::WETH,
            test_util::USDC,
            test_util::AKT,
        ];

        let mt = Tree::new(&items);

        let result = mt.find_proof(&test_util::ION);

        assert_eq!(false, result.is_none());

        let actual_proof = result.unwrap();

        assert_eq!(3, actual_proof.get_num_entries());

        assert_eq!(
            Entry::new(false, hash::leaf(test_util::AKT)),
            *actual_proof.get_entry_at(0)
        );

        assert_eq!(
            Entry::new(
                true,
                hash::branch(&hash::leaf(test_util::OSMO), &hash::leaf(test_util::USDC)) // B_0
            ),
            *actual_proof.get_entry_at(1)
        );

        let b_3 = hash::branch(&hash::leaf(test_util::WETH), &hash::leaf(test_util::WETH));
        assert_eq!(
            Entry::new(
                false,
                hash::branch(&b_3, &b_3) // B_5
            ),
            *actual_proof.get_entry_at(2)
        );
    }

    #[test]
    fn find_proof_five_items_left_last_node_found() {
        // N.B.: Please see previous test for the visualization.
        // The same tree is used while requesting for a different node.

        let items: Vec<&[u8]> = vec![
            test_util::OSMO,
            test_util::ION,
            test_util::WETH,
            test_util::USDC,
            test_util::AKT,
        ];

        let mt = Tree::new(&items);

        let result = mt.find_proof(&test_util::WETH);

        assert_eq!(false, result.is_none());

        let actual_proof = result.unwrap();

        assert_eq!(3, actual_proof.get_num_entries());

        assert_eq!(
            Entry::new(false, hash::leaf(test_util::WETH)),
            *actual_proof.get_entry_at(0)
        );

        assert_eq!(
            Entry::new(
                false,
                hash::branch(&hash::leaf(test_util::WETH), &hash::leaf(test_util::WETH)) // B_0
            ),
            *actual_proof.get_entry_at(1)
        );

        assert_eq!(
            Entry::new(
                true,
                hash::branch(
                    &hash::branch(&hash::leaf(test_util::OSMO), &hash::leaf(test_util::USDC)),
                    &hash::branch(&hash::leaf(test_util::ION), &hash::leaf(test_util::AKT))
                ) // B_4
            ),
            *actual_proof.get_entry_at(2)
        );
    }
}

#[cfg(test)]
pub mod test_util {
    use super::*;

    // Hashes to: {155, 130, 51, 5, 37, 74, 205, 223, ...}
    pub const OSMO: &[u8] = b"osmo";
    // Hashes to: {184, 142, 37, 50, 165, 100, 87, 208, ...}
    pub const ION: &[u8] = b"ion";
    // Hashes to: {248, 118, 160, 140, 119, 75, 115, ...}
    pub const WETH: &[u8] = b"weth";
    // Hashes to: {173, 151, 188, 10, 157, 161, 47, 157, ...}
    pub const USDC: &[u8] = b"usdc";
    // Hashes to: {222, 232, 28, 167, 68, 180, 84, 72, ...}
    pub const AKT: &[u8] = b"akt";

    pub fn hash_and_sort(items: &mut Vec<&[u8]>) {
        // We expect the constructor to sort the nodes by hash.
        pdqsort::sort_by(items, |a, b| hash::leaf(a).cmp(&hash::leaf(b)));
    }

    pub fn sort(items: &mut Vec<hash::Hash>) {
        // We expect the constructor to sort the nodes by hash.
        pdqsort::sort_by(items, |a, b| a.cmp(b));
    }
}
