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
  let mut vals = nodes_to_num_vec(head);

  for i in (1..vals.len()).step_by(2) {
    let temp = vals[i];
    vals[i] = vals[i - 1];
    vals[i - 1] = temp;
  }

  num_vec_to_nodes(vals, false)
}

#[test]
fn test_swap_pairs() {
  assert_eq!(
    swap_pairs(num_to_nodes(1234, false)),
    num_to_nodes(2143, false)
  );
  assert_eq!(swap_pairs(num_to_nodes(1, false)), num_to_nodes(1, false));
}
