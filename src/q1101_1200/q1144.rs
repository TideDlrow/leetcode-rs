///给你一个整数数组 nums，每次 操作 会从中选择一个元素并 将该元素的值减少 1。
///
/// 如果符合下列情况之一，则数组 A 就是 锯齿数组：
///
/// 每个偶数索引对应的元素都大于相邻的元素，即`A[0] > A[1] < A[2] > A[3] < A[4] > ...`
/// 或者，每个奇数索引对应的元素都大于相邻的元素，即`A[0] < A[1] > A[2] < A[3] > A[4] < ...`
/// 返回将数组`nums`转换为锯齿数组所需的最小操作次数。
///
///
/// 示例 1：
/// ```
/// 输入：nums = [1,2,3]
/// 输出：2
/// 解释：我们可以把 2 递减到 0，或把 3 递减到 1。
/// ```
/// 示例 2：
/// ```
/// 输入：nums = [9,6,1,6,2]
/// 输出：4
/// ```
///
/// 提示：
/// ```
/// 1 <= nums.length <= 1000
/// 1 <= nums[i] <= 1000
/// ```
pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
    //思路：根据题意，我们遍历数组，分两种情况讨论，
    // 要么将奇数位置的元素减少到刚好比相邻的偶数位置元素小，
    // 要么将偶数位置的元素减少到刚好比相邻的奇数位置元素小。
    // 返回两种情况操作较少的作为答案。
    use std::cmp::max;
    use std::cmp::min;
    if nums.len() < 2 {
        return 0;
    }
    let mut odd_count = 0;
    let mut even_count = 0;
    let len = nums.len();
    for i in 0..len {
        //当前位置左侧的数字
        let left_num = if i > 0 {
            nums[i - 1]
        } else {
            i32::MAX
        };
        let right_num = if i < len - 1 {
            nums[i + 1]
        } else {
            i32::MAX
        };
        if i & 1 == 1 {
            //奇数位置
            odd_count += max(0, nums[i] - min(left_num, right_num) + 1);
        } else {
            even_count += max(0, nums[i] - min(left_num, right_num) + 1);
        }
    }
    min(odd_count, even_count)
}
