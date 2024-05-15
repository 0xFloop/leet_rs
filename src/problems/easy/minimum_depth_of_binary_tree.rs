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
pub fn solve() -> i32 {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));

    if root == None {
        return 0;
    }
    let mut shortest = i32::MAX;
    recurse(&root, &mut 1, &mut shortest);
    shortest
}

fn recurse(curr_node: &Option<Rc<RefCell<TreeNode>>>, curr_depth: &mut i32, shortest: &mut i32) {
    let curr = curr_node.as_ref().unwrap().as_ref().borrow();
    if curr.left.is_none() && curr.right.is_none() {
        if curr_depth < shortest {
            *shortest = *curr_depth;
        }
        return;
    }
    *curr_depth += 1;
    if curr.left.is_some() {
        recurse(&curr.left, curr_depth, shortest);
    }
    if curr.right.is_some() {
        recurse(&curr.right, curr_depth, shortest);
    }
    *curr_depth -= 1;
}
