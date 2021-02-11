use std::collections::HashMap;
use std::option::Option::Some;

///给定一个正整数数组 A，如果 A的某个子数组中不同整数的个数恰好为 K，则称 A 的这个连续、不一定独立的子数组为好子数组。
///
/// （例如，[1,2,3,1,2] 中有3个不同的整数：1，2，以及3。）
///
/// 返回A中好子数组的数目。
///
///
/// 示例 1：
/// ```
/// 输入：A = [1,2,1,2,3], K = 2
/// 输出：7
/// 解释：恰好由 2 个不同整数组成的子数组：[1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2].
/// ```
/// 示例 2：
/// ```
/// 输入：A = [1,2,1,3,4], K = 3
/// 输出：3
/// 解释：恰好由 3 个不同整数组成的子数组：[1,2,1,3], [2,1,3], [1,3,4].
/// ```
/// 提示：
/// ```
/// 1 <= A.length <= 20000
/// 1 <= A[i] <= A.length
/// 1 <= K <= A.length
/// ```
pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
    sub_arrays_with_most_k(&a, k) - sub_arrays_with_most_k(&a, k - 1)
}

fn sub_arrays_with_most_k(nums: &Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::<i32, i32>::new();
    let mut count = 0;
    let mut res = 0;
    let mut left = 0;
    let mut right = 0;
    while right < nums.len() {
        map.insert(nums[right], map.get(&nums[right]).or(Some(&0)).unwrap() + 1);
        if let Some(&1) = map.get(&nums[right]) {
            count += 1;
        }
        right += 1;
        while count > k {
            map.insert(nums[left], map.get(&nums[left]).unwrap() - 1);
            if let Some(&0) = map.get(&nums[left]) {
                count -= 1;
            }
            left += 1;
        }
        res += right - left + 1;
    }
    res as i32
}

#[test]
fn test() {
    println!("{}", subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2));
    println!("{}", subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3));
}
