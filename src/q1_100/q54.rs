///给你一个 m 行 n 列的矩阵 matrix ，请按照 顺时针螺旋顺序 ，返回矩阵中的所有元素。
///
///  
///
/// 示例 1：
///
///
/// 输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
/// 输出：[1,2,3,6,9,8,7,4,5]
/// 示例 2：
///
///
/// 输入：matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
/// 输出：[1,2,3,4,8,12,11,10,9,5,6,7]
///  
///
/// 提示：
///
/// m == matrix.length
/// n == matrix[i].length
/// 1 <= m, n <= 10
/// -100 <= matrix[i][j] <= 100
///
/// 1. 首先设定上下左右边界
/// 2. 其次向右移动到最右，此时第一行因为已经使用过了，可以将其从图中删去，体现在代码中就是重新定义上边界
/// 3. 判断若重新定义后，上下边界交错，表明螺旋矩阵遍历结束，跳出循环，返回答案
/// 4. 若上下边界不交错，则遍历还未结束，接着向下向左向上移动，操作过程与第一，二步同理
/// 5. 不断循环以上步骤，直到某两条边界交错，跳出循环，返回答案
///
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return vec![];
    }
    //赋值上下左右边界
    let mut u = 0usize;
    let mut d = matrix.len() - 1;
    let mut l = 0usize;
    let mut r = matrix[0].len() - 1;
    let mut ans = Vec::with_capacity((d + 1) * (r + 1));
    loop {
        //向右移动直到最右
        for i in l..=r {
            ans.push(matrix[u][i]);
        }
        //重新设定上边界，若上边界大于下边界，则遍历完成
        if u + 1 > d {
            break;
        }
        u += 1;
        //向下
        for i in u..=d {
            ans.push(matrix[i][r]);
        }
        //重新设定右边界
        if r as i32 - 1 < l as i32 {
            break;
        }
        r -= 1;
        //向左
        for i in (l..=r).rev() {
            ans.push(matrix[d][i]);
        }
        //重新设定下边界
        if d as i32 - 1 < u as i32 {
            break;
        }
        d -= 1;
        //向上
        for i in (u..=d).rev() {
            ans.push(matrix[i][l]);
        }
        //重新设定左边界
        l += 1;
        if l > r {
            break;
        }
    }
    ans
}

#[test]
fn test() {
    println!("{:?}", spiral_order(vec![
        vec![3],
        vec![2],
    ]));
    // println!("{:?}", spiral_order(vec![
    //     vec![1, 2, 3],
    //     vec![4, 5, 6],
    //     vec![7, 8, 9],
    //     vec![10, 11, 12]
    // ]));
    // for i in (0..=1).rev() {
    //     println!("{}", i);
    // }
}
