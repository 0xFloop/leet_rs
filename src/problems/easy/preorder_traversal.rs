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
pub fn solve() -> Vec<i32> {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));

    let mut order = Vec::new();
    recurse(&root, &mut order);
    order
}

fn recurse(node: &Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
    if node.is_none() {
        return;
    }
    let node = node.as_ref().unwrap().as_ref().borrow();

    order.push(node.val);
    recurse(&node.left, order);
    recurse(&node.right, order);
}
