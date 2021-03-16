use std::option::Option::Some;

///序列化二叉树的一种方法是使用前序遍历。当我们遇到一个非空节点时，我们可以记录下这个节点的值。如果它是一个空节点，我们可以使用一个标记值记录，例如 #。
///
///      _9_
///     /   \
///    3     2
///   / \   / \
///  4   1  #  6
/// / \ / \   / \
/// # # # #   # #
/// 例如，上面的二叉树可以被序列化为字符串 "9,3,4,#,#,1,#,#,2,#,6,#,#"，其中 # 代表一个空节点。
///
/// 给定一串以逗号分隔的序列，验证它是否是正确的二叉树的前序序列化。编写一个在不重构树的条件下的可行算法。
///
/// 每个以逗号分隔的字符或为一个整数或为一个表示 null 指针的 '#' 。
///
/// 你可以认为输入格式总是有效的，例如它永远不会包含两个连续的逗号，比如 "1,,3" 。
///
/// 示例 1:
///
/// 输入: "9,3,4,#,#,1,#,#,2,#,6,#,#"
/// 输出: true
/// 示例 2:
///
/// 输入: "1,#"
/// 输出: false
/// 示例 3:
///
/// 输入: "9,#,#,1"
/// 输出: false
///
///  string从后遍历，用num记录#的个数，当遇到正常节点时，#的个数-2，并将该节点转化成#，num+1，，整体即为num-1; 当出现num的个数不足2时，即false，最终也须保证num为1。
pub fn is_valid_serialization(preorder: String) -> bool {
    let nodes: Vec<&str> = preorder.split(",").collect();
    let mut num = 0;
    let len = nodes.len();
    let mut index: i32 = (len - 1) as i32;
    while index >= 0 {
        if nodes[index as usize] == "#" {
            num += 1;
        } else {
            if num >= 2 {
                num -= 1;
            } else {
                return false;
            }
        }
        index -= 1;
    }
    if num != 1 {
        return false;
    }
    true
}

#[test]
fn test() {
    assert_eq!(is_valid_serialization(String::from("9,3,4,#,#,1,#,#,2,#,6,#,#")), true);
    assert_eq!(is_valid_serialization(String::from("1,#")), false);
    assert_eq!(is_valid_serialization(String::from("9,#,#,1")), false);
}
