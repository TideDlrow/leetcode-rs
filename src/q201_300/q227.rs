///给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。
///
/// 整数除法仅保留整数部分。
///
///  
///
/// 示例 1：
///
/// 输入：s = "3+2*2"
/// 输出：7
/// 示例 2：
///
/// 输入：s = " 3/2 "
/// 输出：1
/// 示例 3：
///
/// 输入：s = " 3+5 / 2 "
/// 输出：5
///  
///
/// 提示：
///
/// 1 <= s.length <= 3 * 105
/// s 由整数和算符 ('+', '-', '*', '/') 组成，中间由一些空格隔开
/// s 表示一个 有效表达式
/// 表达式中的所有整数都是非负整数，且在范围 [0, 231 - 1] 内
/// 题目数据保证答案是一个 32-bit 整数
///
pub fn calculate(s: String) -> i32 {
    let mut result = 0;
    //保存上一个符号，初始为+
    let mut sign = b'+';
    let s = s.as_bytes();
    let mut num_stack = Vec::new();
    //保存当前数字，如：12是两个字符，需要进位累加
    let mut num = 0i32;
    for i in 0..s.len() {
        let cur = s[i];
        if cur >= b'0' {
            //记录当前数字。先减，防溢出
            num = num * 10 - b'0' as i32 + cur as i32;
        }
        if (cur < b'0' && cur != b' ') || i == s.len() - 1 {
            //判断上一个符号是什么
            match sign {
                //当前符号前的数字直接压栈
                b'+' => num_stack.push(num),
                //当前符号前的数字取反压栈
                b'-' => num_stack.push(-num),
                //数字栈 栈顶的数字出栈，与当前符号前的数字相乘，结果值压栈
                b'*' => {
                    let top = num_stack.pop().unwrap();
                    num_stack.push(top * num);
                }
                //数字栈 栈顶的数字出栈，除以当前符号前的数字，结果值压栈
                b'/' => {
                    let top = num_stack.pop().unwrap();
                    num_stack.push(top / num);
                }
                _ => {}
            }
            //记录当前符号
            sign = cur;
            //数字清零
            num = 0;
        }
    }
    //将站内剩余数字累加，即为结果
    while !num_stack.is_empty() {
        result += num_stack.pop().unwrap();
    }
    result
}

#[test]
fn test() {
    println!("{}", calculate(String::from(" 3+5 / 2 ")));
}
