///给定一个包含非负整数的数组，你的任务是统计其中可以组成三角形三条边的三元组个数。
///
/// 示例 1:
/// ```
/// 输入: [2,2,3,4]
/// 输出: 3
/// 解释:
/// 有效的组合是:
/// 2,3,4 (使用第一个 2)
/// 2,3,4 (使用第二个 2)
/// 2,2,3
/// ```
/// 注意:
///
/// 数组长度不超过1000。
/// 数组里整数的范围为 [0, 1000]。
///
pub fn triangle_number(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len<3 {
        return 0;
    }
    let mut res = 0;
    let mut nums = nums.clone();
    nums.sort();
    let mut i = len-1;
    while i > 1 {
        let (mut l,mut r) = (0,i-1);
        while l < r {
            if nums[l]+nums[r]>nums[i] {
                res+=r-1;
                r-=1;
            }else {
                l+=1;
            }
        }
        i-=1;
    }
    res as i32
}
#[test]
fn test(){
    println!("{}",triangle_number(vec![2,2,3,4]));
}
