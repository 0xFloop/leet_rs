//Definition for singly-linked list.
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
    let list1: Option<Box<ListNode>> = None;

    let list2 = Some(Box::new(ListNode { val: 0, next: None }));

    let mut merged_list = ListNode::new(0);
    let mut curr_list = &mut merged_list;
    let mut l1 = list1;
    let mut l2 = list2;

    while l1.is_some() || l2.is_some() {
        let mut val1 = i32::MAX;
        let mut val2 = i32::MAX;
        if let Some(node) = &l1 {
            println!("currently l1 val: {}", node.val);
            val1 = node.val;
        }
        if let Some(node) = &l2 {
            println!("currently l2 val: {}", node.val);
            val2 = node.val;
        }
        match val1.cmp(&val2) {
            std::cmp::Ordering::Greater => {
                println!("l1 val greater: {}", val1);

                curr_list.next = l2.clone();
                curr_list = curr_list.next.as_mut().unwrap();
                l2 = l2.unwrap().next;
            }
            std::cmp::Ordering::Less => {
                println!("l2 val greater: {}", val2);
                curr_list.next = l1.clone();
                curr_list = curr_list.next.as_mut().unwrap();
                l1 = l1.unwrap().next;
            }
            std::cmp::Ordering::Equal => {
                println!("values equal: val1: {} val2: {}", val1, val2);

                curr_list.next = l1.clone();
                curr_list = curr_list.next.as_mut().unwrap();
                curr_list.next = l2.clone();
                curr_list = curr_list.next.as_mut().unwrap();
                l1 = l1.unwrap().next;
                l2 = l2.unwrap().next;
            }
        };
        println!("Curr list: {:?}", &curr_list);
    }
    merged_list.next
}
