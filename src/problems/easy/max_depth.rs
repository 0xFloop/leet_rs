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

pub fn solve() -> i32 {
    let head = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                right: None,
                left: None,
            }))),
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: None,
                left: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: None,
                left: None,
            }))),
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                right: None,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    right: None,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        right: None,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            right: None,
                            left: None,
                        }))),
                    }))),
                }))),
            }))),
        }))),
    })));
    search(&head)
}
fn search(node: &WrappedTreeNode) -> i32 {
    if node.is_none() {
        return 0;
    }

    let l_val = search(&node.as_ref().unwrap().as_ref().borrow().left) + 1;
    let r_val = search(&node.as_ref().unwrap().as_ref().borrow().right) + 1;

    return i32::max(l_val, r_val);
}
