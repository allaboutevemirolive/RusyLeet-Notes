// https://leetcode.com/problems/regular-expression-matching/solutions/3127379/regex-computation-in-rust/

pub struct Solution;


// We minus 1 to get the initial index of the string s and p. 
// Because we start from 0 in calculating the dp array(matrix).

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        // i will iterate from 0 to s.len() + 1.
        for i in 0..=s.len() {
            // j will iterate from 1 to p.len() + 1.
            for j in 1..=p.len() {
                if p[j - 1] == b'*' {
                    // Check if 2 initial positions before the current position is true.
                    dp[i][j] = dp[i][j - 2]
                         // or if there is a row above the current position,
                        || (i > 0 
                            // and the current position for the row above is true,
                            && dp[i - 1][j] 
                            // and the current char of the string s is equal to char before the current char of the string p
                            && (s[i - 1] == p[j - 2] 
                                // or the initial position before char * is a dot.
                                || p[j - 2] == b'.'));
                } else {
                    dp[i][j] =
                        // Check if there is a row above the current position.
                        i > 0 
                        // and the row before minus 1 is true.
                        && dp[i - 1][j - 1] 
                        // and the current char of the string s is equal to the current char of the string p
                        && (s[i - 1] == p[j - 1] 
                            // or the current char of the string p is a dot.
                            || p[j - 1] == b'.');
                }
            }
        }
        dp[s.len()][p.len()]
    }
}
