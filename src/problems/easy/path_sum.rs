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

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub fn solve() -> bool {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
        }))),
    })));

    let target_sum = 22;

    if root == None {
        return false;
    }
    recurse(&root, &mut 0, target_sum)
}

fn recurse(curr_node: &Option<Rc<RefCell<TreeNode>>>, curr_sum: &mut i32, target: i32) -> bool {
    let curr = curr_node.as_ref().unwrap().as_ref().borrow();

    *curr_sum += curr.val;

    if curr.left.is_none() && curr.right.is_none() && curr_sum == &target {
        return true;
    }

    if curr.left.is_some() && recurse(&curr.left, curr_sum, target) {
        return true;
    }

    if curr.right.is_some() && recurse(&curr.right, curr_sum, target) {
        return true;
    }

    *curr_sum -= curr.val;
    false
}
