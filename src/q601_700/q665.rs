///给你一个长度为n的整数数组,请你判断在 最多 改变1个元素的情况下,该数组能否变成一个非递减数列。
///
/// 我们是这样定义一个非递减数列的：对于数组中所有的i (0 <= i <= n-2),总满足 nums[i] <= nums[i + 1]。
///
///
/// 示例 1:
/// ```
/// 输入: nums = [4,2,3]
/// 输出: true
/// 解释: 你可以通过把第一个4变成1来使得它成为一个非递减数列。
/// ```
/// 示例 2:
/// ```
/// 输入: nums = [4,2,1]
/// 输出: false
/// 解释: 你不能在只改变一个元素的情况下将其变为非递减数列。
/// ```
///
/// 说明：
/// ```
/// 1 <= n <= 10 ^ 4
/// - 10 ^ 5<= nums[i] <= 10 ^ 5
/// ```
/// 来源：力扣（LeetCode）
/// 链接：https:///leetcode-cn.com/problems/non-decreasing-array
/// 著作权归领扣网络所有。商业转载请联系官方授权,非商业转载请注明出处。
pub fn check_possibility(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let mut count = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            count += 1;
            if i > 0 && nums[i - 1] > nums[i + 1] {
                nums[i + 1] = nums[i];
            }
        }
    }
    count < 2
}

pub fn check(nums: Vec<i32>) -> bool {
    let max = *nums.iter().max().unwrap();
    let mut index = 0;
    for i in 1..nums.len() {
        if nums[i] < nums[i - 1] {
            index = i;
            if nums[i - 1] != max {
                return false;
            }
        }
        if nums[index] > nums[0] {
            return false;
        }
    }
    true
}

pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
    let mut vec = vec![a,b,c];
    vec.sort();
    if vec[0]+vec[1]<vec[2] {
        return vec[0]+vec[1];
    }
    return match (a, b, c) {
        (0, 0, _) => { 0 }
        (0, _, 0) => { 0 }
        (_, 0, 0) => { 0 }
        (_, 0, _) => { if a < c { a } else { c } }
        (_, _, 0) => { if a < b { a } else { b } }
        (0, _, _) => { if b < c { b } else { c } }
        (_, _, _) => { (a + b + c) / 2 }
    }
}

pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
    let mut value = goal.abs();
    let mut nums = nums;
    nums.sort();
    for i in 0..nums.len() {
        if (nums[i]-goal).abs() < value {
            value = (nums[i]-goal).abs();
        }
    }
    value
}

#[test]
fn test() {
    // println!("{}", check_possibility(vec![3,4,2,3]));
    // println!("{}", check(vec![2,1]));
    /*assert_eq!(maximum_score(2, 4, 6), 6);
    assert_eq!(maximum_score(4, 4, 6), 7);
    assert_eq!(maximum_score(1, 8, 8), 8);
    assert_eq!(maximum_score(1, 7, 8), 8);
    assert_eq!(maximum_score(0, 4, 6), 4);
    assert_eq!(maximum_score(6, 2, 1), 3);*/

    println!("{}", min_abs_difference(vec![5,-7,3,5,1],6));
    println!("{}", min_abs_difference(vec![7,-9,15,-2],-5));
    println!("{}", min_abs_difference(vec![1,2,3],-7));
}
