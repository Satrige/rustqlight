use std::cell::RefCell;
use std::rc::Rc;
use std::{io, mem};

enum TreeNode<K, V> {
    Common(CommonNode<K, V>),
    Leaf(LeafNode<K, V>),
}

enum InsertType<K, V> {
    // We just put value to the leaf node
    Normal,
    // We had to split the leaf node
    // First value is a new median
    // Second is a left new node
    // Third is a right new node
    Split(K, Rc<RefCell<TreeNode<K, V>>>, Rc<RefCell<TreeNode<K, V>>>),
    // We put the value into the leaf node,
    // it got full,
    // so we also updated the minimum value of the current leaf node and
    // moved the smallest value to the left leaf node.
    // The wrapped value is the new smallest for the current node
    LeftMove(K),
    // Same as before, but we moved the biggest value to the right leaf node
    RightMove(K),
}

// Here the T type isn't the same as for CommonNode
struct LeafNode<K, V> {
    degree: usize,
    keys: Vec<K>,
    values: Vec<V>,
    left_node: Option<Rc<RefCell<TreeNode<K, V>>>>,
    right_node: Option<Rc<RefCell<TreeNode<K, V>>>>,
}

impl<K: Ord + Copy, V: Clone> LeafNode<K, V> {
    pub fn new(degree: usize, keys: Option<Vec<K>>, values: Option<Vec<V>>) -> Self {
        let mut new_node = LeafNode {
            degree,
            values: vec![],
            keys: vec![],
            left_node: None,
            right_node: None,
        };

        if let Some(keys) = keys {
            new_node.keys.extend(keys);
        }

        if let Some(values) = values {
            new_node.values.extend(values);
        };

        new_node
    }

    fn split(&mut self) -> (K, Rc<RefCell<TreeNode<K, V>>>, Rc<RefCell<TreeNode<K, V>>>) {
        let cur_num_values = self.values.len();

        let median_idx = cur_num_values / 2;
        let median_key = self.keys[median_idx];

        let right_keys = self.keys.split_off(median_idx);
        let right_values = self.values.split_off(median_idx);
        let new_right_node = LeafNode::new(self.degree, Some(right_keys), Some(right_values));

        let left_keys = mem::take(&mut self.keys);
        let left_values = mem::take(&mut self.values);
        let new_left_node = LeafNode::new(self.degree, Some(left_keys), Some(left_values));

        let new_left_ref = Rc::new(RefCell::new(TreeNode::Leaf(new_left_node)));
        let new_right_ref = Rc::new(RefCell::new(TreeNode::Leaf(new_right_node)));

        if let TreeNode::Leaf(ref mut new_left_node) = *new_left_ref.borrow_mut() {
            new_left_node.right_node = Some(Rc::clone(&new_right_ref));

            if let Some(ref mut left_node) = self.left_node {
                new_left_node.left_node = Some(Rc::clone(left_node));

                if let TreeNode::Leaf(ref mut old_left_node) = *left_node.borrow_mut() {
                    old_left_node.right_node = Some(Rc::clone(&new_left_ref));
                }
            }
        }

        if let TreeNode::Leaf(ref mut new_right_node) = *new_right_ref.borrow_mut() {
            new_right_node.left_node = Some(Rc::clone(&new_left_ref));

            if let Some(ref mut right_node) = self.right_node {
                new_right_node.right_node = Some(Rc::clone(right_node));

                if let TreeNode::Leaf(ref mut old_right_node) = *right_node.borrow_mut() {
                    old_right_node.left_node = Some(Rc::clone(&new_right_ref));
                }
            }
        }

        (median_key, new_left_ref, new_right_ref)
    }

    fn insert(&mut self, new_key: &K, new_value: &V) -> InsertType<K, V> {
        let new_key_pos = self.keys.binary_search(new_key).unwrap_or_else(|idx| idx);
        self.keys.insert(new_key_pos, *new_key);
        self.values.insert(new_key_pos, new_value.clone());

        let values_len = self.values.len();

        if values_len <= self.degree {
            return InsertType::Normal;
        }

        if let Some(ref left_node) = self.left_node {
            if let TreeNode::Leaf(ref mut borrowed_left_node) = *left_node.borrow_mut() {
                if borrowed_left_node.values.len() < self.degree {
                    borrowed_left_node.keys.push(self.keys.remove(0));
                    borrowed_left_node.values.push(self.values.remove(0));

                    return InsertType::LeftMove(self.keys[0].clone());
                }
            }
        };

        if let Some(ref right_node) = self.right_node {
            if let TreeNode::Leaf(ref mut borrowed_right_node) = *right_node.borrow_mut() {
                if (borrowed_right_node.values).len() < self.degree {
                    let max_key = self.keys.pop().unwrap();
                    borrowed_right_node.keys.insert(0, max_key.clone());

                    let max_val = self.values.pop().unwrap();
                    borrowed_right_node.values.insert(0, max_val);

                    return InsertType::RightMove(max_key);
                }
            }
        };

        let (median_key, new_left_leaf_ref, new_right_leaf_ref) = self.split();

        InsertType::Split(median_key, new_left_leaf_ref, new_right_leaf_ref)
    }
}

struct CommonNode<K, V> {
    degree: usize,
    keys: Vec<K>,
    childs: Vec<Rc<RefCell<TreeNode<K, V>>>>,
}

impl<K: Ord + Copy, V: Clone> CommonNode<K, V> {
    fn new(
        degree: usize,
        keys: Option<Vec<K>>,
        childs: Option<Vec<Rc<RefCell<TreeNode<K, V>>>>>,
    ) -> Self {
        let mut new_node = CommonNode {
            degree,
            keys: vec![],
            childs: vec![],
        };

        if let Some(keys) = keys {
            new_node.keys.extend(keys);
        }

        if let Some(childs) = childs {
            new_node.childs.extend(childs);
        }

        new_node
    }

    fn split(&mut self) -> (K, Rc<RefCell<TreeNode<K, V>>>, Rc<RefCell<TreeNode<K, V>>>) {
        let median_idx = self.keys.len() / 2;

        let right_keys = self.keys.split_off(median_idx + 1); // [median_idx + 1, n]
        let right_childs = self.childs.split_off(median_idx + 1); // [median_idx + 1, m]

        let median_key= self.keys.pop().unwrap();

        let left_keys = mem::take(&mut self.keys); // [0, median_idx - 1]
        let left_childs = mem::take(&mut self.childs); // [0, median_idx]

        let left_node = CommonNode::new(self.degree, Some(left_keys), Some(left_childs));
        let right_node = CommonNode::new(self.degree, Some(right_keys), Some(right_childs));

        (
            median_key,
            Rc::new(RefCell::new(TreeNode::Common(left_node))),
            Rc::new(RefCell::new(TreeNode::Common(right_node))),
        )
    }

    fn insert(&mut self, new_key: &K, new_value: &V) -> InsertType<K, V> {
        let new_key_idx = self.keys
            .binary_search(new_key)
            .unwrap_or_else(|idx| idx);

        let insert_result = match *self.childs[new_key_idx].borrow_mut() {
            TreeNode::Common(ref mut borrowed_common_child) => borrowed_common_child.insert(new_key, new_value),
            TreeNode::Leaf(ref mut borrowed_leaf_child) => borrowed_leaf_child.insert(new_key, new_value),
        };

        match insert_result {
            InsertType::Normal => insert_result,
            InsertType::Split(new_median_key, left_node_ref, right_node_ref) => {
                self.keys.insert(new_key_idx, new_median_key);
                self.childs
                    .splice(new_key_idx..=new_key_idx, vec![left_node_ref, right_node_ref]);

                if self.keys.len() <= self.degree {
                    return InsertType::Normal;
                }

                let (new_median, left_node_ref, right_node_ref) = self.split();
                InsertType::Split(new_median, left_node_ref, right_node_ref)
            }
            InsertType::LeftMove(new_left_key) => {
                // If child_idx == 0
                // => means we inserted to the most right node of the left neighbour
                // => no need to update anything on the current level,
                // but need to propagate the value to the next level
                if new_key_idx != 0 {
                    self.keys[new_key_idx - 1] = new_left_key;
                    InsertType::Normal
                } else {
                    InsertType::LeftMove(new_left_key)
                }
            }
            InsertType::RightMove(new_right_key) => {
                // If child_idx == self.degree
                // => means we inserted to the most left node of the right neighbour
                // => no need to update anything on the current level,
                // but need to propagate the value to the next level
                if new_key_idx < self.keys.len() {
                    self.keys[new_key_idx] = new_right_key;
                    InsertType::Normal
                } else {
                    InsertType::RightMove(new_right_key)
                }
            }
        }
    }
}

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
            TreeNode::Common(common_node) => &common_node.keys,
            TreeNode::Leaf(leaf_node) => &leaf_node.keys,
        };

        if keys.len() == 0 {
            return None;
        }

        match keys.binary_search(key) {
            Ok(idx) => match node {
                TreeNode::Common(ref common_node) => {
                    self.find_value(&*common_node.childs[idx + 1].borrow(), key)
                }
                TreeNode::Leaf(ref leaf_node) => Some(leaf_node.values[idx].clone()),
            },
            Err(idx) => match node {
                TreeNode::Common(ref common_node) => {
                    self.find_value(&*common_node.childs[idx].borrow(), key)
                }
                TreeNode::Leaf(_) => None,
            },
        }
    }

    pub fn search(&self, key: &K) -> Option<V> {
        self.find_value(&self.root, key)
    }

    pub fn show_settings(&self) {
        println!("Common degree: {:}", self.common_degree);
        println!("Leaf degree: {:}", self.leaf_degree);
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

            assert_eq!(b_tree.leaf_degree, 2);
            assert_eq!(b_tree.common_degree, 3);

            match &b_tree.root {
                TreeNode::Leaf(ref leaf_node) => {
                    assert_eq!(leaf_node.degree, 2);
                    assert_eq!(leaf_node.values.len(), 0);
                }
                _ => unreachable!("The root can't be the common node"),
            }
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
                    assert_eq!(root_node.keys, vec![10, 20]);
                    assert_eq!(root_node.values, vec![110, 120]);
                }
                _ => unreachable!("Root should be a leaf node"),
            };
        }

        #[test]
        fn it_should_store_all_values_correctly_for_leaf_degree_2_common_degree_3() {
            let mut b_tree: BTree<i32, i32> = BTree::new(2, 3).unwrap();

            b_tree.insert(&10, &110);
            b_tree.insert(&20, &120);
            b_tree.insert(&5,&15);
            b_tree.insert(&6, &16);
            b_tree.insert(&12, &112);
            b_tree.insert(&30, &130);
            b_tree.insert(&7, &17);
            b_tree.insert(&4, &14);

            match &b_tree.root {
                TreeNode::Common(ref root_node) => {
                    assert_eq!(root_node.keys, vec![6, 10, 20]);

                    let child_node_0 = &root_node.childs[0];
                    match *child_node_0.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(child.keys, vec![4, 5]);
                            assert_eq!(child.values, vec![14, 15]);
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_1 = &root_node.childs[1];
                    match *child_node_1.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(child.keys, vec![6, 7]);
                            assert_eq!(child.values, vec![16, 17]);
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_2 = &root_node.childs[2];
                    match *child_node_2.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(child.keys, vec![10, 12]);
                            assert_eq!(child.values, vec![110, 112]);
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_3 = &root_node.childs[3];
                    match *child_node_3.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(child.keys, vec![20, 30]);
                            assert_eq!(child.values, vec![120, 130]);
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
                    assert_eq!(root.keys, vec![11]);

                    let child_node_0 = &root.childs[0];
                    match *child_node_0.borrow() {
                        TreeNode::Common(ref child) => {
                            assert_eq!(child.keys, vec![8]);

                            let child_node_0 = &child.childs[0];
                            match *child_node_0.borrow() {
                                TreeNode::Leaf(ref child) => {
                                    assert_eq!(child.keys, vec![4, 6]);
                                    assert_eq!(child.values, vec![14, 16]);
                                }
                                _ => unreachable!("unreachable"),
                            };

                            let child_node_1 = &child.childs[1];
                            match *child_node_1.borrow() {
                                TreeNode::Leaf(ref child) => {
                                    assert_eq!(child.keys, vec![8, 10]);
                                    assert_eq!(child.values, vec![18, 110]);
                                }
                                _ => unreachable!("unreachable"),
                            };
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_1 = &root.childs[1];
                    match *child_node_1.borrow() {
                        TreeNode::Common(ref child) => {
                            assert_eq!(child.keys, vec![14]);

                            let child_node_0 = &child.childs[0];
                            match *child_node_0.borrow() {
                                TreeNode::Leaf(ref child) => {
                                    assert_eq!(child.keys, vec![11, 12]);
                                    assert_eq!(child.values, vec![111, 112]);
                                }
                                _ => unreachable!("unreachable"),
                            };

                            let child_node_1 = &child.childs[1];
                            match *child_node_1.borrow() {
                                TreeNode::Leaf(ref child) => {
                                    assert_eq!(child.keys, vec![14, 16]);
                                    assert_eq!(child.values, vec![114, 116]);
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
