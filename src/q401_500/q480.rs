use std::collections::BTreeMap;
use std::option::Option::Some;

///中位数是*有序序列*最中间的那个数。如果序列的长度是偶数，则没有最中间的数；此时中位数是最中间的两个数的平均数。
///
/// 例如：
///
/// [2,3,4]，中位数是3
/// [2,3]，中位数是 (2 + 3) / 2 = 2.5
/// 给你一个数组 nums，有一个长度为 k 的窗口从最左端滑动到最右端。窗口中有 k 个数，每次窗口向右移动 1 位。你的任务是找出每次窗口移动后得到的新窗口中元素的中位数，并输出由它们组成的数组。
///
///
/// 示例：
///
/// 给出nums = [1,3,-1,-3,5,3,6,7]，以及k = 3。
/// ```
/// 窗口位置                      中位数
/// ---------------               -----
/// [1  3  -1] -3  5  3  6  7       1
///  1 [3  -1  -3] 5  3  6  7      -1
///  1  3 [-1  -3  5] 3  6  7      -1
///  1  3  -1 [-3  5  3] 6  7       3
///  1  3  -1  -3 [5  3  6] 7       5
///  1  3  -1  -3  5 [3  6  7]      6
/// ```
/// 因此，返回该滑动窗口的中位数数组[1,-1,-1,3,5,6]。
///
/// 来源：力扣（LeetCode）
/// 链接：https:///leetcode-cn.com/problems/sliding-window-median
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let k = k as usize;
    let mut ans = vec![];
    let mut map = BTreeMap::new();
    let mut init_index = 0;
    for i in 0..nums.len() {
        if init_index >= k {
            let key = nums[i - k];
            map.insert(key, map.get(&key).unwrap() - 1);
            if let Some(0) = map.get(&key) {
                map.remove(&key);
            }
        }
        let key = nums[i];
        map.insert(key, map.get(&key).unwrap_or(&0) + 1);
        init_index += 1;
        if i >= k - 1 {
            let middle = get_middle_from_btree_map(&map, k);
            ans.push(middle);
        }
    }
    ans
}

fn get_element_from_slice(slice: &[i32]) -> Vec<i32> {
    let mut res = vec![];
    for i in slice {
        res.push(*i);
    }
    res
}
/*fn average(a:i32,b:i32)->f64{
    if a.checked_add(b).is_none() {
        let a = a as f64;
        let b = b as f64;
        return (a+b)/2.0;
    }else {
        return (a as f64+b as f64)/2.0;
    }
    0.0
}*/
///该map中value是key出现的次数
/// 求的是map中的中位数
fn get_middle_from_btree_map(map: &BTreeMap<i32, usize>, len: usize) -> f64 {
    // let len = map.len();
    let middle_index = len / 2;
    let mut num1 = 0;
    let mut num2 = 0;
    let mut index = 0;
    let mut iter = map.iter();
    while index <= middle_index {
        let (key, value) = iter.next().unwrap();
        for _ in 0..*value {
            if index == middle_index - 1 {
                num1 = *key;
            }
            if index == middle_index {
                num2 = *key;
            }
            index += 1;
        }
    }
    if len % 2 == 0 {
        (num2 as f64 + num1 as f64) / 2.0
    } else {
        num2 as f64
    }
}

#[test]
fn test() {
    // println!("{:?}", median_sliding_window(vec![2147483647, 2147483647], 2));
    // println!("{:?}", median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));
    println!("{:?}", median_sliding_window(vec![7, 0, 3, 9, 9, 9, 1, 7, 2, 3], 6));
    // let mut map = BTreeMap::new();
    // map.insert(2, 1);
    // map.insert(1, 3);
    // map.insert(3, 2);
    // println!("{}", get_middle_from_btree_map(&map, 6));
}
