///给定一个二维矩阵，计算其子矩形范围内元素的总和，该子矩阵的左上角为 (row1, col1) ，右下角为 (row2, col2)。
///
///
///
/// 示例:
///
/// 给定 matrix = [
///   [3, 0, 1, 4, 2],
///   [5, 6, 3, 2, 1],
///   [1, 2, 0, 1, 5],
///   [4, 1, 0, 1, 7],
///   [1, 0, 3, 0, 5]
/// ]
///
/// sumRegion(2, 1, 4, 3) -> 8
/// sumRegion(1, 1, 2, 2) -> 11
/// sumRegion(1, 2, 2, 4) -> 12
/// 说明:
///
/// 你可以假设矩阵不可变。
/// 会多次调用 sumRegion 方法。
/// 你可以假设 row1 ≤ row2 且 col1 ≤ col2。
///
struct NumMatrix {
    //sum[i+1][j+1] = sum_Region(0,0,i,j)
    //即每项元素为矩阵matrix从0,0到i,j的和
    sum: Option<Vec<Vec<i32>>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return Self {
                sum: None
            };
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut sum = vec![vec![0; cols + 1]; rows + 1];
        for i in 0..rows {
            for j in 0..cols {
                sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + matrix[i][j];
            }
        }
        Self {
            sum: Some(sum)
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let sum = &self.sum;
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;
        if let Some(sum) = sum {
            sum[row2 + 1][col2 + 1] - sum[row1][col2 + 1] - sum[row2 + 1][col1] + sum[row1][col1]
        } else {
            0
        }
    }
}

#[test]
fn test() {
    let t = NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5]
    ]);
    println!("{}", t.sum_region(2, 1, 4, 3));
    println!("{}", t.sum_region(1, 1, 2, 2));
    println!("{}", t.sum_region(1, 2, 2, 4));
}
