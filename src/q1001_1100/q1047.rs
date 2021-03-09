///给出由小写字母组成的字符串 S，重复项删除操作会选择两个相邻且相同的字母，并删除它们。
///
/// 在 S 上反复执行重复项删除操作，直到无法继续删除。
///
/// 在完成所有重复项删除操作后返回最终的字符串。答案保证唯一。
///
///  
///
/// 示例：
///
/// 输入："abbaca"
/// 输出："ca"
/// 解释：
/// 例如，在 "abbaca" 中，我们可以删除 "bb" 由于两字母相邻且相同，这是此时唯一可以执行删除操作的重复项。之后我们得到字符串 "aaca"，其中又只有 "aa" 可以执行重复项删除操作，所以最后的字符串为 "ca"。
///  
///
/// 提示：
///
/// 1 <= S.length <= 20000
/// S 仅由小写英文字母组成。
///
/// 用栈，类似括号匹配
pub fn remove_duplicates(s: String) -> String {
    let mut res = String::new();
    for char in s.chars() {
        if let Some(ch) = res.pop() {
            //如果出栈的字符与当前字符不同，则把当前字符和出栈的字符重新放回去
            //如果相同，则不操作跳到下一个字符(因为字符已经出栈)
            if ch != char {
                res.push(ch);
                res.push(char);
            }
        } else {
            res.push(char);
        }
    }
    res
}

#[test]
fn test() {
    println!("{:?}", remove_duplicates(String::from("abbaca")));
}
