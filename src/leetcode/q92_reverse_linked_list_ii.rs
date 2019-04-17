/**
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
  use std::mem::swap;

  let mut nodes = vec![];
  let mut current = head;
  while let Some(v) = current.as_mut() {
    let mut node = None;
    swap(&mut v.next, &mut node);
    nodes.push(current);
    current = node;
  }

  let mut result = None;
  let mut node = &mut result;
  for i in 1..=nodes.len() {
    let (m, n) = (m as usize, n as usize);
    let i = if i > n || i < m { i } else { n - i + m };
    swap(node, &mut nodes[i - 1]);
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
