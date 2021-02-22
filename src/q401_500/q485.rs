///给定一个二进制数组， 计算其中最大连续1的个数。
///
/// 示例 1:
/// ```
/// 输入: [1,1,0,1,1,1]
/// 输出: 3
/// 解释: 开头的两位和最后的三位都是连续1，所以最大连续1的个数是 3.
/// ```
/// 注意：
/// ```
/// 输入的数组只包含0 和1。
/// 输入数组的长度是正整数，且不超过 10,000。
/// ```
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_count = 0;
    let mut count = 0;
    for num in nums {
        if num == 1 {
            count += 1;
        } else {
            max_count = std::cmp::max(count, max_count);
            count = 0;
        }
    }
    std::cmp::max(count, max_count)
}

#[test]
fn test() {
    println!("{}", find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]));
}
