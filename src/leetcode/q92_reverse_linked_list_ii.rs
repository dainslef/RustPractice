/**
 * 92. Reverse Linked List II
 * https://leetcode.com/problems/reverse-linked-list-ii/
 *
 * Reverse a linked list from position m to n. Do it in one-pass.
 *
 * Note: 1 ≤ m ≤ n ≤ length of list.
 *
 * Example:
 *
 * Input: 1->2->3->4->5->NULL, m = 2, n = 4
 * Output: 1->4->3->2->5->NULL
 */
use super::*;

fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
  let mut result = None;
  let (mut node, mut nodes) = (&mut result, nodes_to_node_vec(head));

  for i in 1..=nodes.len() {
    let (m, n) = (m as usize, n as usize);
    let index = if i > n || i < m { i } else { n - i + m };
    std::mem::swap(node, &mut nodes[index - 1]);
    if let Some(v) = node {
      node = &mut v.next;
    }
  }

  result
}

#[test]
fn test_reverse_between() {
  assert_eq!(
    reverse_between(num_to_nodes(123456, false), 2, 4),
    num_to_nodes(143256, false)
  );
  assert_eq!(
    reverse_between(num_to_nodes(12345, false), 1, 4),
    num_to_nodes(43215, false)
  );
}
