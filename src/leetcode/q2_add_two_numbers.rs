/**
* You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

* You may assume the two numbers do not contain any leading zero, except the number 0 itself.

* Example:

* Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
* Output: 7 -> 0 -> 8
* Explanation: 342 + 465 = 807.
*/

#[derive(PartialEq, Eq, Debug)]
struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let (mut l1, mut l2) = (&l1, &l2);
  let (mut temp, mut vec, mut is_carry) = (None, vec![], false);

  let mut deal_result = |num: i32| {
    // compute the result by carry
    let result = num + if is_carry { 1 } else { 0 };

    // if the result is larger than 10, set the carry
    is_carry = if result >= 10 {
      vec.push(result - 10);
      true
    } else {
      vec.push(result);
      false
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
        if is_carry {
          vec.push(1);
        }
        break {
          vec.reverse();
          for i in vec {
            let next = temp;
            temp = Some(Box::new(ListNode { val: i, next }));
          }
          temp
        };
      }
    }
  }
}

#[test]
fn test_add_two_numbers() {
  fn build_nodes(mut num: i32) -> Option<Box<ListNode>> {
    let (mut temp, mut vec) = (None, vec![]);

    while num / 10 > 0 {
      vec.push(num % 10);
      num /= 10;
    }
    vec.push(num % 10);
    vec.reverse();

    for i in vec {
      let next = temp;
      temp = Some(Box::new(ListNode { val: i, next }));
    }

    temp
  }

  println!(
    "{:?}",
    add_two_numbers(build_nodes(1234), build_nodes(11111))
  );
  println!(
    "{:?}",
    add_two_numbers(build_nodes(9), build_nodes(999999991))
  );
}
