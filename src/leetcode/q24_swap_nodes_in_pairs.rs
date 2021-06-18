/**
 * Given a linked list, swap every two adjacent nodes and return its head.
 *
 * You may not modify the values in the list's nodes, only nodes itself may be changed.
 *
 *
 *
 * Example:
 *
 * Given 1->2->3->4, you should return the list as 2->1->4->3.
 */
use super::*;

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut vals = head.to_num_vec();

  for i in (1..vals.len()).step_by(2) {
    let temp = vals[i];
    vals[i] = vals[i - 1];
    vals[i - 1] = temp;
  }

  vals.to_list_node(false)
}

#[test]
fn q24_test() {
  assert_eq!(
    swap_pairs(1234.to_list_node(false)),
    2143.to_list_node(false)
  );
  assert_eq!(swap_pairs(1.to_list_node(false)), 1.to_list_node(false));
}
