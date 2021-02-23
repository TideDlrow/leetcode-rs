///今天，书店老板有一家店打算试营业 customers.length 分钟。每分钟都有一些顾客（customers[i]）会进入书店，所有这些顾客都会在那一分钟结束后离开。
///
/// 在某些时候，书店老板会生气。 如果书店老板在第 i 分钟生气，那么 grumpy[i] = 1，否则 grumpy[i] = 0。 当书店老板生气时，那一分钟的顾客就会不满意，不生气则他们是满意的。
///
/// 书店老板知道一个秘密技巧，能抑制自己的情绪，可以让自己连续 X 分钟不生气，但却只能使用一次。
///
/// 请你返回这一天营业下来，最多有多少客户能够感到满意的数量。
///
/// 示例：
///
/// 输入：customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], X = 3
/// 输出：16
/// 解释：
/// 书店老板在最后 3 分钟保持冷静。
/// 感到满意的最大客户数量 = 1 + 1 + 1 + 1 + 7 + 5 = 16.
///  
///
/// 提示：
///
/// 1 <= X <= customers.length == grumpy.length <= 20000
/// 0 <= customers[i] <= 1000
/// 0 <= grumpy[i] <= 1
///
/// 题解：先求出满意的顾客数，再用窗口为X求出因为生气而被赶走的顾客数g
/// 最终得到最大的g再加上之前满意的顾客数就是答案
pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
    let x = x as usize;
    //满意的顾客数
    let mut grumpy_customers = 0;
    for i in 0..customers.len() {
        if grumpy[i] == 0 {
            grumpy_customers += customers[i];
        }
    }

    let mut max_grumpy_customers = 0;
    let mut temp = 0;
    //先计算起始的[0,x)
    for i in 0..x {
        if grumpy[i] == 1 {
            temp += customers[i];
        }
    }
    max_grumpy_customers = std::cmp::max(max_grumpy_customers, temp);
    // 然后利用滑动窗口，每次向右移动一步
    for i in x..customers.len() {
        // 如果新进入窗口的元素是生气的，累加不满意的顾客到滑动窗口中
        if grumpy[i] == 1 {
            temp += customers[i];
        }
        //如果离开窗口的元素是生气的，则从滑动窗口中减去该不满意的顾客数
        if grumpy[i - x] == 1 {
            temp -= customers[i - x];
        }
        max_grumpy_customers = std::cmp::max(max_grumpy_customers, temp);
    }
    println!("满意的：{},不满意的：{}", grumpy_customers, max_grumpy_customers);
    grumpy_customers + max_grumpy_customers
}

#[test]
fn test() {
    // println!("{}", max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 1], 3));
    println!("{}", max_satisfied(vec![3], vec![1], 1));
}
