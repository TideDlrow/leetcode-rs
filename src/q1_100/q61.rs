/// Definition for singly-linked list.
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

///给定一个链表，旋转链表，将链表每个节点向右移动k个位置，其中k是非负数。
///
/// 示例1:
/// ```
/// 输入: 1->2->3->4->5->NULL, k = 2
/// 输出: 4->5->1->2->3->NULL
/// 解释:
/// 向右旋转 1 步: 5->1->2->3->4->NULL
/// 向右旋转 2 步: 4->5->1->2->3->NULL
/// ```
/// 示例2:
/// ```
/// 输入: 0->1->2->NULL, k = 4
/// 输出: 2->0->1->NULL
/// 解释:
/// 向右旋转 1 步: 2->0->1->NULL
/// 向右旋转 2 步: 1->2->0->NULL
/// 向右旋转 3 步:0->1->2->NULL
/// 向右旋转 4 步:2->0->1->NULL
/// ```

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // 思路为：
    // 1. 如果k超过链表长度，则先取余
    // 2. 把从后往前的第k个节点的前一个节点的next置null  把原先最后的一个节点的next指向第一个节点
    // 3. 返回(原先)从后往前的第k个节点
    let mut head = head;
    if k == 0 || head.is_none() {
        return head;
    }

    let mut ref_node = head.as_ref();
    let mut len = 0;
    while let Some(node) = ref_node {
        len += 1;
        ref_node = node.next.as_ref();
    }

    let k = len - (k % len); // 注意要先取余
    if len == 1 || k == len {
        return head;
    }

    // 找到分割的位置
    let mut ref_node = head.as_mut();
    let mut i = 0;
    let mut new_head: Option<Box<ListNode>> = None;

    while let Some(node) = ref_node {
        if i == k - 1 {
            new_head = node.next.take();
            break;
        } else {
            ref_node = node.next.as_mut();
        }
        i += 1;
    }

    let mut ref_node = new_head.as_mut();
    // 重新拼接两段链表
    while let Some(node) = ref_node {
        if node.next.is_none() {
            // 将前面一段链表接到后面的一段链表里
            node.next.replace(head.unwrap());
            // node.next =head;
            break;
        }
        ref_node = node.next.as_mut();
    }

    new_head
}

#[test]
fn test() {}
