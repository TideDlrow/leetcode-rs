
///实现一个基本的计算器来计算一个简单的字符串表达式 s 的值。
///
///  
///
/// 示例 1：
///
/// 输入：s = "1 + 1"
/// 输出：2
/// 示例 2：
///
/// 输入：s = " 2-1 + 2 "
/// 输出：3
/// 示例 3：
///
/// 输入：s = "(1+(4+5+2)-3)+(6+8)"
/// 输出：23
///  
///
/// 提示：
///
/// 1 <= s.length <= 3 * 105
/// s 由数字、'+'、'-'、'('、')'、和 ' ' 组成
/// s 表示一个有效的表达式
pub fn calculate(s: String) -> i32 {
    let mut ret = 0;
    let s = s.as_bytes();
    let mut ops = Vec::new();
    ops.push(1);
    let mut sign = 1;
    let mut i = 0;
    let n = s.len();
    while i < n {
        match s[i] {
            b' ' => {
                i += 1;
            }
            b'+' => {
                sign = *ops.last().unwrap();
                i += 1;
            }
            b'-' => {
                sign = -*ops.last().unwrap();
                i += 1;
            }
            b'(' => {
                ops.push(sign);
                i += 1;
            }
            b')' => {
                ops.pop();
                i += 1;
            }
            _ => {
                let mut num = 0;
                while i < n && s[i] >= b'0' && s[i] <= b'9' {
                    num = num * 10 + (s[i] - b'0') as i32;
                    i += 1;
                }
                ret += sign * num;
            }
        }
    }
    ret
}

#[test]
fn test() {
    println!("{}", calculate(String::from(" 2-1 + 2 ")));
}
