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
fn delete_duplicates_by_value(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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

/**
 * Runtime: 0 ms, faster than 100.00% of Rust online submissions for Remove Duplicates from Sorted List II.
 * Memory Usage: 2 MB, less than 71.43% of Rust online submissions for Remove Duplicates from Sorted List II.
 */
fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut last: *mut Option<Box<ListNode>> = &mut None;
  let mut current: *mut Option<Box<ListNode>> = &mut head;
  let mut same: Option<i32> = None;

  unsafe {
    while let Some(c) = &mut *current {
      macro_rules! exchange {
        ($current: expr, $empty_same: expr) => {{
          if $empty_same {
            same = Some(c.val);
          }
          $current = c.next.take();
          current = &mut $current;
        }};
      }
      match (&mut *last, &mut c.next, same) {
        (Some(l), _, Some(s)) if c.val == s => exchange!(l.next, false),
        (None, _, Some(s)) if c.val == s => exchange!(head, false),
        (Some(l), Some(n), _) if c.val == n.val => exchange!(l.next, true),
        (None, Some(n), _) if c.val == n.val => exchange!(head, true),
        _ => {
          last = current;
          current = &mut c.next;
        }
      }
    }

    head
  }
}

#[test]
fn q82_test() {
  fn test(delete_duplicates: impl Fn(Option<Box<ListNode>>) -> Option<Box<ListNode>>) {
    assert_eq!(
      delete_duplicates(vec![1, 2, 3, 3, 3, 4, 4, 5].to_list_node(false)),
      vec![1, 2, 5].to_list_node(false)
    );
    assert_eq!(
      delete_duplicates(vec![1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 5].to_list_node(false)),
      vec![5].to_list_node(false)
    );
    assert_eq!(
      delete_duplicates(vec![].to_list_node(false)),
      vec![].to_list_node(false)
    );
    assert_eq!(
      delete_duplicates(vec![1].to_list_node(false)),
      vec![1].to_list_node(false)
    );
    assert_eq!(
      delete_duplicates(vec![1, 1].to_list_node(false)),
      vec![].to_list_node(false)
    );
    assert_eq!(
      delete_duplicates(vec![1, 1, 2].to_list_node(false)),
      vec![2].to_list_node(false)
    );
  }

  test(delete_duplicates);
  test(delete_duplicates_by_value);
}
