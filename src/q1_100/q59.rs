///给你一个正整数 n ，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。
///
///  
///
/// 示例 1：
///
///
/// 输入：n = 3
/// 输出：[[1,2,3],[8,9,4],[7,6,5]]
/// 示例 2：
///
/// 输入：n = 1
/// 输出：[[1]]
///  
///
/// 提示：
///
/// 1 <= n <= 20
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    if n == 1 {
        return vec![vec![1]];
    }
    let mut ans = vec![vec![0; n as usize]; n as usize];
    let mut l = 0;
    let mut r = n - 1;
    let mut t = 0;
    let mut b = n - 1;

    let mut num = 1;
    let tar = n * n;
    while num <= tar {
        for i in l..=r as usize {
            ans[t][i] = num;
            num += 1;
        }
        t += 1;
        for i in t..=b as usize {
            ans[i][r as usize] = num;
            num += 1;
        }
        r -= 1;
        for i in (l..=r as usize).rev() {
            ans[b as usize][i] = num;
            num += 1;
        }
        b -= 1;
        for i in (t..=b as usize).rev() {
            ans[i][l] = num;
            num += 1;
        }
        l += 1;
    }
    ans
}

#[test]
fn test() {
    println!("{:?}", generate_matrix(4));
}
