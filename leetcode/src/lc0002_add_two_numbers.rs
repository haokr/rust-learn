use crate::leetcode_base::ListNode;

struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1node = l1.unwrap();
        let mut l2node = l2.unwrap();
        let mut res = Solution::make_node(l1node.val + l2node.val);
        let mut head = ListNode {
            val: 0,
            next: Some(Box::new(res.0)),
        };

        let mut curr = &mut head.next;
        while l1node.next != None || l2node.next != None || res.1 != 0 {
            let v1 = if let Some(ref v1) = l1node.next {
                let r = v1.val;
                l1node = l1node.next.unwrap();
                r
            } else {
                0
            };
            let v2 = if let Some(ref v2) = l2node.next {
                let r = v2.val;
                l2node = l2node.next.unwrap();
                r
            } else {
                0
            };
            let next_node_val = v1 + v2 + res.1;
            res = Solution::make_node(next_node_val);
            let cur_node = curr.as_mut().unwrap();
            cur_node.next = Some(Box::new(res.0));
            curr = &mut cur_node.next;
        }

        head.next
    }

    fn make_node(mut val: i32) -> (ListNode, i32) {
        let mut carry = 0;
        if val > 9 {
            carry = 1;
            val -= 10;
        }
        (ListNode::new(val), carry)
    }
}