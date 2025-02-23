/*
 * @lc app=leetcode.cn id=136 lang=rust
 *
 * [136] 只出现一次的数字
 * 给你一个 非空 整数数组 nums ，除了某个元素只出现一次以外，其余每个元素均出现两次。
 * 找出那个只出现了一次的元素。
 * 
 * 你必须设计并实现线性时间复杂度的算法来解决此问题，且该算法只使用常量额外空间。
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        single_number_v1(nums)
    }


    // 方法1: 使用异或
    pub fn single_number_v1(nums: Vec<i32>) -> i32 {
        // x ^ 0 = x
        // x ^ x = 0
        let mut res = 0;
        nums.iter().for_each(|x| res ^= *x);
        res
    }
}
// @lc code=end

