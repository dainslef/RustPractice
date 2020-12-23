/**
 * 86. Partition List
 * https://leetcode.com/problems/partition-list/
 *
 * Given a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
 *
 * You should preserve the original relative order of the nodes in each of the two partitions.
 *
 * Example:
 *
 * Input: head = 1->4->3->2->5->2, x = 3
 * Output: 1->2->2->4->3->5
 */
use super::*;

/**
 * Runtime: 0 ms, faster than 100.00% of Rust online submissions for Partition List.
 * Memory Usage: 2.1 MB, less than 100.00% of Rust online submissions for Partition List.
 */
fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
  let nums = nodes_to_num_vec(head);
  let (mut less, mut great) = (vec![], vec![]);

  for v in nums {
    if v < x { &mut less } else { &mut great }.push(v);
  }

  less.append(&mut great);
  num_vec_to_nodes(less, false)
}

#[test]
fn q86_test() {
  assert_eq!(
    partition(num_vec_to_nodes(vec![1, 4, 3, 2, 5, 2], false), 3),
    num_vec_to_nodes(vec![1, 2, 2, 4, 3, 5], false)
  );
  assert_eq!(
    partition(
      num_vec_to_nodes(vec![1, 4, 3, 2, 5, 2, 1, 7, 6, 9, 8], false),
      3
    ),
    num_vec_to_nodes(vec![1, 2, 2, 1, 4, 3, 5, 7, 6, 9, 8], false)
  );
}
