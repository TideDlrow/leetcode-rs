///编写一个函数来查找字符串数组中的最长公共前缀。
///
/// 如果不存在公共前缀，返回空字符串 ""。
///
///  
///
/// 示例 1：
///
/// 输入：strs = ["flower","flow","flight"]
/// 输出："fl"
/// 示例 2：
///
/// 输入：strs = ["dog","racecar","car"]
/// 输出：""
/// 解释：输入不存在公共前缀。
///  
///
/// 提示：
///
/// 0 <= strs.length <= 200
/// 0 <= strs[i].length <= 200
/// strs[i] 仅由小写英文字母组成
///
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 {
        return "".to_string();
    }
    //公共前缀比所有字符串都短，随便选一个
    let mut s = strs[0].as_str();
    for str in strs.iter() {
        while !str.starts_with(s) {
            if s.len() == 0 {
                return "".to_string();
            }
            //公共前缀不匹配就让它变短
            s = &s[0..s.len() - 1];
        }
    }
    String::from(s)
}

#[test]
fn test() {
    println!("{}", longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
}
