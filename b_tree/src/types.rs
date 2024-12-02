use crate::common_node::CommonNode;
use crate::leaf_node::LeafNode;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum TreeNode<K, V> {
    Common(CommonNode<K, V>),
    Leaf(LeafNode<K, V>),
}

pub enum InsertType<K, V> {
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
