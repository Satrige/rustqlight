use std::cell::RefCell;
use std::io;
use std::rc::Rc;

enum TreeNode<T> {
    Common(CommonNode<T>),
    Leaf(LeafNode<T>),
}

enum InsertType<T> {
    // We just put value to the leaf node
    Normal,
    // We had to split the leaf node
    // First value is a new median
    // Second is a left new node
    // Third is a right new node
    Split(T, Rc<RefCell<TreeNode<T>>>, Rc<RefCell<TreeNode<T>>>),
    // We put the value into the leaf node,
    // it got full,
    // so we also updated the minimum value of the current leaf node and
    // moved the smallest value to the left leaf node.
    // The wrapped value is the new smallest for the current node
    LeftMove(T),
    // Same as before, but we moved the biggest value to the right leaf node
    RightMove(T),
}

// Here the T type isn't the same as for CommonNode
struct LeafNode<T> {
    degree: usize,
    values: Vec<T>,
    left_node: Option<Rc<RefCell<TreeNode<T>>>>,
    right_node: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Ord + Copy> LeafNode<T> {
    pub fn new(degree: usize, values: Option<Vec<T>>) -> Self {
        let mut new_node = LeafNode {
            degree,
            values: vec![],
            left_node: None,
            right_node: None,
        };

        if let Some(values) = values {
            new_node.values.extend(values);
        };

        new_node
    }

    fn split(&mut self) -> (T, Rc<RefCell<TreeNode<T>>>, Rc<RefCell<TreeNode<T>>>) {
        let cur_num_values = self.values.len();

        let median_idx = cur_num_values / 2;
        let median_val = self.values[median_idx];

        let new_left_node = LeafNode::new(
            self.degree,
            Some(self.values.iter().take(median_idx).copied().collect()),
        );
        let new_right_node = LeafNode::new(
            self.degree,
            Some(self.values.iter().skip(median_idx).copied().collect()),
        );

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

        (median_val, new_left_ref, new_right_ref)
    }

    fn insert(&mut self, new_value: &T) -> InsertType<T> {
        self.values.push(*new_value);
        self.values.sort();

        let values_len = self.values.len();

        if values_len <= self.degree {
            return InsertType::Normal;
        }

        if let Some(ref mut left_node) = self.left_node {
            if let TreeNode::Leaf(ref mut borrowed_left_node) = *left_node.borrow_mut() {
                if borrowed_left_node.values.len() < self.degree {
                    let min_val = self.values[0].clone();
                    self.values.remove(0);
                    borrowed_left_node.values.push(min_val);

                    return InsertType::LeftMove(self.values[0].clone());
                }
            }
        };

        if let Some(ref right_node) = self.right_node {
            if let TreeNode::Leaf(ref mut borrowed_right_node) = *right_node.borrow_mut() {
                if (borrowed_right_node.values).len() < self.degree {
                    let max_val = self.values.pop().unwrap();
                    borrowed_right_node.values.insert(0, max_val);

                    return InsertType::RightMove(max_val);
                }
            }
        };

        let (median_value, new_left_leaf_ref, new_right_leaf_ref) = self.split();

        InsertType::Split(median_value, new_left_leaf_ref, new_right_leaf_ref)
    }
}

struct CommonNode<T> {
    degree: usize,
    values: Vec<T>,
    childs: Vec<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Ord + Copy> CommonNode<T> {
    fn new(
        degree: usize,
        values: Option<Vec<T>>,
        childs: Option<Vec<Rc<RefCell<TreeNode<T>>>>>,
    ) -> Self {
        let mut new_node = CommonNode {
            degree,
            values: vec![],
            childs: vec![],
        };

        if let Some(values) = values {
            new_node.values.extend(values);
        }

        if let Some(childs) = childs {
            new_node.childs.extend(childs);
        }

        new_node
    }

    fn split(&mut self) -> (T, Rc<RefCell<TreeNode<T>>>, Rc<RefCell<TreeNode<T>>>) {
        let median_idx = self.values.len() / 2;
        let median_val = self.values[median_idx];

        let left_node = CommonNode::new(
            self.degree,
            Some(self.values.iter().take(median_idx).copied().collect()), // [0, median_idx - 1]
            Some(
                self.childs
                    .iter()
                    .take(median_idx + 1)
                    .map(Rc::clone)
                    .collect(),
            ), // [0, median_idx]
        );
        let right_node = CommonNode::new(
            self.degree,
            Some(self.values.iter().skip(median_idx + 1).copied().collect()), // [median_idx + 1, n]
            Some(
                self.childs
                    .iter()
                    .skip(median_idx + 1)
                    .map(Rc::clone)
                    .collect(),
            ), // [median_idx + 1, m]
        );

        (
            median_val,
            Rc::new(RefCell::new(TreeNode::Common(left_node))),
            Rc::new(RefCell::new(TreeNode::Common(right_node))),
        )
    }

    fn insert(&mut self, new_value: &T) -> InsertType<T> {
        let child_idx = self
            .values
            .binary_search(new_value)
            .unwrap_or_else(|idx| idx);

        let insert_result = match *self.childs[child_idx].borrow_mut() {
            TreeNode::Common(ref mut borrowed_common_child) => {
                borrowed_common_child.insert(new_value)
            }
            TreeNode::Leaf(ref mut borrowed_leaf_child) => borrowed_leaf_child.insert(new_value),
        };

        match insert_result {
            InsertType::Normal => insert_result,
            InsertType::Split(new_median, left_node_ref, right_node_ref) => {
                self.values.insert(child_idx, new_median);
                self.childs
                    .splice(child_idx..=child_idx, vec![left_node_ref, right_node_ref]);

                if self.values.len() <= self.degree {
                    return InsertType::Normal;
                }

                let (new_median, left_node_ref, right_node_ref) = self.split();
                InsertType::Split(new_median, left_node_ref, right_node_ref)
            }
            InsertType::LeftMove(new_left_value) => {
                // If child_idx == 0
                // => means we inserted to the most right node of the left neighbour
                // => no need to update anything on the current level,
                // but need to propagate the value to the next level
                if child_idx != 0 {
                    self.values[child_idx - 1] = new_left_value;
                    InsertType::Normal
                } else {
                    InsertType::LeftMove(new_left_value)
                }
            }
            InsertType::RightMove(new_right_value) => {
                // If child_idx == self.degree
                // => means we inserted to the most left node of the right neighbour
                // => no need to update anything on the current level,
                // but need to propagate the value to the next level
                if child_idx < self.values.len() {
                    self.values[child_idx] = new_right_value;
                    InsertType::Normal
                } else {
                    InsertType::RightMove(new_right_value)
                }
            }
        }
    }
}

pub struct BTree<T> {
    leaf_degree: usize,
    common_degree: usize,
    root: TreeNode<T>,
}

impl<T: Ord + Copy> BTree<T> {
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
            root: TreeNode::Leaf(LeafNode::new(leaf_degree, None)),
        })
    }

    pub fn insert(&mut self, new_value: &T) {
        let insert_result = match self.root {
            TreeNode::Leaf(ref mut leaf_node) => leaf_node.insert(new_value),
            TreeNode::Common(ref mut common_node) => common_node.insert(new_value),
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

    fn find_value(&self, node: &TreeNode<T>, value: &T) -> Option<T> {
        let values = match node {
            TreeNode::Common(common_node) => &common_node.values,
            TreeNode::Leaf(leaf_node) => &leaf_node.values,
        };

        if values.len() == 0 {
            return None;
        }

        match values.binary_search(value) {
            Ok(idx) => match node {
                TreeNode::Common(ref common_node) =>
                    self.find_value(&*common_node.childs[idx + 1].borrow(), value),
                TreeNode::Leaf(_) => Some(*value),
            },
            Err(idx) => match node {
                TreeNode::Common(ref common_node) =>
                    self.find_value(&*common_node.childs[idx].borrow(), value),
                TreeNode::Leaf(_) => None,
            },
        }
    }

    pub fn search(&self, value: &T) -> Option<T> {
        self.find_value(&self.root, value)
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
            let b_tree: BTree<i32> = BTree::new(2, 3).unwrap();

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
            let b_tree: BTree<i32> = BTree::new(2, 3).unwrap();

            assert_eq!(b_tree.search(&10), None);
        }

        #[test]
        fn it_should_search_in_tree_with_single_value() {
            let mut b_tree: BTree<i32> = BTree::new(2, 3).unwrap();

            b_tree.insert(&10);
            assert_eq!(b_tree.search(&10), Some(10));
        }

        #[test]
        fn test_search_multiple_values() {
            let mut b_tree: BTree<i32> = BTree::new(2, 3).unwrap();

            b_tree.insert(&1);
            b_tree.insert(&2);
            b_tree.insert(&3);
            b_tree.insert(&4);

            assert_eq!(b_tree.search(&1), Some(1));
            assert_eq!(b_tree.search(&2), Some(2));
            assert_eq!(b_tree.search(&3), Some(3));
            assert_eq!(b_tree.search(&4), Some(4));
        }
    }

    mod tree_topology {
        use super::*;

        #[test]
        fn it_should_store_all_values_correctly_for_1_leaf_node_degree_2() {
            let mut b_tree: BTree<i32> = BTree::new(2, 3).unwrap();

            b_tree.insert(&10);
            b_tree.insert(&20);

            match &b_tree.root {
                TreeNode::Leaf(ref root_node) => {
                    assert_eq!(root_node.values, vec![10, 20]);
                }
                _ => unreachable!("Root should be a leaf node"),
            };
        }

        #[test]
        fn it_should_store_all_values_correctly_for_leaf_degree_2_common_degree_3() {
            let mut b_tree: BTree<i32> = BTree::new(2, 3).unwrap();

            b_tree.insert(&10);
            b_tree.insert(&20);
            b_tree.insert(&5);
            b_tree.insert(&6);
            b_tree.insert(&12);
            b_tree.insert(&30);
            b_tree.insert(&7);
            b_tree.insert(&4);

            match &b_tree.root {
                TreeNode::Common(ref root_node) => {
                    assert_eq!(root_node.values, vec![6, 10, 20]);

                    let child_node_0 = &root_node.childs[0];
                    match *child_node_0.borrow() {
                        TreeNode::Leaf(ref child) => assert_eq!(child.values, vec![4, 5]),
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_1 = &root_node.childs[1];
                    match *child_node_1.borrow() {
                        TreeNode::Leaf(ref child) => assert_eq!(child.values, vec![6, 7]),
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_2 = &root_node.childs[2];
                    match *child_node_2.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(child.values, vec![10, 12]);
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_3 = &root_node.childs[3];
                    match *child_node_3.borrow() {
                        TreeNode::Leaf(ref child) => {
                            assert_eq!(child.values, vec![20, 30]);
                        }
                        _ => unreachable!("unreachable"),
                    };
                }
                _ => unreachable!("Root should be a common node"),
            }
        }

        #[test]
        fn it_should_correctly_store_all_values_after_left_move() {
            let mut b_tree: BTree<i32> = BTree::new(2, 2).unwrap();

            b_tree.insert(&6);
            b_tree.insert(&8);
            b_tree.insert(&10);
            b_tree.insert(&12);
            b_tree.insert(&14);
            b_tree.insert(&4);
            b_tree.insert(&16);
            b_tree.insert(&11);

            match &b_tree.root {
                TreeNode::Common(ref root) => {
                    assert_eq!(root.values, vec![11]);

                    let child_node_0 = &root.childs[0];
                    match *child_node_0.borrow() {
                        TreeNode::Common(ref child) => {
                            assert_eq!(child.values, vec![8]);

                            let child_node_0 = &child.childs[0];
                            match *child_node_0.borrow() {
                                TreeNode::Leaf(ref child) => assert_eq!(child.values, vec![4, 6]),
                                _ => unreachable!("unreachable"),
                            };

                            let child_node_1 = &child.childs[1];
                            match *child_node_1.borrow() {
                                TreeNode::Leaf(ref child) => assert_eq!(child.values, vec![8, 10]),
                                _ => unreachable!("unreachable"),
                            };
                        }
                        _ => unreachable!("unreachable"),
                    };

                    let child_node_1 = &root.childs[1];
                    match *child_node_1.borrow() {
                        TreeNode::Common(ref child) => {
                            assert_eq!(child.values, vec![14]);

                            let child_node_0 = &child.childs[0];
                            match *child_node_0.borrow() {
                                TreeNode::Leaf(ref child) => assert_eq!(child.values, vec![11, 12]),
                                _ => unreachable!("unreachable"),
                            };

                            let child_node_1 = &child.childs[1];
                            match *child_node_1.borrow() {
                                TreeNode::Leaf(ref child) => assert_eq!(child.values, vec![14, 16]),
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
