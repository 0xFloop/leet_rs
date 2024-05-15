// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type WrappedTreeNode = Option<Rc<RefCell<TreeNode>>>;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
pub fn solve() -> bool {
    let p = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            right: None,
            left: None,
        }))),
        right: None,
    })));

    let q = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            right: None,
            left: None,
        }))),
    })));
    compare_trees(&p, &q)
}

fn compare_trees(p: &WrappedTreeNode, q: &WrappedTreeNode) -> bool {
    println!("p: {:#?}, q: {:#?}", p, q);
    if (p.is_some() && q.is_none()) || (p.is_none() && q.is_some()) {
        return false;
    }
    if p.is_none() && q.is_none() {
        return true;
    }
    let p_node = p.as_ref().unwrap().as_ref().borrow();
    let q_node = q.as_ref().unwrap().as_ref().borrow();
    println!("p_node.val: {}, q_node.val: {}", p_node.val, q_node.val);

    if q_node.val != p_node.val {
        return false;
    }

    return compare_trees(&p_node.left, &q_node.left)
        && compare_trees(&p_node.right, &q_node.right);
}
