mod median_of_two_sorted_arrays468977;

use median_of_two_sorted_arrays468977::Solution;

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("The median is: {}", result);
}
