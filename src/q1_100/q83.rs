
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

///给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 没有重复出现 的数字。
///
/// 示例 1:
///
/// 输入: 1->1->2
/// 输出: 1->2
/// 示例 2:
///
/// 输入: 1->1->2->2->3
/// 输出: 1->2->3
pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut cur = head.as_mut();
    while cur.is_some() && cur.as_ref().unwrap().next.is_some() {
        if cur.as_ref().unwrap().val == cur.as_ref().unwrap().next.as_ref().unwrap().val
        {
            let next = cur.as_mut().unwrap().next.as_mut().unwrap().next.take();
            cur.as_mut().unwrap().next = next;
        } else {
            cur = cur.unwrap().next.as_mut();
        }
    }
    head
}

#[test]
fn test() {
    let node1 = ListNode { val: 3, next: None };
    let node2 = ListNode { val: 2, next: Some(Box::new(node1)) };
    let node3 = ListNode { val: 1, next: Some(Box::new(node2)) };
    let node4 = ListNode { val: 1, next: Some(Box::new(node3)) };
    let node5 = ListNode { val: 1, next: Some(Box::new(node4)) };
    println!("{:?}", delete_duplicates(Some(Box::new(node5))));
}
