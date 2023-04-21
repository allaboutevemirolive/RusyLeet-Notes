mod regular_expression_matching3127379;

use regular_expression_matching3127379::Solution;

fn main() {
    let s = String::from("aa");
    let p = String::from("a*");
    let result = Solution::is_match(s, p);
    println!("{}", result);
}
