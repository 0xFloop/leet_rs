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
use std::cmp::max;
use std::rc::Rc;

pub fn solve() -> bool {
    let root_2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    })));

    //traverse it an tally the heights
    let curr_node = root;

    if curr_node.is_none() {
        return true;
    }

    if get_height(&curr_node).is_err() {
        return false;
    }
    true
}

fn get_height(curr_node: &Option<Rc<RefCell<TreeNode>>>) -> Result<i32, &str> {
    if curr_node.is_none() {
        return Ok(0);
    }

    let Ok(node_l) = get_height(&curr_node.as_ref().unwrap().as_ref().borrow().left) else {
        return Err("unbalanced node");
    };

    let Ok(node_r) = get_height(&curr_node.as_ref().unwrap().as_ref().borrow().right) else {
        return Err("Unbalanced node");
    };

    let bf = node_l - node_r;

    if !(-1..=1).contains(&bf) {
        return Err("Unbalanced node");
    }
    Ok(1 + max(node_l, node_r))
}
