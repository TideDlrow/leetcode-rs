use std::collections::HashMap;

///给定一个非空且只包含非负数的整数数组 nums，数组的度的定义是指数组里任一元素出现频数的最大值。
///
/// 你的任务是在 nums 中找到与 nums 拥有相同大小的度的最短连续子数组，返回其长度。
///
///  
///
/// 示例 1：
///
/// 输入：[1, 2, 2, 3, 1]
/// 输出：2
/// 解释：
/// 输入数组的度是2，因为元素1和2的出现频数最大，均为2.
/// 连续子数组里面拥有相同度的有如下所示:
/// [1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
/// 最短连续子数组[2, 2]的长度为2，所以返回2.
/// 示例 2：
///
/// 输入：[1,2,2,3,1,4,2]
/// 输出：6
///  
///
/// 提示：
///
/// nums.length 在1到 50,000 区间范围内。
/// nums[i] 是一个在 0 到 49,999 范围内的整数。
///
pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    //先遍历一遍明确各数字出现的次数和位置
    //再找到最大的度，最后找到最大度中最小的长度
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        let num = nums[i];
        if !map.contains_key(&num) {
            //从左至右分别是出现次数，第一次出现的位置、最后一次出现的位置
            let tuple = (1, i, i);
            map.insert(num, tuple);
        } else {
            let tuple = map.get_mut(&num).unwrap();
            tuple.0 += 1;
            tuple.2 = i;
        }
    }
    //最大的度
    let mut max_du = 0;
    //最大度对应的key
    let mut max_du_key = -1;
    for i in map.keys() {
        let tuple = map.get(i).unwrap();
        if tuple.0 > max_du {
            max_du = tuple.0;
            max_du_key = *i;
        }
    }

    let mut min_len = i32::MAX;
    for key in map.keys() {
        let tuple = map.get(key).unwrap();

        if tuple.0 == max_du {
            min_len = std::cmp::min(min_len, (tuple.2 - tuple.1) as i32);
        }
    }
    // println!("{:?}", map);
    min_len + 1
}
#[test]
fn test() {
    // println!("{}", find_shortest_sub_array(vec![1, 2, 2, 3, 1]));
    // println!("{:?}",&mfe);
}
