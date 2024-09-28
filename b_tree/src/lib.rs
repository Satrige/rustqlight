mod common_node;
mod leaf_node;
mod types;

use common_node::CommonNode;
use leaf_node::LeafNode;
use std::io;
use types::{InsertType, TreeNode};

pub struct BTree<K, V> {
    leaf_degree: usize,
    common_degree: usize,
    root: TreeNode<K, V>,
}

impl<K: Ord + Copy, V: Clone> BTree<K, V> {
    pub fn new(leaf_degree: usize, common_degree: usize) -> io::Result<Self> {
        if leaf_degree == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "The leaf node degree could not be equal to 0",
            ));
        }

        if common_degree <= 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "The common node degree should be greater than 1",
            ));
        }

        Ok(BTree {
            leaf_degree,
            common_degree,
            root: TreeNode::Leaf(LeafNode::new(leaf_degree, None, None)),
        })
    }

    pub fn insert(&mut self, new_key: &K, new_value: &V) {
        let insert_result = match self.root {
            TreeNode::Leaf(ref mut leaf_node) => leaf_node.insert(new_key, new_value),
            TreeNode::Common(ref mut common_node) => common_node.insert(new_key, new_value),
        };

        match insert_result {
            InsertType::Normal => return,
            InsertType::Split(new_median, new_left_ref, new_right_ref) => {
                self.root = TreeNode::Common(CommonNode::new(
                    self.common_degree,
                    Some(vec![new_median]),
                    Some(vec![new_left_ref, new_right_ref]),
                ));
            }
            _ => unreachable!("The root can either split or normally insert"),
        }
    }

    fn find_value(&self, node: &TreeNode<K, V>, key: &K) -> Option<V> {
        let keys = match node {
            TreeNode::Common(common_node) => common_node.get_keys(),
            TreeNode::Leaf(leaf_node) => leaf_node.get_keys(),
        };

        if keys.len() == 0 {
            return None;
        }

        match keys.binary_search(key) {
            Ok(idx) => match node {
                TreeNode::Common(ref common_node) => {
                    let childs = common_node.get_childs();
                    self.find_value(&childs[idx + 1].borrow(), key)
                }
                TreeNode::Leaf(ref leaf_node) => {
                    let values = leaf_node.get_values();
                    Some(values[idx].clone())
                }
            },
            Err(idx) => match node {
                TreeNode::Common(ref common_node) => {
                    let childs = common_node.get_childs();
                    self.find_value(&childs[idx].borrow(), key)
                }
                TreeNode::Leaf(_) => None,
            },
        }
    }

    pub fn search(&self, key: &K) -> Option<V> {
        self.find_value(&self.root, key)
    }

    pub fn get_settings(&self) -> (usize, usize) {
        (self.common_degree, self.leaf_degree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tree_creation {
        use super::*;

        #[test]
        fn it_should_create_new_tree() {
            let b_tree: BTree<i32, i32> = BTree::new(2, 3).unwrap();
            let (common_degree, leaf_degree) = b_tree.get_settings();

            assert_eq!(leaf_degree, 2);
            assert_eq!(common_degree, 3);
        }
    }

    mod tree_insert_and_search {
        use super::*;

        #[test]
        fn it_should_search_in_empty_tree() {
            let b_tree: BTree<i32, i32> = BTree::new(2, 3).unwrap();

            assert_eq!(b_tree.search(&10), None);
        }

        #[test]
        fn it_should_search_in_tree_with_single_value() {
            let mut b_tree: BTree<i32, i32> = BTree::new(2, 3).unwrap();

            b_tree.insert(&10, &110);
            assert_eq!(b_tree.search(&10), Some(110));
        }

        #[test]
        fn test_search_multiple_values() {
            let mut b_tree: BTree<i32, i32> = BTree::new(2, 3).unwrap();

            b_tree.insert(&1, &11);
            b_tree.insert(&2, &12);
            b_tree.insert(&3, &13);
            b_tree.insert(&4, &14);

            assert_eq!(b_tree.search(&1), Some(11));
            assert_eq!(b_tree.search(&2), Some(12));
            assert_eq!(b_tree.search(&3), Some(13));
            assert_eq!(b_tree.search(&4), Some(14));
        }
    }

    mod tree_topology {
        use super::*;

        #[test]
        fn it_should_store_all_values_correctly_for_1_leaf_node_degree_2() {
            let mut b_tree: BTree<i32, i32> = BTree::new(2, 3).unwrap();

            b_tree.insert(&10, &110);
            b_tree.insert(&20, &120);

            match &b_tree.root {
                TreeNode::Leaf(ref root_node) => {
                    assert_eq!(*root_node.get_keys(), vec![10, 20]);
                    assert_eq!(*root_node.get_values(), vec![110, 120]);
                }
                _ => unreachable!("Root should be a leaf node"),
            };
        }

        #[test]
        fn it_should_store_all_values_correctly_for_leaf_degree_2_common_degree_3() {
            let mut b_tree: BTree<i32, i32> = BTree::new(2, 3).unwrap();

            b_tree.insert(&10, &110);
            b_tree.insert(&20, &120);
            b_tree.insert(&5, &15);
            b_tree.insert(&6, &16);
            b_tree.insert(&12, &112);
            b_tree.insert(&30, &130);
            b_tree.insert(&7, &17);
            b_tree.insert(&4, &14);

            match &b_tree.root {
                TreeNode::Common(ref root_node) => {
                    assert_eq!(*root_node.get_keys(), vec![6, 10, 20]);

                    let child_node_0 = &root_node.get_childs()[0];
                    match *child_node_0.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(*child.get_keys(), vec![4, 5]);
                            assert_eq!(*child.get_values(), vec![14, 15]);
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_1 = &root_node.get_childs()[1];
                    match *child_node_1.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(*child.get_keys(), vec![6, 7]);
                            assert_eq!(*child.get_values(), vec![16, 17]);
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_2 = &root_node.get_childs()[2];
                    match *child_node_2.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(*child.get_keys(), vec![10, 12]);
                            assert_eq!(*child.get_values(), vec![110, 112]);
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_3 = &root_node.get_childs()[3];
                    match *child_node_3.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(*child.get_keys(), vec![20, 30]);
                            assert_eq!(*child.get_values(), vec![120, 130]);
                        }
                        _ => unreachable!("unreachable"),
                    };
                }
                _ => unreachable!("Root should be a common node"),
            }
        }

        #[test]
        fn it_should_correctly_store_all_values_after_left_move() {
            let mut b_tree: BTree<i32, i32> = BTree::new(2, 2).unwrap();

            b_tree.insert(&6, &16);
            b_tree.insert(&8, &18);
            b_tree.insert(&10, &110);
            b_tree.insert(&12, &112);
            b_tree.insert(&14, &114);
            b_tree.insert(&4, &14);
            b_tree.insert(&16, &116);
            b_tree.insert(&11, &111);

            match &b_tree.root {
                TreeNode::Common(ref root) => {
                    assert_eq!(*root.get_keys(), vec![11]);

                    let child_node_0 = &root.get_childs()[0];
                    match *child_node_0.borrow() {
                        TreeNode::Common(ref child) => {
                            assert_eq!(*child.get_keys(), vec![8]);

                            let child_node_0 = &child.get_childs()[0];
                            match *child_node_0.borrow() {
                                TreeNode::Leaf(ref child) => {
                                    assert_eq!(*child.get_keys(), vec![4, 6]);
                                    assert_eq!(*child.get_values(), vec![14, 16]);
                                }
                                _ => unreachable!("unreachable"),
                            };

                            let child_node_1 = &child.get_childs()[1];
                            match *child_node_1.borrow() {
                                TreeNode::Leaf(ref child) => {
                                    assert_eq!(*child.get_keys(), vec![8, 10]);
                                    assert_eq!(*child.get_values(), vec![18, 110]);
                                }
                                _ => unreachable!("unreachable"),
                            };
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_1 = &root.get_childs()[1];
                    match *child_node_1.borrow() {
                        TreeNode::Common(ref child) => {
                            assert_eq!(*child.get_keys(), vec![14]);

                            let child_node_0 = &child.get_childs()[0];
                            match *child_node_0.borrow() {
                                TreeNode::Leaf(ref child) => {
                                    assert_eq!(*child.get_keys(), vec![11, 12]);
                                    assert_eq!(*child.get_values(), vec![111, 112]);
                                }
                                _ => unreachable!("unreachable"),
                            };

                            let child_node_1 = &child.get_childs()[1];
                            match *child_node_1.borrow() {
                                TreeNode::Leaf(ref child) => {
                                    assert_eq!(*child.get_keys(), vec![14, 16]);
                                    assert_eq!(*child.get_values(), vec![114, 116]);
                                }
                                _ => unreachable!("unreachable"),
                            };
                        }
                        _ => unreachable!("unreachable"),
                    };
                }
                _ => unreachable!("Root should be a common node"),
            };
        }
    }
}
