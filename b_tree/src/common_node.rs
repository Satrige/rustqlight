use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::{InsertType, TreeNode};

#[derive(Debug)]
pub struct CommonNode<K, V> {
    degree: usize,
    keys: Vec<K>,
    childs: Vec<Rc<RefCell<TreeNode<K, V>>>>,
}

impl<K: Ord + Copy, V: Clone> CommonNode<K, V> {
    pub fn new(
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

        let median_key = self.keys.pop().unwrap();

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

    pub fn insert(&mut self, new_key: &K, new_value: &V) -> InsertType<K, V> {
        let new_key_idx = self.keys.binary_search(new_key).unwrap_or_else(|idx| idx);

        let insert_result = match *self.childs[new_key_idx].borrow_mut() {
            TreeNode::Common(ref mut borrowed_common_child) => {
                borrowed_common_child.insert(new_key, new_value)
            }
            TreeNode::Leaf(ref mut borrowed_leaf_child) => {
                borrowed_leaf_child.insert(new_key, new_value)
            }
        };

        match insert_result {
            InsertType::Normal => insert_result,
            InsertType::Split(new_median_key, left_node_ref, right_node_ref) => {
                self.keys.insert(new_key_idx, new_median_key);
                self.childs.splice(
                    new_key_idx..=new_key_idx,
                    vec![left_node_ref, right_node_ref],
                );

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

    pub fn get_keys(&self) -> &Vec<K> {
        &self.keys
    }

    pub fn get_childs(&self) -> &Vec<Rc<RefCell<TreeNode<K, V>>>> {
        &self.childs
    }
}
