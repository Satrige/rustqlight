use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::{InsertType, TreeNode};

pub struct LeafNode<K, V> {
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

    pub fn insert(&mut self, new_key: &K, new_value: &V) -> InsertType<K, V> {
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

    pub fn get_keys(&self) -> &Vec<K> {
        &self.keys
    }

    pub fn get_values(&self) -> &Vec<V> {
        &self.values
    }
}
