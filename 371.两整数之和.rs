/*
 * @lc app=leetcode.cn id=371 lang=rust
 *
 * [371] 两整数之和
 */

// @lc code=start
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        // 给你两个整数 a 和 b ，不使用 运算符 + 和 - ​​​​​​​，计算并返回两整数之和

        // let mut result: i32;
        // unsafe {
        //     std::arch::asm!(
        //         "add {0}, {1}",
        //         inout(reg) a => result,
        //         in(reg) b,
        //     );
        // }
        // result

        let mut a = a;
        let mut b = b;

        while b != 0 {
            let carry = a & b; // 计算进位
            a = a ^ b; // 计算无进位的和
            b = carry << 1; // 将进位左移一位
        }

        a
    }
}
// @lc code=end
