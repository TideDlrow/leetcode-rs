///给你两个有序整数数组 nums1 和 nums2，请你将 nums2 合并到 nums1 中，使 nums1 成为一个有序数组。
///
/// 初始化 nums1 和 nums2 的元素数量分别为 m 和 n 。你可以假设 nums1 的空间大小等于 m + n，这样它就有足够的空间保存来自 nums2 的元素。
///
///  
///
/// 示例 1：
///
/// 输入：nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
/// 输出：[1,2,2,3,5,6]
/// 示例 2：
///
/// 输入：nums1 = [1], m = 1, nums2 = [], n = 0
/// 输出：[1]
///  
///
/// 提示：
///
/// nums1.length == m + n
/// nums2.length == n
/// 0 <= m, n <= 200
/// 1 <= m + n <= 200
/// -10^9 <= nums1[i], nums2[i] <= 10^9
///
/// 法一：先把nums2放入nums1再进行排序
///
/// 法二：用一个新数组，每次从两个数组头部取出比较小的数字放到结果中。再把新数组中的元素复制到nums1中
///
/// 法三：由于nums1的后半段是空的，所以可以把法二的方法反过来，可以指针设置为从后向前遍历，每次取两者之中的较大者放进
/// nums1的最后面
///
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    //这里用法二
    let m = m as usize;
    let n = n as usize;
    let mut sorted = vec![0; (m + n)];
    let mut p1 = 0;
    let mut p2 = 0;
    let mut cur = 0;
    while p1 < m || p2 < n {
        if p1 == m {
            cur = nums2[p2];
            p2 += 1;
        } else if p2 == n {
            cur = nums1[p1];
            p1 += 1;
        } else if nums1[p1] < nums2[p2] {
            cur = nums1[p1];
            p1 += 1;
        } else {
            cur = nums2[p2];
            p2 += 1;
        }
        sorted[p1 + p2 - 1] = cur;
    }

    for i in 0..(m + n) {
        nums1[i] = sorted[i];
    }
}

#[test]
fn test() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:?}", nums1);
}
