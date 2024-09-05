//link to problem: https://leetcode.com/problems/swap-nodes-in-pairs/

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

impl Iterator for ListNode {
    type Item = Box<ListNode>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take()
    }
}
pub fn solve() -> Option<Box<ListNode>> {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    }));
    let mut a = Some(Box::new(ListNode::new(1)));
    let mut b = Some(Box::new(ListNode::new(2)));

    a.as_mut().unwrap().next = b.clone();
    b.as_mut().unwrap().next = a;

    let mut head = head;

    let mut curr = &mut head;

    while let Some(node) = curr {
        if node.next.is_none() {
            return head;
        }

        let mut tmp = node.next.as_mut().unwrap().next.clone();

        std::mem::swap(&mut node.next, &mut tmp);

        tmp.as_mut().unwrap().next = Some(Box::new(*node.clone()));

        std::mem::swap(node, tmp.as_mut().unwrap());

        curr = &mut node.next.as_mut().unwrap().next;
    }
    head
}
