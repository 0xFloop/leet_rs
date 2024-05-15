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
use std::cell::RefCell;
use std::rc::Rc;

pub fn solve() -> Option<Rc<RefCell<TreeNode>>> {
    let nums = vec![-10, -3, 0, 5, 9];
    recurse(&nums)
}

fn recurse(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    println!("{nums:?}");
    if nums.is_empty() {
        return None;
    }
    let (left, right) = nums.split_at(nums.len() / 2);
    let (curr, right) = right.split_first().unwrap();

    Some(Rc::new(RefCell::new(TreeNode {
        val: *curr,
        right: recurse(right),
        left: recurse(left),
    })))
}
