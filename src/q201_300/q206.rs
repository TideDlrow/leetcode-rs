

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
///反转一个单链表。
///
/// 示例:
/// ```
/// 输入: 1->2->3->4->5->NULL
/// 输出: 5->4->3->2->1->NULL
/// ```
/// 进阶:
/// 你可以迭代或递归地反转链表。你能否用两种方法解决这道题？
///
/// 来源：力扣（LeetCode）
/// 链接：https:///leetcode-cn.com/problems/reverse-linked-list
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    ///翻转链表的关键，是保证不会有两个变量抢一个节点，同时所有的节点都能被访问到。
    /// 从头部开始循环翻转，循环时需要能拿到之前的一个节点和之后的一个节点，这样把当前节点的 next 指向前一个节点，然后把当前节点指向下一个节点，继续下一轮循环即可。
    /// 下一个节点可通过当前节点的 next 用 take() 取出，因此建立一个临时变量 prev 存储上一个节点，初始值自然为 None。
    let mut prev = None; // 上一个节点
    let mut cur = head; // 当前节点
    while let Some(mut _node) = cur { // 用take置换next中的节点需要 mut
        cur = _node.next.take(); // 换出 next 作为下一次的 cur
        _node.next = prev; // 把next指向前一个节点
        prev = Some(_node);  // 更新 prev
    }
    return prev;  // 跳出循环时,prev就是翻转后的头节点
}
