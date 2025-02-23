/*
 * @lc app=leetcode.cn id=461 lang=rust
 *
 * [461] 汉明距离
 */

// @lc code=start
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let a = x as u32;
        let b = y as u32;

        let mut res = 0;
        for nbit in 0..32 {
            let mask = 1_u32 << nbit;
            if (a & mask) != (b & mask) {
                res+=1;
            }
        }
        res
    }
}
// @lc code=end

