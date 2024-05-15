// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn solve() -> Option<Box<ListNode>> {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        })),
    }));
    let mut head = head.clone();

    let mut current_node = &mut head;
    while let Some(node) = current_node {
        while let Some(next) = &mut node.next {
            if next.val == node.val {
                node.next = next.next.take();
            } else {
                break;
            }
        }
        current_node = &mut node.next;
    }
    head
}
pub fn naive_solve() -> Option<Box<ListNode>> {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        })),
    }));
    let mut ret_list = ListNode::new(0);

    let mut head = head;

    let mut curr_node = &mut ret_list;

    while head.is_some() {
        let mut curr_val = i32::MAX;
        let mut next_val = i32::MAX - 1;
        if let Some(node) = &head {
            curr_val = node.val;
        }
        if let Some(next_node) = &head.as_mut().unwrap().next {
            next_val = next_node.val;
        }
        if !(next_val == curr_val) {
            println!("values do not match");
            curr_node.next = head.clone();
            curr_node = curr_node.next.as_mut().unwrap();
        } else {
            println!("values match");
        }

        head = head.unwrap().next
    }

    ret_list.next
}
