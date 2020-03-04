/**
 * 61. Rotate List
 * https://leetcode.com/problems/rotate-list/
 *
 * Given a linked list, rotate the list to the right by k places, where k is non-negative.
 *
 * Example 1:
 *
 * Input: 1->2->3->4->5->NULL, k = 2
 * Output: 4->5->1->2->3->NULL
 * Explanation:
 * rotate 1 steps to the right: 5->1->2->3->4->NULL
 * rotate 2 steps to the right: 4->5->1->2->3->NULL
 * Example 2:
 *
 * Input: 0->1->2->NULL, k = 4
 * Output: 2->0->1->NULL
 * Explanation:
 * rotate 1 steps to the right: 2->0->1->NULL
 * rotate 2 steps to the right: 1->2->0->NULL
 * rotate 3 steps to the right: 0->1->2->NULL
 * rotate 4 steps to the right: 2->0->1->NULL
 */
use super::*;

fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
  let mut nodes = nodes_to_num_vec(head);
  if nodes.is_empty() {
    return None;
  }
  let i = nodes.len() - k as usize % nodes.len(); // compute the offset
  let mut out: Vec<i32> = nodes.drain(i..).collect();
  out.append(&mut nodes);
  num_vec_to_nodes(out, false)
}

#[test]
fn q61_test() {
  assert_eq!(
    rotate_right(num_to_nodes(12345, false), 2),
    num_to_nodes(45123, false)
  );
  assert_eq!(
    rotate_right(num_to_nodes(312, false), 4),
    num_to_nodes(231, false)
  );
  assert_eq!(
    rotate_right(num_vec_to_nodes(vec![], false), 2),
    num_vec_to_nodes(vec![], false)
  );
  assert_eq!(
    rotate_right(num_vec_to_nodes(vec![1], false), 4),
    num_vec_to_nodes(vec![1], false)
  );
}
