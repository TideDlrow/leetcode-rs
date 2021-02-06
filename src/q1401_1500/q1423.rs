///几张卡牌 排成一行，每张卡牌都有一个对应的点数。点数由整数数组 cardPoints 给出。
///
/// 每次行动，你可以从行的开头或者末尾拿一张卡牌，最终你必须正好拿 k 张卡牌。
///
/// 你的点数就是你拿到手中的所有卡牌的点数之和。
///
/// 给你一个整数数组 cardPoints 和整数 k，请你返回可以获得的最大点数。
///
///
/// 示例 1：
/// ```
/// 输入：cardPoints = [1,2,3,4,5,6,1], k = 3
/// 输出：12
/// 解释：第一次行动，不管拿哪张牌，你的点数总是 1 。但是，先拿最右边的卡牌将会最大化你的可获得点数。最优策略是拿右边的三张牌，最终点数为 1 + 6 + 5 = 12 。
/// ```
/// 示例 2：
/// ```
/// 输入：cardPoints = [2,2,2], k = 2
/// 输出：4
/// 解释：无论你拿起哪两张卡牌，可获得的点数总是 4 。
/// ```
/// 示例 3：
/// ```
/// 输入：cardPoints = [9,7,7,9,7,7,9], k = 7
/// 输出：55
/// 解释：你必须拿起所有卡牌，可以获得的点数为所有卡牌的点数之和。
/// ```
/// 示例 4：
/// ```
/// 输入：cardPoints = [1,1000,1], k = 1
/// 输出：1
/// 解释：你无法拿到中间那张卡牌，所以可以获得的最大点数为 1 。
/// ```
/// 示例 5：
/// ```
/// 输入：cardPoints = [1,79,80,1,1,1,200,1], k = 3
/// 输出：202
/// ```
/// 提示：
/// ```
/// 1 <= cardPoints.length <= 10^5
/// 1 <= cardPoints[i] <= 10^4
/// 1 <= k <= cardPoints.length
/// ```
/// # 解法：
///维护一个len-k的窗口，保证窗口里面和最小，然后剩余的k个数的和就是最大
pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let len = card_points.len();
    //数字总和
    let mut sum = 0;
    sum = card_points.iter().sum();
    //窗口大小
    let window_len = len - k as usize;
    //选取前n-k个作为初始值
    let mut min_sum = 0;
    for i in 0..window_len {
        min_sum += card_points[i];
    }
    let mut temp = min_sum;
    for i in window_len..len {
        // 滑动窗口每向右移动一格，增加从右侧进入窗口的元素值，并减少从左侧离开窗口的元素值
        temp += card_points[i] - card_points[i - window_len];
        min_sum = std::cmp::min(min_sum, temp);
    }
    sum - min_sum
}

#[test]
fn test() {
    println!("{}", max_score(vec![9, 7, 7, 9, 7, 7, 9], 7));
}
