///给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
///
///  
///
/// 示例 1：
///
///
///
/// 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
/// 输出：6
/// 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
/// 示例 2：
///
/// 输入：height = [4,2,0,3,2,5]
/// 输出：9
///  
///
/// 提示：
///
/// n == height.length
/// 0 <= n <= 3 * 10^4
/// 0 <= height[i] <= 10^5
///
///
/// 核心精神：总体积减去柱子体积就是水的容量
/// 利用左右指针的下标差值计算出每一层雨水和柱子的体积。如下图，第一层体积为11，第二层为8，第三层为1。累加得到整体体积volume = 20（注意：每一层从左边第一个方格到右边最后一个方格之间一定是被蓝黑两种颜色的方格填满，不会存在空白，所以我们可以这么求值）
/// 计算柱子的总体积Sum，就是height：[0,1,0,2,1,0,1,3,2,1,2,1]数组之和14
/// 返回结果volume − Sum就是雨水的体积。
pub fn trap(height: Vec<i32>) -> i32 {
    // 柱子的体积
    let sum: i32 = height.iter().sum();
    //总体积和高度初始化
    let mut volume = 0i32;
    let mut hight = 1;
    let size = height.len();
    if size == 0 {
        return 0;
    }
    // 双指针初始化
    let mut left = 0;
    let mut right = size - 1;
    while left <= right {
        while left <= right && height[left] < hight {
            left += 1;
        }
        while left <= right && height[right] < hight {
            right -= 1;
        }
        // 每一层的容量都加起来
        volume += (right + 1 - left) as i32;
        // 高度加一
        hight += 1;
    }
    // 总体积减去柱子体积，即雨水总量
    volume - sum
}

#[test]
fn test() {
    println!("{}", trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
}
