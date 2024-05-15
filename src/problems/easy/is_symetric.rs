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
                left: None,
            }))),
        }))),
    })));
    let l_node = &p.as_ref().unwrap().as_ref().borrow().left;
    let r_node = &p.as_ref().unwrap().as_ref().borrow().right;

    cmp_nodes(l_node, r_node)
}

fn cmp_nodes(l_node: &WrappedTreeNode, r_node: &WrappedTreeNode) -> bool {
    if (l_node.is_some() && r_node.is_none()) || (l_node.is_none() && r_node.is_some()) {
        return false;
    }
    if l_node.is_none() && r_node.is_none() {
        return true;
    }
    let l = l_node.as_ref().unwrap().as_ref().borrow();
    let r = r_node.as_ref().unwrap().as_ref().borrow();
    println!("l.val: {}, r.val: {}", l.val, r.val);

    if l.val != r.val {
        return false;
    }

    return cmp_nodes(&l.left, &r.right) && cmp_nodes(&l.right, &r.left);
}
