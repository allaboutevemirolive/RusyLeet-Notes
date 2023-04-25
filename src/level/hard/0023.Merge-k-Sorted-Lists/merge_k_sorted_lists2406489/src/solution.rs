#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}


pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // Collect all values from all nodes in all lists.
        let mut vals = Vec::new();
        lists.iter().for_each(|list| {
            let mut head = list.as_ref();
            while let Some(node) = head {
                vals.push(node.val);
                head = node.next.as_ref();
            }
        });
        vals.sort_unstable();

        // Build the returned list in reverse, starting with None.
        let mut head = None;
        vals.into_iter().rev().for_each(|val| {
            head = Some(Box::new(ListNode { val, next: head.take() }));
        });
        head
    }
}
