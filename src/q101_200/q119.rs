///给定一个非负索引 k，其中 k ≤ 33，返回杨辉三角的第 k 行。
/// 在杨辉三角中，每个数是它左上方和右上方的数的和。
///
/// 示例:
///
/// 输入: 3
/// 输出: [1,3,3,1]
/// 进阶：
///
/// 你可以优化你的算法到 O(k) 空间复杂度吗？
///
/// 获取杨辉三角的指定行
/// 第n行m列的值为 第n行m-1列乘以(n-m+1)/m
/// 第n行0列的值=1
pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut row = Vec::with_capacity(row_index as usize + 1);
    row.push(1);
    for i in 1..row_index as usize + 1 {
        let temp = ((row[i - 1] as i64 * (row_index - i as i32 + 1) as i64) / i as i64) as i32;
        row.push(temp);
    }
    row
}

#[test]
fn test() {
    println!("{:?}", get_row(30));
}
