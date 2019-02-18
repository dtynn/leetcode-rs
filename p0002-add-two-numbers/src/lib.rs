// https://leetcode.com/problems/add-two-numbers/
//
// corner cases:
// l1 l2 都没有后续节点, 但是 remain 不为 0, 即存在上一次计算的进位

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            return l2;
        }

        if l2.is_none() {
            return l1;
        }

        fn get_val(ln: Option<&Box<ListNode>>) -> i32 {
            ln.and_then(|node| Some(node.val)).unwrap_or(0)
        }

        let mut ln1 = l1;
        let mut ln2 = l2;
        let mut remain = 0;

        let mut nodes = Vec::new();

        {
            let mut sum = get_val(ln1.as_ref()) + get_val(ln2.as_ref());
            if sum >= 10 {
                sum -= 10;
                remain = 1;
            }

            nodes.push(ListNode::new(sum));
        }

        loop {
            ln1 = ln1.and_then(|n| n.next);
            ln2 = ln2.and_then(|n| n.next);
            let mut sum = remain + get_val(ln1.as_ref()) + get_val(ln2.as_ref());
            remain = 0;
            if sum == 0 && ln1.is_none() && ln2.is_none() {
                break;
            }

            if sum >= 10 {
                sum -= 10;
                remain = 1;
            }

            nodes.push(ListNode::new(sum));
        }

        while nodes.len() > 1 {
            let last = nodes.pop().unwrap();
            nodes.last_mut().and_then(|n| {
                n.next = Some(Box::new(last));
                Some(())
            });
        }

        nodes.pop().and_then(|n| Some(Box::new(n)))
    }
}
