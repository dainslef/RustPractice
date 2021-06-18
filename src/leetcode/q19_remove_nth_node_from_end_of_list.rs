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
  let mut vals = head.to_num_vec();
  vals.reverse();
  vals.remove(n as usize - 1);
  vals.to_list_node(true)
}

#[test]
fn q19_test() {
  assert_eq!(remove_nth_from_end(1.to_list_node(false), 1), None);
  assert_eq!(
    remove_nth_from_end(12345.to_list_node(false), 2),
    1235.to_list_node(false)
  );
}
