///给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是回文。
///
/// 返回符合要求的 最少分割次数 。
///
///  
///
/// 示例 1：
///
/// 输入：s = "aab"
/// 输出：1
/// 解释：只需一次分割就可将 s 分割成 ["aa","b"] 这样两个回文子串。
/// 示例 2：
///
/// 输入：s = "a"
/// 输出：0
/// 示例 3：
///
/// 输入：s = "ab"
/// 输出：1
///  
///
/// 提示：
///
/// 1 <= s.length <= 2000
/// s 仅由小写英文字母组成
///
///
/// 如果从分割字符串的角度考虑这个问题的话，对于一个区间内的字符串来说，
/// 每一个位置都将是可能的分割点，可以用暴力递归的方式找出答案，但是时间复杂度太高，
/// 加上预处理回文数组能勉强通过。 换个角度想想，当切割次数最少使得切割后的所有字符串都是回文时，
/// 也正是这些回文子串最长的时候，那么如果说能找到以每个字符为中心的最长回文串，
/// 实际上就已经找到了答案。
pub fn min_cut(s: String) -> i32 {
    if s.len() <= 1 {
        return 0;
    }
    let len = s.len();
    let mut dp = vec![(len - 1) as i32; len];
    for i in 0..len {
        min_cut_helper(&s, i, i, &mut dp);
        min_cut_helper(&s, i, i + 1, &mut dp);
    }
    return dp[len - 1];
}

fn min_cut_helper(s: &str, i: usize, j: usize, dp: &mut Vec<i32>) {
    let len = s.len();
    let mut i = i as i32;
    let mut j = j as i32;
    while i >= 0 && j < len as i32 && s[i as usize..i as usize + 1] == s[j as usize..j as usize + 1] {
        dp[j as usize] = std::cmp::min(dp[j as usize], if i == 0 { -1 } else { dp[i as usize - 1] } + 1);
        i -= 1;
        j += 1;
    }
}

#[test]
fn test() {
    // let s1 = String::from("hello");
    // let s2 = String::from("world");
    // let s11 = &s1[4..5];
    // let s21 = &s2[1..2];
    // println!("{:?},{:?}", s11, s21);
    // println!("{}", s11 == s21);
    // println!("{}", s11.eq(s21));
    println!("{}", min_cut(String::from("aab")));
}
