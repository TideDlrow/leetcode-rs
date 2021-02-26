use std::collections::HashMap;

///外国友人仿照中国字谜设计了一个英文版猜字谜小游戏，请你来猜猜看吧。
///
/// 字谜的迷面 puzzle 按字符串形式给出，如果一个单词 word 符合下面两个条件，那么它就可以算作谜底：
///
/// 单词 word 中包含谜面 puzzle 的第一个字母。
/// 单词 word 中的每一个字母都可以在谜面 puzzle 中找到。
/// 例如，如果字谜的谜面是 "abcdefg"，那么可以作为谜底的单词有 "faced", "cabbage", 和 "baggage"；而 "beefed"（不含字母 "a"）以及 "based"（其中的 "s" 没有出现在谜面中）都不能作为谜底。
/// 返回一个答案数组 answer，数组中的每个元素 answer[i] 是在给出的单词列表 words 中可以作为字谜迷面 puzzles[i] 所对应的谜底的单词数目。
///
///
/// 示例：
///
/// 输入：
/// words = ["aaaa","asas","able","ability","actt","actor","access"],
/// puzzles = ["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"]
/// 输出：[1,1,3,2,4,0]
/// 解释：
/// 1 个单词可以作为 "aboveyz" 的谜底 : "aaaa"
/// 1 个单词可以作为 "abrodyz" 的谜底 : "aaaa"
/// 3 个单词可以作为 "abslute" 的谜底 : "aaaa", "asas", "able"
/// 2 个单词可以作为 "absoryz" 的谜底 : "aaaa", "asas"
/// 4 个单词可以作为 "actresz" 的谜底 : "aaaa", "asas", "actt", "access"
/// 没有单词可以作为 "gaswxyz" 的谜底，因为列表中的单词都不含字母 'g'。
///  
///
/// 提示：
///
/// 1 <= words.length <= 10^5
/// 4 <= words[i].length <= 50
/// 1 <= puzzles.length <= 10^4
/// puzzles[i].length == 7
/// words[i][j], puzzles[i][j] 都是小写英文字母。
/// 每个 puzzles[i] 所包含的字符都不重复。
///
/// 题解：
/// 作者：AC_OIer
/// 链接：https://leetcode-cn.com/problems/number-of-valid-words-for-each-puzzle/solution/xiang-jin-zhu-shi-xiang-jie-po-su-wei-yu-3cr2/
/// 来源：力扣（LeetCode）
/// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    let mut res = vec![];
    // 转用 「哈希表」来统计出所有的 word 所对应的二进制数值
    let mut map = HashMap::new();
    for w in words {
        let t = get_bin(&w);
        map.insert(t, map.get(&t).unwrap_or(&0) + 1);
    }
    // 判定每个 puzzle 有多少个谜底
    for puzzle in puzzles {
        res.push(get_cnt(&map, &puzzle));
    }
    res
}

fn get_cnt(map: &HashMap<i32, i32>, str: &String) -> i32 {
    let mut ans = 0;
    let m = str.len();
    let cs = str.as_bytes();
    // 当前 puzzle 的首个字符在二进制数值中的位置
    let first = cs[0] - b'a';
    // 枚举「除首个字母」以外的所有组合
    // 因为我们需要先固定 puzzle 的首位字母，然后枚举剩余的 6 位是什么
    // 由于是二进制，每一位共有 0 和 1 两种选择，因此共有 2^6 种可能性，也就是 2^6 = 1 << (7 - 1) = 64 种
    for i in 0..(1 << m - 1) {
        // 先将首字母提取出来
        let mut u = 1 << first;
        // 枚举「除首个字母」之后的每一位，结合上面的首个字母
        // 其实就是枚举以 str 首字母开头的字符有多少种（枚举子集）
        for j in 1..m {
            if (i >> (j - 1) & 1) != 0 {
                u += 1 << (cs[j] - b'a');
            }
        }
        // 查询这样的字符是否出现在 `words` 中，出现了多少次
        if map.contains_key(&u) {
            ans += map.get(&u).unwrap();
        }
    }
    ans
}

/// 将 str 所包含的字母用二进制标识
/// 如果 str = abz 则对应的二进制为 100...011 (共 26 位，从右往左是 a - z)
fn get_bin(str: &String) -> i32 {
    let mut t = 0;
    let cs = str.as_bytes();
    for i in 0..cs.len() {
        //每一位字符对应二进制数字中的哪一位
        let u = cs[i] - b'a';
        //如果当前位置为0，代表还没记录过，则进行记录(不重复记录)
        if (t >> u & 1) == 0 {
            t += 1 << u;
        }
    }
    t
}

#[test]
fn test() {
    let n = find_num_of_valid_words(vec!["aaaa".to_string(), "asas".to_string(), "able".to_string(), "ability".to_string(), "actt".to_string(), "actor".to_string(), "access".to_string()],
                                    vec!["aboveyz".to_string(), "abrodyz".to_string(), "abslute".to_string(), "absoryz".to_string(), "actresz".to_string(), "gaswxyz".to_string()]);
    println!("{:?}", n);
}
