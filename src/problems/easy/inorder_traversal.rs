use std::{cell::RefCell, rc::Rc, slice::ChunksMut};

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

pub fn solve() -> Vec<i32> {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                right: None,
                left: None,
            }))),
            right: None,
        }))),
    })));
    if root.is_none() {
        return vec![];
    }

    let mut tracker = Vec::new();

    search_node(&root.unwrap(), &mut tracker);

    tracker
}

fn search_node(curr_node: &Rc<RefCell<TreeNode>>, tracker_vec: &mut Vec<i32>) {
    let node = curr_node.borrow();
    if let Some(l_node) = &node.left {
        search_node(l_node, tracker_vec)
    }
    tracker_vec.push(node.val);
    if let Some(r_node) = &node.right {
        search_node(r_node, tracker_vec)
    }
}
