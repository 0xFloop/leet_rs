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

pub fn solve() -> u32 {
    let head = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        })),
    }));
    let mut t = Vec::new();
    t.push(head.as_ref().unwrap().val);

    let mut curr = head;

    while let Some(node) = curr.unwrap().next {
        println!("{:?}", node);
        t.push(node.val);
        curr = Some(node);
    }
    println!("{:?}", t);

    let d = t.clone();
    t.reverse();
    println!("{:?}", d == t);

    0
}
