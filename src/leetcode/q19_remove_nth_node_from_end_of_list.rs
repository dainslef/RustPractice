/**
 * Given a linked list, remove the n-th node from the end of list and return its head.
 *
 * Example:
 *
 * Given linked list: 1->2->3->4->5, and n = 2.
 *
 * After removing the second node from the end, the linked list becomes 1->2->3->5.
 * Note:
 *
 * Given n will always be valid.
 *
 * Follow up:
 *
 * Could you do this in one pass?
 */
use super::*;

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
  let mut vals = nodes_to_vec(head);
  vals.reverse();
  vals.remove(n as usize - 1);
  vec_to_nodes(vals, true)
}

#[test]
fn test_remove_nth_from_end() {
  assert_eq!(remove_nth_from_end(num_to_nodes(1, false), 1), None);
  assert_eq!(remove_nth_from_end(num_to_nodes(12345, false), 2), num_to_nodes(1235, false));
}
