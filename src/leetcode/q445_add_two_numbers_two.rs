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
use super::ListNode;

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  fn get_vec(l: Option<Box<ListNode>>) -> Vec<i32> {
    let (mut vec, mut temp) = (vec![], &l);
    while let Some(n) = temp {
      vec.push(n.val);
      temp = &n.next;
    }
    vec
  }

  let (v1, v2) = (get_vec(l1), get_vec(l2));
  let (v1, v2) = if v1.len() > v2.len() {
    (v1, v2)
  } else {
    (v2, v1)
  };
  let offset = v1.len() - v2.len();
  let (mut next, mut carry) = (None, 0);

  for i in (0..v1.len()).rev() {
    let mut val = if i >= offset {
      v1[i] + v2[i - offset]
    } else {
      v1[i]
    } + carry;
    carry = if val >= 10 {
      val -= 10;
      1
    } else {
      0
    };
    next = Some(Box::new(ListNode { val, next }));
  }

  if carry > 0 {
    next = Some(Box::new(ListNode { val: carry, next }));
  }

  next
}

#[test]
fn test_add_two_numbers_two() {
  assert_eq!(
    add_two_numbers(
      super::build_nodes(7243, false),
      super::build_nodes(564, false)
    ),
    super::build_nodes(7807, false)
  );
  assert_eq!(
    add_two_numbers(super::build_nodes(999, false), super::build_nodes(1, false)),
    super::build_nodes(1000, false)
  );
  assert_eq!(
    add_two_numbers(
      super::build_nodes(2, false),
      super::build_nodes(10998, false)
    ),
    super::build_nodes(11000, false)
  );
}
