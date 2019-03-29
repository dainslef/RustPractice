/**
 * You are given two non-empty linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 * Follow up:
 * What if you cannot modify the input lists? In other words, reversing the lists is not allowed.
 *
 * Example:
 *
 * Input: (7 -> 2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 8 -> 0 -> 7
 */
use super::*;

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  // get the long number vector, the short number vector and the offset
  let (long, short, offset) = {
    let (v1, v2) = (nodes_to_vec(l1), nodes_to_vec(l2));
    let (len1, len2) = (v1.len(), v2.len());
    if len1 > len2 {
      (v1, v2, len1 - len2)
    } else {
      (v2, v1, len2 - len1)
    }
  };
  let (mut next, mut carry) = (None, false);

  for i in (0..long.len()).rev() {
    // check the offset, and calculate the sum of the values of the nodes at the same position in two lists
    let mut val = if i >= offset {
      long[i] + short[i - offset]
    } else {
      long[i]
    } + carry as i32;

    // check if the value needs to be carried
    carry = val >= 10;
    if carry {
      // recalculate the value after the value is carried
      val -= 10;
    }

    next = Some(Box::new(ListNode { val, next }));
  }

  // if the list is ended, check and set the carry
  if carry {
    next = Some(Box::new(ListNode {
      val: carry as i32,
      next,
    }));
  }

  next
}

#[test]
fn test_add_two_numbers_two() {
  assert_eq!(
    add_two_numbers(
      num_to_nodes(7243, false),
      num_to_nodes(564, false)
    ),
    num_to_nodes(7807, false)
  );
  assert_eq!(
    add_two_numbers(
      num_to_nodes(999, false),
      num_to_nodes(1, false)
    ),
    num_to_nodes(1000, false)
  );
  assert_eq!(
    add_two_numbers(
      num_to_nodes(2, false),
      num_to_nodes(10998, false)
    ),
    num_to_nodes(11000, false)
  );
}
