mod solution;
use crate::solution::Solution;
use crate::solution::ListNode;

fn main() {
    let input = vec![
        Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 5, next: None })) })) })),
        Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode { val: 4, next: None })) })) })),
        Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 6, next: None })) })),
    ];
    let merged_list = Solution::merge_k_lists(input);
    if let Some(mut node) = merged_list {
        print!("{}", node.val);
        while let Some(next) = node.next.take() {
            print!(", {}", next.val);
            node = Box::new(*next);
        }
        println!();
    } else {
        println!("None");
    }
}
