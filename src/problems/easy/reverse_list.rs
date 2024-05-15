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
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));

    // my incredibly naive approach because swapping links within the first loop was driving me mad
    // let mut nodes: Vec<Option<Box<ListNode>>> = Vec::new();
    //
    // let mut head = head;
    //
    // let mut curr = &mut head;
    //
    // loop {
    //     if curr.is_none() {
    //         break;
    //     }
    //
    //     nodes.push(curr.clone());
    //
    //     curr = &mut curr.as_mut().unwrap().next;
    // }
    //
    // let mut new = Some(Box::new(ListNode::new(-1)));
    // let mut walk = &mut new;
    //
    // for node in nodes.into_iter().rev() {
    //     walk.as_mut().unwrap().next = node;
    //     walk = &mut walk.as_mut().unwrap().next;
    // }
    // walk.as_mut().unwrap().next = None;
    //
    // new.unwrap().next

    //better solution that swaps links in first loop
    let mut prev = None;
    let mut curr = head;

    while let Some(mut node) = curr {
        let next = node.next.take();
        node.next = prev.take();
        prev = Some(node);
        curr = next;
    }

    prev
}
