///
///给定 n 个整数，找出平均数最大且长度为 k 的连续子数组，并输出该最大平均数。
///示例：
/// ```
///输入：[1,12,-5,-6,50,3], k = 4
///输出：12.75
///解释：最大平均数 (12-5-6+50)/4 = 51/4 = 12.75
/// ```
///提示：
///1 <= k <= n <= 30,000。
///所给数据范围 [-10,000，10,000]。
///
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut sum = 0;
    let mut max_i = 0f64;
    for i in 0..k {
        sum += nums[i];
    }
    max_i = sum as f64 / k as f64;
    for i in k..nums.len() {
        // let temp = sum as f64 / k as f64;
        sum -= nums[i - k];
        sum += nums[i];
        let temp2 = sum as f64 / k as f64;
        if temp2>max_i {
            max_i = temp2;
        }
    }
    max_i
}
#[test]
fn test(){
    println!("{}",find_max_average(vec![1,12,-5,-6,50,3],6))
}
