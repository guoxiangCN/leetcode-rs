/*
 * @lc app=leetcode.cn id=191 lang=rust
 *
 * [191] 位1的个数
 */

// @lc code=start
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut res = 0;
        let un = n as u32;

        for shift in 0..32 {
            if (un >> shift) & 1 == 1 {
                res += 1;
            }
        }
        return res;
    }
}
// @lc code=end

