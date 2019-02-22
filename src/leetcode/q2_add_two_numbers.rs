/**
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
use super::ListNode;

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let (mut l1, mut l2) = (&l1, &l2);
  let (mut temp, mut vec, mut carry) = (None, vec![], 0);

  let mut deal_result = |num| {
    // compute the result by carry
    let result = num + carry;

    // if the result is larger than 10, set the carry
    carry = if result >= 10 {
      vec.push(result - 10);
      1
    } else {
      vec.push(result);
      0
    }
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
        if carry > 0 {
          vec.push(carry);
        }
        break {
          vec.reverse();
          for val in vec {
            let next = temp;
            temp = Some(Box::new(ListNode { val, next }));
          }
          temp
        };
      }
    }
  }
}

#[test]
fn test_add_two_numbers() {
  assert_eq!(add_two_numbers(super::build_nodes(1234, true), super::build_nodes(11111, true)),
    super::build_nodes(12345, true)
  );
  assert_eq!(add_two_numbers(super::build_nodes(9, true), super::build_nodes(999999991, true)),
    super::build_nodes(1000000000, true)
  );
}
