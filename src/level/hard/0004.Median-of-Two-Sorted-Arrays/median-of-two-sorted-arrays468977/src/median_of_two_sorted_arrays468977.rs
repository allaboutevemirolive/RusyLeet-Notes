// https://leetcode.com/problems/median-of-two-sorted-arrays/solutions/468977/rust-4ms/

// sort_unstable() is a stable sort that does not allocate memory.
// pattern-defeating quicksort by Orson Peters
// https://github.com/orlp/pdqsort
pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums: Vec<i32> = vec![];

        nums.extend_from_slice(&nums1);
        println!("nums1: {:?}", nums); // prints nums1 to the terminal

        nums.extend_from_slice(&nums2);
        println!("nums2: {:?}", nums); // prints nums2 to the terminal

        nums.sort_unstable();
        println!("sorted nums: {:?}", nums); // prints sorted nums to the terminal

        let len: usize = nums.len();
        if len % 2 == 1 {
            return nums[len / 2] as f64;
        } else {
            return (nums[len / 2] + nums[len / 2 - 1]) as f64 / 2.0;
        }
    }
}
