use std::{borrow::Borrow, cell::RefCell, rc::Rc};

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
pub fn solve() -> u32 {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
    })));

    let mut path_list: Vec<String> = Vec::new();

    traverse(&root, &mut path_list, &mut String::new());
    println!("{:?}", path_list);
    0
}

pub fn traverse(
    curr_node: &Option<Rc<RefCell<TreeNode>>>,
    path_list: &mut Vec<String>,
    curr_path: &mut String,
) {
    let curr = curr_node.as_ref().unwrap().as_ref().borrow();

    if curr_path.is_empty() {
        curr_path.push_str(&curr.val.to_string());
    } else {
        let mut addr = "->".to_owned();
        addr.push_str(&curr.val.to_string());
        curr_path.push_str(&addr);
    }

    if curr.left.is_none() && curr.right.is_none() {
        path_list.push(curr_path.clone());
        return;
    }
    if curr.left.is_some() {
        traverse(&curr.left, path_list, &mut curr_path.clone());
    }
    if curr.right.is_some() {
        traverse(&curr.right, path_list, curr_path);
    }
}
