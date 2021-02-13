///给定一个范围在1 ≤ a[i] ≤ n (n = 数组大小 ) 的 整型数组，数组中的元素一些出现了两次，另一些只出现一次。
///
/// 找到所有在 [1, n] 范围之间没有出现在数组中的数字。
///
/// 您能在不使用额外空间且时间复杂度为O(n)的情况下完成这个任务吗? 你可以假定返回的数组不算在额外空间内。
///
/// 示例:
/// ```
/// 输入:
/// [4,3,2,7,8,2,3,1]
///
/// 输出:
/// [5,6]
/// ```
pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    ///将所有正数作为数组下标，置对应数组值为负值。那么，仍为正数的位置即为（未出现过）消失的数字。
    ///
    /// 举个例子：
    ///
    /// 原始数组：[4,3,2,7,8,2,3,1]
    ///
    /// 重置后为：[-4,-3,-2,-7,8,2,-3,-1]
    ///
    /// 结论：[8,2] 分别对应的index为[5,6]（消失的数字）
    let mut res = Vec::new();
    let mut nums = nums;
    for i in 0..nums.len() {
        let num = nums[i];
        let num = (num.abs() - 1) as usize;
        nums[num] = -nums[num].abs();
    }

    for i in 0..nums.len() {
        if nums[i] > 0 {
            res.push((i + 1) as i32);
        }
    }
    res
}

#[test]
fn test() {
    println!("{:?}", find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]));
}
