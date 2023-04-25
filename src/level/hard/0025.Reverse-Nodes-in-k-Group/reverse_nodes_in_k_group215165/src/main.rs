mod solution;

use solution::Solution;
use std::boxed::Box;

use crate::solution::ListNode;

fn main() {
    let list = Some(Box::new(ListNode {
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

    let k = 2;
    if let Some(node) = Solution::reverse_k_group(list, k) {
        let mut current = Some(node);
        while let Some(n) = current {
            print!("{}, ", n.val);
            current = n.next;
        }
    }
}
