/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut tmp_map = std::collections::HashMap::new();
        for (idx, val) in nums.iter().enumerate() {
            tmp_map.insert(*val, idx);
        }
        for (idx, val) in nums.iter().enumerate() {
            let search = target-*val;
            if let Some(id2) = tmp_map.get(&search) {
                if (idx != *id2) {
                    return vec![idx as i32, *id2 as i32];                   
                }
            }
        }
        vec![]
    }
}
// @lc code=end

