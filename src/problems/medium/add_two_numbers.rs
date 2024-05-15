use core::panic;
use std::ops::Add;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        if next.is_some() {
            return ListNode { next, val };
        }
        return ListNode { next: None, val };
    }
}
pub fn solve() -> Option<Box<ListNode>> {
    let l1 = Some(Box::new(ListNode::new(
        2,
        Some(Box::new(ListNode::new(
            4,
            Some(Box::new(ListNode::new(3, None))),
        ))),
    )));
    let l2 = Some(Box::new(ListNode::new(
        5,
        Some(Box::new(ListNode::new(
            6,
            Some(Box::new(ListNode::new(4, None))),
        ))),
    )));
    println!("hehe");

    let mut l1 = l1;
    let mut l2 = l2;

    let mut ret_list = ListNode::new(0, None);
    let mut curr_node = &mut ret_list;

    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry > 0 {
        let mut value = carry;

        if let Some(node) = l1 {
            value += node.val;
            l1 = node.next;
        }
        if let Some(node) = l2 {
            value += node.val;
            l2 = node.next;
        }

        if value >= 10 {
            carry = 1;
            value -= 10;
        } else {
            carry = 0;
        }

        curr_node.next = Some(Box::new(ListNode::new(value, None)));
        curr_node = curr_node.next.as_mut().unwrap();
    }
    ret_list.next
}
// pub fn naive_solve() -> Option<Box<ListNode>> {
//     let l1 = ListNode::new(2, ListNode::new(4, ListNode::new(3, None)));
//     let l2 = ListNode::new(5, ListNode::new(6, ListNode::new(4, None)));
//
//     // let l1 = Some(Box::new(ListNode { val: 9, next: None }));
//     // let l2 = Some(Box::new(ListNode { val: 1, next: None }));
//     let mut l1_val_str = String::new();
//     let mut l2_val_str = String::new();
//
//     let mut curr_node = l1.clone();
//
//     while let Some(new_node) = curr_node {
//         println!("{:?}", new_node.val);
//         l1_val_str.push_str(&new_node.val.to_string());
//         curr_node = new_node.next;
//     }
//
//     curr_node = l2.clone();
//
//     while let Some(new_node) = curr_node {
//         l2_val_str.push_str(&new_node.val.to_string());
//         curr_node = new_node.next;
//     }
//
//     let mut l1_rev_str = String::new();
//     l1_val_str
//         .chars()
//         .into_iter()
//         .rev()
//         .for_each(|c| l1_rev_str.push_str(&c.to_string()));
//
//     let mut l2_rev_str = String::new();
//
//     l2_val_str
//         .chars()
//         .rev()
//         .for_each(|c| l2_rev_str.push_str(&c.to_string()));
//
//     let sum: String =
//         (l1_rev_str.parse::<u32>().unwrap() + l2_rev_str.parse::<u32>().unwrap()).to_string();
//
//     let ret_list = sum.chars().rev().fold(None, |mut curr, c| {
//         if curr.is_none() {
//             Some(Box::new(ListNode {
//                 val: c.to_string().parse::<i32>().unwrap(),
//                 next: None,
//             }))
//         } else {
//             //this neesds to get sorted out
//             //
//             let mut current = &mut curr;
//
//             loop {
//                 match current.as_mut().unwrap().next {
//                     Some(_) => current = &mut current.as_mut().unwrap().next,
//                     None => break,
//                 }
//             }
//
//             current.as_mut().unwrap().next = Some(Box::new(ListNode {
//                 val: c.to_string().parse::<i32>().unwrap(),
//                 next: None,
//             }));
//
//             curr
//         }
//     });
//     println!("ret list: {:?}", ret_list);
//
//     ret_list
// }
