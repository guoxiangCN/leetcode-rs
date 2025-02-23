/*
 * @lc app=leetcode.cn id=342 lang=rust
 *
 * [342] 4的幂
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // 4^0=1:   0000 0000 0001   1 lz: 31
        // 4^1=4:   0000 0000 0100   3 lz: 29
        // 4^2=16:  0000 0001 0000   5 lz: 27
        // 4^3=64:  0000 0100 0000   7 lz: 25
        // 4^4=256: 0001 0000 0000   9
        // ...
        let nbits = n.count_ones();
        let leading_zeros = n.leading_zeros();

        n > 0 && (nbits==1 && leading_zeros%2==1)
    }
}
// @lc code=end

