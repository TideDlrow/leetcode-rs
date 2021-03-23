use crate::q301_400::q341::NestedInteger::List;

///给你一个嵌套的整型列表。请你设计一个迭代器，使其能够遍历这个整型列表中的所有整数。
///
/// 列表中的每一项或者为一个整数，或者是另一个列表。其中列表的元素也可能是整数或是其他列表。
///
///  
///
/// 示例 1:
///
/// 输入: [[1,1],2,[1,1]]
/// 输出: [1,1,2,1,1]
/// 解释: 通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,1,2,1,1]。
/// 示例 2:
///
/// 输入: [1,[4,[6]]]
/// 输出: [1,4,6]
/// 解释: 通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,4,6]。
///
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    data: Vec<i32>,
    index: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut data = vec![];
        NestedIterator::p(&mut data, &nested_list);
        // let index = data.len();
        Self {
            data,
            index: 0,
        }
    }
    fn p(list: &mut Vec<i32>, nested_list: &Vec<NestedInteger>) {
        for item in nested_list.iter() {
            if let NestedInteger::Int(int) = item {
                list.push(*int);
            } else if let NestedInteger::List(list1) = item {
                NestedIterator::p(list, list1);
            }
        }
    }

    fn next(&mut self) -> i32 {
        let index = self.index;
        self.index += 1;
        return self.data[index];
    }

    fn has_next(&self) -> bool {
        return self.index < self.data.len();
    }
}

#[test]
fn test() {
    let m1 = NestedInteger::Int(1);
    let m2 = NestedInteger::Int(2);
    let m3 = NestedInteger::Int(3);
    let m4 = NestedInteger::List(vec![m2, m3]);
    let m5 = vec![m1, m4];

    let mut n = NestedIterator::new(m5);
    while n.has_next() {
        println!("{}", n.next());
    }
}
