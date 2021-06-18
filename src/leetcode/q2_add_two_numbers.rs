/**
 * 2. Add Two Numbers
 * https://leetcode.com/problems/add-two-numbers/
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 * Example:
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 */
use super::*;

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let (mut l1, mut l2) = (&l1, &l2);
  let (mut temp, mut carry) = (vec![], false);

  let mut deal_result = |num| {
    // compute the result by carry
    let result = num + carry as i32;
    // if the result is larger than 10, set the carry
    carry = result >= 10;
    temp.push(if carry { result - 10 } else { result });
  };

  loop {
    match (l1, l2) {
      (Some(n1), Some(n2)) => {
        l1 = &n1.next;
        l2 = &n2.next;
        deal_result(n1.val + n2.val);
      }
      (Some(n1), None) => {
        l1 = &n1.next;
        deal_result(n1.val);
      }
      (None, Some(n2)) => {
        l2 = &n2.next;
        deal_result(n2.val);
      }
      (None, None) => {
        // if the list is ended, check the carry
        if carry {
          temp.push(carry as i32);
        }
        break temp.to_list_node(false);
      }
    }
  }
}

#[test]
fn q2_test() {
  assert_eq!(
    add_two_numbers(1234.to_list_node(true), 11111.to_list_node(true)),
    12345.to_list_node(true)
  );
  assert_eq!(
    add_two_numbers(9.to_list_node(true), 999999991.to_list_node(true)),
    1000000000.to_list_node(true)
  );
}
