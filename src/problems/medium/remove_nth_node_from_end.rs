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
        next: Some(Box::new(ListNode { val: 2, next: None })),
    }));
    let n = 2;

    let mut head = head;
    let mut len = 0;
    let mut curr = &head;
    while let Some(_) = curr {
        len += 1;
        curr = &curr.as_ref().unwrap().next;
    }
    println!("len {len}");

    if len < 2 {
        return None;
    }
    if len == n {
        return head.unwrap().next;
    }

    let node_remove_idx = len - n;

    let mut curr = &mut head;
    let mut count = 0;
    while let Some(node) = curr {
        if count == node_remove_idx {
            node.next = node.next.as_mut().unwrap().next.take();
        } else {
            curr = &mut curr.as_mut().unwrap().next;
        }
        count += 1;
    }
    head
}
