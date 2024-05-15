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
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));

    let mut h = 0;

    let curr = &root;

    get_height(curr, &mut h);

    let max_count = 2i32.pow(h as u32) - 1;

    let mut missing = 0;

    let curr = &root;

    count_missing(curr, h, 0, &mut missing);

    max_count - missing
}

fn count_missing(
    node: &Option<Rc<RefCell<TreeNode>>>,
    max_height: i32,
    curr_height: i32,
    missing: &mut i32,
) {
    if node.is_none() {
        if curr_height != max_height {
            *missing += 1;
        }
        return;
    }

    let left = &node.as_ref().unwrap().as_ref().borrow().left;
    let right = &node.as_ref().unwrap().as_ref().borrow().right;

    count_missing(right, max_height, curr_height + 1, missing);

    count_missing(left, max_height, curr_height + 1, missing);
}

fn get_height(node: &Option<Rc<RefCell<TreeNode>>>, height: &mut i32) {
    if node.is_none() {
        return;
    }
    *height += 1;

    let left = &node.as_ref().unwrap().as_ref().borrow().left;

    get_height(left, height);
}
