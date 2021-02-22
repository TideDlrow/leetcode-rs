#![feature(map_first_last)]

use std::collections::BTreeMap;

///给你一个整数数组 nums ，和一个表示限制的整数 limit，请你返回最长连续子数组的长度，该子数组中的任意两个元素之间的绝对差必须小于或者等于 limit 。
///
/// 如果不存在满足条件的子数组，则返回 0 。
///
///  
///
/// 示例 1：
///
/// 输入：nums = [8,2,4,7], limit = 4
/// 输出：2
/// 解释：所有子数组如下：
/// [8] 最大绝对差 |8-8| = 0 <= 4.
/// [8,2] 最大绝对差 |8-2| = 6 > 4.
/// [8,2,4] 最大绝对差 |8-2| = 6 > 4.
/// [8,2,4,7] 最大绝对差 |8-2| = 6 > 4.
/// [2] 最大绝对差 |2-2| = 0 <= 4.
/// [2,4] 最大绝对差 |2-4| = 2 <= 4.
/// [2,4,7] 最大绝对差 |2-7| = 5 > 4.
/// [4] 最大绝对差 |4-4| = 0 <= 4.
/// [4,7] 最大绝对差 |4-7| = 3 <= 4.
/// [7] 最大绝对差 |7-7| = 0 <= 4.
/// 因此，满足题意的最长子数组的长度为 2 。
/// 示例 2：
///
/// 输入：nums = [10,1,2,4,7,2], limit = 5
/// 输出：4
/// 解释：满足题意的最长子数组是 [2,4,7,2]，其最大绝对差 |2-7| = 5 <= 5 。
/// 示例 3：
///
/// 输入：nums = [4,2,2,2,4,4,2,2], limit = 0
/// 输出：3
///  
///
/// 提示：
///
/// 1 <= nums.length <= 10^5
/// 1 <= nums[i] <= 10^9
/// 0 <= limit <= 10^9
///
///
/// 题解：
/// 使用 left 和 right两个指针，分别指向滑动窗口的左右边界；定义 multiset(在这用的是TreeMap保存了出现次数) 保存滑动窗口的所有元素；
// right 主动右移：right 指针每次移动一步，把 A[right]A[right] 放入滑动窗口；
// left 被动右移：判断此时窗口内最大值和最小值的差，如果大于 limit，则 left 指针被迫右移，直至窗口内最大值和最小值的差小于等于 limitlimit 为止；leftleft 每次右移之前，需要把 A[left]A[left] 从 multiset 中减去一次。
// 滑动窗口长度的最大值就是所求。
//
// 作者：fuxuemingzhu
// 链接：https://leetcode-cn.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/solution/he-gua-de-shu-ju-jie-gou-hua-dong-chuang-v46j/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let len = nums.len();
    let mut m: BTreeMap<i32, i32> = BTreeMap::new();
    let (mut left, mut right) = (0, 0);
    let mut res = 0;
    while right < len {
        m.insert(nums[right],m.get(&nums[right]).unwrap_or(&0)+1);
        let (last, _) = m.iter().next_back().unwrap();
        let (first, _) = m.iter().next().unwrap();
        if last - first > limit {
            m.insert(nums[left], m.get(&nums[left]).unwrap() - 1);
            if let Some(0) = m.get(&nums[left]) {
                m.remove(&nums[left]);
            }
            left += 1;
        }
        res = std::cmp::max(res, right - left + 1);
        right += 1;
    }
    res as i32
}

#[test]
fn test() {
    println!("{}", longest_subarray(vec![8, 2, 4, 7], 4));
    println!("{}", longest_subarray(vec![10,1,2,4,7,2], 5));
    println!("{}", longest_subarray(vec![4,2,2,2,4,4,2,2], 0));
}
