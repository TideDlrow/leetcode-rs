use std::cmp::Ordering;

///给定一些标记了宽度和高度的信封，宽度和高度以整数对形式 (w, h) 出现。当另一个信封的宽度和高度都比这个信封大的时候，这个信封就可以放进另一个信封里，如同俄罗斯套娃一样。
///
/// 请计算最多能有多少个信封能组成一组“俄罗斯套娃”信封（即可以把一个信封放到另一个信封里面）。
///
/// 说明:
/// 不允许旋转信封。
///
/// 示例:
///
/// 输入: envelopes = [[5,4],[6,4],[6,7],[2,3]]
/// 输出: 3
/// 解释: 最多信封的个数为 3, 组合为: [2,3] => [5,4] => [6,7]。
///
pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    //先讲数组按宽度升序排列，如果宽度相同再按高度降序排列
    //此时在h上寻找最长递增子序列  寻找递增子序列在题目q300中有
    let mut envelopes = envelopes;
    envelopes.sort_by(|x, y| {
        if x[0] == y[0] {
            if y[1] - x[1] > 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            if x[0] - y[0] > 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    });
    let mut height = Vec::with_capacity(envelopes.len());
    for envelope in envelopes.iter() {
        height.push(envelope[1]);
    }
    // println!("{:?}", envelopes);

    length_of_lis(height)
}

//q300
fn length_of_lis(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut tails = vec![0; len];
    let mut res = 0;
    for num in nums.iter() {
        let mut i = 0;
        let mut j = res;
        while i < j {
            let m = (i + j) / 2;
            if tails[m] < *num {
                i = m + 1;
            } else {
                j = m;
            }
        }
        tails[i] = *num;
        if res == j {
            res += 1;
        }
    }
    res as i32
}

#[test]
fn test() {
    println!("{}", max_envelopes(vec![
        vec![5, 4],
        vec![6, 4],
        vec![6, 7],
        vec![2, 3],
    ]));
}
