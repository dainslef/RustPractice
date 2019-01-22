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
  let (mut temp, mut carry) = (None, 0);

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
    let next = temp;
    temp = Some(Box::new(ListNode { val, next }));
  }

  if carry > 0 {
    let next = temp;
    temp = Some(Box::new(ListNode { val: carry, next }));
  }

  temp
}

#[test]
fn test_add_two_numbers_two() {
  fn build_nodes(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut temp: &mut Option<Box<ListNode>> = &mut None;
    for i in vec {
      if let Some(n) = temp {
        n.next = Some(Box::new(ListNode::new(i)));
        temp = &mut n.next;
      } else {
        head = Some(Box::new(ListNode::new(i)));
        temp = &mut head;
      }
    }
    head
  }

  assert_eq!(
    add_two_numbers(build_nodes(vec![7, 2, 4, 3]), build_nodes(vec![5, 6, 4])),
    build_nodes(vec![7, 8, 0, 7])
  );
  assert_eq!(
    add_two_numbers(build_nodes(vec![9, 9, 9]), build_nodes(vec![1])),
    build_nodes(vec![1, 0, 0, 0])
  );
  assert_eq!(
    add_two_numbers(build_nodes(vec![2]), build_nodes(vec![1, 0, 9, 9, 8])),
    build_nodes(vec![1, 1, 0, 0, 0])
  );
}
