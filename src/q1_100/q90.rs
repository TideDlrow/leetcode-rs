///给你一个整数数组 nums ，其中可能包含重复元素，请你返回该数组所有可能的子集（幂集）。
///
/// 解集 不能 包含重复的子集。返回的解集中，子集可以按 任意顺序 排列。
///
///  
///
/// 示例 1：
///
/// 输入：nums = [1,2,2]
/// 输出：[[],[1],[1,2],[1,2,2],[2],[2,2]]
/// 示例 2：
///
/// 输入：nums = [0]
/// 输出：[[],[0]]
///  
///
/// 提示：
///
/// 1 <= nums.length <= 10
/// -10 <= nums[i] <= 10
///
pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![vec![]];
    let mut path: Vec<i32> = vec![];

    let mut used: Vec<bool> = vec![false; nums.len()];

    nums.sort_unstable(); // 先排序
    backtracking(&nums, &mut res, &mut path, &mut used, 0);
    res
}

pub fn backtracking(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, used: &mut Vec<bool>, start_index: usize) {
    if !path.is_empty() {
        res.push(path.clone());
    }

    for i in start_index..nums.len() {
        // 判断是否重复
        if i > 0 && nums[i] == nums[i - 1] && used[i - 1] == false {
            continue;
        }
        path.push(nums[i]);
        used[i] = true;
        backtracking(nums, res, path, used, i + 1);
        used[i] = false;
        path.pop();
    }
}

#[test]
fn test() {
    println!("{:?}", subsets_with_dup(vec![0]));
}
