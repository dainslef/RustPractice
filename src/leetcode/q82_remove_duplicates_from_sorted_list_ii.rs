/**
 * 82. Remove Duplicates from Sorted List II
 * https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
 *
 * Given a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list.
 *
 * Return the linked list sorted as well.
 *
 * Example 1:
 *
 * Input: 1->2->3->3->4->4->5
 * Output: 1->2->5
 * Example 2:
 *
 * Input: 1->1->1->2->3
 * Output: 2->3
 */
use super::*;

/**
 * Runtime: 0 ms, faster than 100.00% of Rust online submissions for Remove Duplicates from Sorted List II.
 * Memory Usage: 2.1 MB, less than 75.00% of Rust online submissions for Remove Duplicates from Sorted List II.
 */
fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  use std::collections::HashSet;

  let (mut nums, mut next, mut same) = (vec![], head, HashSet::new());

  while let Some(v) = next {
    if nums.last().map(|l| *l == v.val).unwrap_or(false) {
      same.insert(v.val); // record the same elements
    }
    nums.push(v.val);
    next = v.next;
  }

  // the elements in Vec place in the opposite order
  while let Some(val) = nums.pop() {
    // check if the "val" is a duplicate element
    if !same.contains(&val) {
      // only add the dissimilar element
      next = Some(Box::new(ListNode { val, next }));
    }
  }

  next
}

#[test]
fn q82_test() {
  assert_eq!(
    delete_duplicates(num_vec_to_nodes(vec![], false)),
    num_vec_to_nodes(vec![], false)
  );
  assert_eq!(
    delete_duplicates(num_vec_to_nodes(vec![1], false)),
    num_vec_to_nodes(vec![1], false)
  );
  assert_eq!(
    delete_duplicates(num_vec_to_nodes(vec![1, 1], false)),
    num_vec_to_nodes(vec![], false)
  );
  assert_eq!(
    delete_duplicates(num_vec_to_nodes(vec![1, 1, 2], false)),
    num_vec_to_nodes(vec![2], false)
  );
  assert_eq!(
    delete_duplicates(num_vec_to_nodes(vec![1, 2, 3, 3, 3, 4, 4, 5], false)),
    num_vec_to_nodes(vec![1, 2, 5], false)
  );
}
