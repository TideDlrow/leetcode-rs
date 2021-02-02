
///给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的中位数。
///
/// 示例 1：
/// ```
/// 输入：nums1 = [1,3], nums2 = [2]
/// 输出：2.00000
/// 解释：合并数组 = [1,2,3] ，中位数 2
///```
/// 示例 2：
/// ```
/// 输入：nums1 = [1,2], nums2 = [3,4]
/// 输出：2.50000
/// 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
/// ```
/// 示例 3：
/// ```
/// 输入：nums1 = [0,0], nums2 = [0,0]
/// 输出：0.00000
/// ```
/// 示例 4：
/// ```
/// 输入：nums1 = [], nums2 = [1]
/// 输出：1.00000
/// ```
/// 示例 5：
/// ```
/// 输入：nums1 = [2], nums2 = []
/// 输出：2.00000
/// ```
///
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut total: Vec<i32> = Vec::new();
    /*let nums1_length = nums1.len();
    let nums2_length = nums2.len();
    //每次将nums1中的元素添加到total时，该计数就+1
    let mut nums1_index = 0;
    //每次将nums2中的元素添加到total时，该计数就+1
    let mut nums2_index = 0;
    let mut index = 0;
    while nums1_index + nums2_index < nums1_length + nums2_length {
        if nums1_index>=nums1_length {
            total.push(nums2[nums2_index]);
            nums2_index += 1;
            continue;
        }
        if nums2_index>=nums2_length {
            total.push(nums1[nums1_index]);
            nums1_index += 1;
            continue;
        }
        if nums1[nums1_index] < nums2[nums2_index] {
            total.push(nums1[nums1_index]);
            nums1_index += 1;
        } else {
            total.push(nums2[nums2_index]);
            nums2_index += 1;
        }

    }
    //此时total中的元素已经是按从小到大排列好的
    let len = total.len();
    if len & 1 == 1 {
        return total[len / 2] as f64;
    } else {
        return (total[len / 2] + total[(len - 1) / 2]) as f64 / 2.0;
    }*/
    let mut nums1_clone = nums1.clone();
    let mut nums2_clone = nums2.clone();
    total.append(&mut nums1_clone);
    total.append(&mut nums2_clone);
    total.sort();
    let len = total.len();
    if len & 1 == 1 {
        return total[len / 2] as f64;
    } else {
        return (total[len / 2] + total[(len - 1) / 2]) as f64 / 2.0;
    }
}

pub fn my_sqrt(x: i32) -> i32 {
    f64::sqrt(x as f64) as i32
}

///假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
///
/// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
///
/// 注意：给定 n 是一个正整数。
///
/// 直接结论：这是一个前两项为1,2的斐波那契数列
pub fn climb_stairs(n: i32) -> i32 {
    let mut first = 1;
    let mut second = 2;
    if n == 1 || n == 2 {
        return n;
    }
    for i in 0..n - 2 {
        let temp = first;
        first = second;
        second = temp + second;
    }
    second
}

pub fn fib(n: i32) -> i32 {
    /*let mut first = 1;
    let mut second = 1;
    let mut temp = 0;
    if n==0 {
        return 0;
    }
    if n==1 || n==2 {
        return 1;
    }

    for i in 0..n-2 {
        temp = first;
        first = second;
        second = temp+second;
    }
    second*/
    let sqrt5 = f64::sqrt(5.0);
    (1.0 / sqrt5 * (f64::powi((1.0 + sqrt5) / 2.0, n) - f64::powi((1.0 - sqrt5) / 2.0, n))) as i32
}

pub fn tribonacci(n: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    let mut third = 1;
    let mut temp1 = 0;
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }
    if n == 3 {
        return 2;
    }
    // 0 1 1 2 4 7...
    for i in 0..n - 2 {
        temp1 = first;
        first = second;
        second = third;
        third = temp1 + first + second;
    }
    third
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits_clone = digits.clone();
    let len = digits_clone.len();
    let mut flag = false;
    for i in (len - 1)..0 {
        if digits[i] <= 8 {
            digits_clone[i] = digits_clone[i] + 1;
            break;
        }
    }
    let mut index: i32 = (len - 1) as i32;
    loop {
        let temp = digits_clone[index as usize];
        if temp <= 8 {
            digits_clone[index as usize] = digits_clone[index as usize] + 1;
            break;
        } else {
            digits_clone[index as usize] = 0;
            index -= 1;
        }
        if index == -1 {
            digits_clone.insert(0, 1);
            break;
        }
    }
    digits_clone
}

///给定一个增序排列数组 nums ，你需要在 原地 删除重复出现的元素，
/// 使得每个元素最多出现两次，返回移除后数组的新长度。
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut new_len = 0;
    /*for num in &mut nums.iter() {
        if new_len < 2 || *num != nums[new_len - 2] {
            nums[new_len] = *num;
            new_len += 1;
        }
    }
    println!("{:?}",nums);*/
    new_len as i32
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    for i in m as usize..nums1.len() {
        nums1.remove(m as usize);
        println!("{:?}", nums1);
    }
    nums1.append(nums2);
    nums1.sort();
    println!("{:?}", nums1);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut last: [i32; 128] = [-1; 128];
    let len = s.len();
    let s_chars: Vec<char> = s.chars().collect();
    let mut res = 0;
    let mut start = 0;
    for i in 0..len {
        let index = s_chars[i];
        start = std::cmp::max(start, last[index as usize] + 1);
        res = std::cmp::max(res, i as usize - start as usize + 1);
        last[index as usize] = i as i32;
    }
    res as i32
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let len = height.len();
    let mut left = 0usize;
    let mut right = len - 1;
    let mut area = 0;
    while left < right {
        area = std::cmp::max(area, (right - left) as i32 * std::cmp::min(height[left], height[right]));
        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }
    }
    return area;
}

///给定两个整数，被除数`dividend`和除数`divisor`。将两数相除，要求不使用乘法、除法和 mod 运算符。
///
/// 返回被除数`dividend`除以除数`divisor`得到的商。
///
/// 整数除法的结果应当截去（truncate）其小数部分，例如：truncate(8.345) = 8 以及 truncate(-2.7335) = -2
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/divide-two-integers
///
/// 抖机灵版
/// a/b = e^(ln(a/b)) = e^(lna - lnb)
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let flag = if dividend < 0 && divisor > 0 { false } else if dividend > 0 && divisor < 0 { false } else { true };
    let result = f64::exp(f64::ln(dividend.abs() as f64) - f64::ln(divisor.abs() as f64)) as i32;
    if flag {
        result
    } else {
        -result
    }
}

fn main() {
    // let nums1 = vec![3];
    // let nums2 = vec![-2,-1];
    // let result = find_median_sorted_arrays(nums1, nums2);
    // println!("{}", result);

    println!("{}", divide(7, -3))
}
