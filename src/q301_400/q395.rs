///找到给定字符串（由小写字符组成）中的最长子串 T ， 要求 T 中的每一字符出现次数都不少于 k 。输出 T 的长度。
///
/// 示例 1:
///
/// 输入:
/// s = "aaabb", k = 3
///
/// 输出:
/// 3
///
/// 最长子串为 "aaa" ，其中 'a' 重复了 3 次。
/// 示例 2:
///
/// 输入:
/// s = "ababbc", k = 2
///
/// 输出:
/// 5
///
/// 最长子串为 "ababb" ，其中 'a' 重复了 2 次， 'b' 重复了 3 次。
///
pub fn longest_substring(s: String, k: i32) -> i32 {
    let mut ret = 0;
    let n = s.len();
    let s = s.as_bytes();
    let k = k as u8;
    for t in 0..26 {
        let mut l = 0;
        let mut r = 0;
        let mut cnt = [0u8; 26];
        let mut tot = 0;
        let mut less = 0;
        while r < n {
            cnt[s[r] as usize - b'a' as usize] += 1;
            let tmp = cnt[s[r] as usize - b'a' as usize];
            if tmp == 1 {
                tot += 1;
                less += 1;
            }
            if tmp == k {
                less -= 1;
            }

            while tot > t {
                cnt[s[l] as usize - b'a' as usize] -= 1;
                let tmp = cnt[s[l] as usize - b'a' as usize];
                if tmp == k - 1 {
                    less += 1;
                }
                if tmp == 0 {
                    tot -= 1;
                    less -= 1;
                }
                l += 1;
            }
            if less == 0 {
                ret = std::cmp::max(ret, r as i32 - l as i32 + 1);
            }
            r += 1;
        }
    }
    ret as i32
}

#[test]
fn test() {
    println!("{}", longest_substring("aaabb".to_string(), 3));
    println!("{}", longest_substring("ababbc".to_string(), 2));
}
