///给定两个字符串 text1 和 text2，返回这两个字符串的最长 公共子序列 的长度。如果不存在 公共子序列 ，返回 0 。
///
/// 一个字符串的 子序列 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。
///
/// 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。
/// 两个字符串的 公共子序列 是这两个字符串所共同拥有的子序列。
///
///  
///
/// 示例 1：
///
/// 输入：text1 = "abcde", text2 = "ace"
/// 输出：3
/// 解释：最长公共子序列是 "ace" ，它的长度为 3 。
/// 示例 2：
///
/// 输入：text1 = "abc", text2 = "abc"
/// 输出：3
/// 解释：最长公共子序列是 "abc" ，它的长度为 3 。
/// 示例 3：
///
/// 输入：text1 = "abc", text2 = "def"
/// 输出：0
/// 解释：两个字符串没有公共子序列，返回 0 。
///  
///
/// 提示：
///
/// 1 <= text1.length, text2.length <= 1000
/// text1 和 text2 仅由小写英文字符组成。
///
///
///  dp[i][j] 表示text1[0~i-1]和text2[0~j-1]的最长公共子序列长度 dp[0][0]等于0，
/// 等于dp数组总体往后挪了一个，免去了判断出界 转移方程： text1[i-1] == text2[j-1]
/// 当前位置匹配上了: dp[i][j]=dp[i-1][j-1]+1 text1[i-1] ！= text2[j-1]
/// 当前位置没匹配上了 ：dp[i][j]=max(dp[i-1][j],dp[i][j-1]);
/// basecase: 任何一个字符串为0时都是零，初始化时候就完成了basecase是赋值
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let m = text1.len();
    let n = text2.len();
    let text1 = text1.as_bytes();
    let text2 = text2.as_bytes();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if text1[i - 1] == text2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    return dp[m][n];
}

#[test]
fn test() {
    println!("{}", longest_common_subsequence(String::from("abcde"), String::from("ace")));
}
