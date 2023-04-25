pub(crate) struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        let mut count = 0;
        while tail.is_some() && count < k {
            tail = &mut tail.as_mut().unwrap().next;
            count += 1;
        }
        if count == k {
            let mut next = Self::reverse_k_group(tail.take(), k);
            while count > 0 {
                count -= 1;
                let mut node = head.take();
                head = node.as_mut().unwrap().next.take();
                node.as_mut().unwrap().next = next.take();
                next = node;
            }
            head = next;
        }
        head
    }
}
