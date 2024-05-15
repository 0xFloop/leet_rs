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
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
pub fn solve() -> Option<Rc<RefCell<TreeNode>>> {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                right: None,
                left: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: None,
                left: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                right: None,
                left: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                right: None,
                left: None,
            }))),
        }))),
    })));

    let mut root = root;
    let curr = &mut root;

    recurse_reverse(curr);
    root
}
fn recurse_reverse(node: &mut Option<Rc<RefCell<TreeNode>>>) {
    if node.is_none() {
        return;
    }

    let mut mut_node = node.as_ref().unwrap().as_ref().borrow_mut();
    let tmp = mut_node.left.take();

    mut_node.left = mut_node.right.take();
    mut_node.right = tmp;

    recurse_reverse(&mut mut_node.left);
    recurse_reverse(&mut mut_node.right);
}
