///编写一个高效的算法来判断m x n矩阵中，是否存在一个目标值。该矩阵具有如下特性：
///
/// 每行中的整数从左到右按升序排列。
/// 每行的第一个整数大于前一行的最后一个整数。
///
///
/// 示例 1：
///
///
///输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
/// 输出：true
/// 示例 2：
///
///
/// 输入：matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
/// 输出：false
///
///
/// 提示：
///
/// m == matrix.length
/// n == matrix[i].length
/// 1 <= m, n <= 100
/// -10^4 <= matrix[i][j], target <= 10^4
///
/// 思路：二分查找
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    if m == 0 {
        return false;
    }
    let n = matrix[0].len();
    if n == 0 {
        return false;
    }
    let mut low = 0;
    let mut high = m * n - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        let row = mid / n;
        let col = mid % n;
        if matrix[row][col] == target {
            return true;
        }
        if matrix[row][col] > target {
            if mid == 0 {
                return false;
            }
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    false
}

#[test]
fn test() {
    let matrix = vec![
        // vec![1, 3, 5, 7],
        // vec![10, 11, 16, 20],
        // vec![23, 30, 34, 60],
        vec![1, 1]
    ];
    println!("{}", search_matrix(matrix, 0));
}
