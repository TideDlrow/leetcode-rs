///给定两个字符串s1和s2，写一个函数来判断 s2 是否包含 s1的排列。
///
/// 换句话说，第一个字符串的排列之一是第二个字符串的子串。
///
/// 示例1:
/// ```
/// 输入: s1 = "ab" s2 = "eidbaooo"
/// 输出: True
/// 解释: s2 包含 s1 的排列之一 ("ba").
/// ```
///
/// 示例2:
/// ```
/// 输入: s1= "ab" s2 = "eidboaoo"
/// 输出: False
/// ```
///
/// 注意：
/// ```
/// 输入的字符串只包含小写字母
/// 两个字符串的长度都在 [1, 10,000] 之间
/// ```
pub fn check_inclusion(s1: String, s2: String) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let n = s1.len();
    let m = s2.len();
    if n > m {
        return false;
    }
    let mut cnt = [0; 26];
    for i in 0..n {
        cnt[(s1[i] - 'a' as u8) as usize] -= 1;
    }
    let mut left = 0;
    let mut right = 0;
    while right < m {
        let x = (s2[right] - 'a' as u8) as usize;
        cnt[x] += 1;
        while cnt[x] > 0 {
            cnt[(s2[left] - 'a' as u8) as usize] -= 1;
            left += 1;
        }
        if right as i32 - left as i32 + 1 == n as i32 {
            return true;
        }
        right += 1;
    }
    false
}

#[test]
fn test() {
    println!("{}", check_inclusion("ab".to_string(), "eidbaooo".to_string()));
}
