use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode<T> {
    degree: usize,
    is_leaf: bool,
    values: Vec<T>,
    childs: Vec<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Ord + Copy> TreeNode<T> {
    fn new(
        degree: usize,
        is_leaf: bool,
        values: Option<Vec<T>>,
        childs: Option<Vec<Rc<RefCell<TreeNode<T>>>>>,
    ) -> Self {
        let mut new_node = TreeNode {
            degree,
            is_leaf,
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

    fn split(&mut self) -> (T, TreeNode<T>, TreeNode<T>) {
        let cur_num_values = self.values.len();

        let median_idx = cur_num_values / 2;
        let median_val = self.values[median_idx];

        let mut left_node = TreeNode::new(self.degree, self.is_leaf, None, None);
        let mut right_node = TreeNode::new(self.degree, self.is_leaf, None, None);

        left_node.values.extend(self.values[0..median_idx].iter().copied());
        right_node.values.extend(self.values[median_idx + 1..self.values.len()].iter().copied());

        for child in self.childs.iter() {
            let cloned_child = Rc::clone(child);
            // TODO check if we really need to check every time this val.
            // Maybe just median_idx is enough
            if *child.borrow().values.last().unwrap() < median_val {
                left_node.childs.push(cloned_child);
            } else {
                right_node.childs.push(cloned_child);
            }
        }

        (median_val, left_node, right_node)
    }

    fn insert(&mut self, new_value: &T) -> Option<(T, TreeNode<T>, TreeNode<T>)> {
        if self.is_leaf {
            self.values.push(*new_value);
            self.values.sort();

            if self.values.len() <= self.degree {
                return None;
            }

            return Some(self.split());
        }

        // TODO: Rewrite using binary search
        let child_idx = self
            .values
            .iter()
            .position(|&value| value > *new_value)
            .unwrap_or_else(|| self.values.len());

        let insert_result = self.childs[child_idx].borrow_mut().insert(new_value);

        return if let Some((new_median, left_node, right_node)) = insert_result {
            self.values.insert(child_idx, new_median.clone());

            self.childs.splice(
                child_idx..=child_idx,
                vec![
                    Rc::new(RefCell::new(left_node)),
                    Rc::new(RefCell::new(right_node)),
                ],
            );

            if self.values.len() <= self.degree {
                return None;
            }

            Some(self.split())
        } else {
            None
        };
    }
}

pub struct BTree<T> {
    degree: usize,
    root: Rc<RefCell<TreeNode<T>>>,
}

impl<T: Ord + Copy> BTree<T> {
    pub fn new(degree: usize) -> Self {
        if degree == 0 {
            panic!("Wrong value for the tree degree!")
        }

        BTree {
            degree,
            root: Rc::new(RefCell::new(TreeNode::new(degree, true, None, None))),
        }
    }

    pub fn insert(&mut self, new_value: &T) {
        let insert_result = {
            let mut borrowed_root = self.root.borrow_mut();

            borrowed_root.insert(new_value)
        };

        if let Some((new_value, left_node, right_node)) = insert_result {
            let new_root = TreeNode::new(
                self.degree,
                false,
                Some(vec![new_value]),
                Some(vec![
                    Rc::new(RefCell::new(left_node)),
                    Rc::new(RefCell::new(right_node)),
                ]),
            );

            self.root = Rc::new(RefCell::new(new_root));
        }
    }

    fn binary_search(data: &Vec<T>, value: &T) -> usize {
        if data.len() == 0 {
            return 0;
        }

        let mut lo: usize = 0;
        let mut hi: usize = data.len();

        while lo < hi {
            let mid_idx = lo + (hi - lo) / 2;
            let mid_val = &data[mid_idx];

            if *mid_val < *value {
                lo = mid_idx + 1;
            } else if *mid_val > *value {
                hi = mid_idx;
            } else {
                lo = mid_idx;
                break;
            }
        }

        lo
    }

    fn find_value(&self, node: &TreeNode<T>, value: &T) -> Option<T> {
        if node.values.len() == 0 {
            return None;
        }

        let idx = Self::binary_search(&node.values, value);

        if idx < node.values.len() && node.values[idx] == *value {
            return Some(*value);
        }

        if node.is_leaf {
            return None;
        }

        self.find_value(&node.childs[idx].borrow(), value)
    }

    pub fn search(&self, value: &T) -> Option<T> {
        self.find_value(&self.root.borrow(), value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod tree_creation {
        use super::*;

        #[test]
        fn it_should_create_new_tree() {
            let b_tree: BTree<i32> = BTree::new(3);
            assert_eq!(b_tree.root.borrow().degree, 3);
            assert!(b_tree.root.borrow().is_leaf);
            assert_eq!(b_tree.root.borrow().values.len(), 0);
            assert_eq!(b_tree.root.borrow().childs.len(), 0);
        }
    }

    mod tree_insert_and_search {
        use super::*;

        #[test]
        fn it_should_search_in_empty_tree() {
            let b_tree: BTree<i32> = BTree::new(3);
            assert_eq!(b_tree.search(&10), None);
        }

        #[test]
        fn it_should_search_in_tree_with_single_value() {
            let mut b_tree: BTree<i32> = BTree::new(3);
            b_tree.insert(&10);
            assert_eq!(b_tree.search(&10), Some(10));
        }

        #[test]
        fn test_search_multiple_values() {
            let mut b_tree: BTree<i32> = BTree::new(3);
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
        fn it_should_store_all_values_correctly_for_degree_3() {
            let mut b_tree: BTree<i32> = BTree::new(3);
            b_tree.insert(&10);
            b_tree.insert(&20);
            b_tree.insert(&5);
            b_tree.insert(&6);
            b_tree.insert(&12);
            b_tree.insert(&30);
            b_tree.insert(&7);
            b_tree.insert(&17);

            let root = b_tree.root.borrow();
            assert_eq!(root.childs.len(), 3);
            assert_eq!(root.values, vec![10, 20]);

            let left_node = root.childs[0].borrow();
            assert_eq!(left_node.values, vec![5, 6, 7]);
            assert_eq!(left_node.is_leaf, true);

            let middle_node = root.childs[1].borrow();
            assert_eq!(middle_node.values, vec![12, 17]);
            assert_eq!(middle_node.is_leaf, true);

            let right_node = root.childs[2].borrow();
            assert_eq!(right_node.values, vec![30]);
            assert_eq!(right_node.is_leaf, true);
        }

        #[test]
        fn it_should_store_all_values_correctly_for_degree_2() {
            let mut b_tree: BTree<i32> = BTree::new(2);
            b_tree.insert(&10);
            b_tree.insert(&20);
            b_tree.insert(&5);
            b_tree.insert(&6);
            b_tree.insert(&12);
            b_tree.insert(&30);
            b_tree.insert(&7);
            b_tree.insert(&17);

            let root = b_tree.root.borrow();
            assert_eq!(root.childs.len(), 2);
            assert_eq!(root.values, vec![10]);

            let left_node_0 = root.childs[0].borrow();
            assert_eq!(left_node_0.values, vec![6]);

            let left_node_1_0 = left_node_0.childs[0].borrow();
            assert_eq!(left_node_1_0.values, vec![5]);
            assert_eq!(left_node_1_0.is_leaf, true);

            let right_node_1_0 = left_node_0.childs[1].borrow();
            assert_eq!(right_node_1_0.values, vec![7]);
            assert_eq!(right_node_1_0.is_leaf, true);

            let right_node_0 = root.childs[1].borrow();
            assert_eq!(right_node_0.values, vec![20]);

            let left_node_1_1 = right_node_0.childs[0].borrow();
            assert_eq!(left_node_1_1.values, vec![12, 17]);
            assert_eq!(left_node_1_1.is_leaf, true);

            let right_node_1_1 = right_node_0.childs[1].borrow();
            assert_eq!(right_node_1_1.values, vec![30]);
            assert_eq!(right_node_1_1.is_leaf, true);
        }
    }
}