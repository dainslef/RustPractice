/**
 * Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.
 *
 * k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes in the end should remain as it is.
 *
 * Example:
 *
 * Given this linked list: 1->2->3->4->5
 *
 * For k = 2, you should return: 2->1->4->3->5
 *
 * For k = 3, you should return: 3->2->1->4->5
 *
 * Note:
 *
 * Only constant extra memory is allowed.
 * You may not alter the values in the list's nodes, only nodes itself may be changed.
 */
use super::*;

fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
  let (mut temp, mut merage_size, nodes) = (vec![], 0, head.to_num_vec());
  let (k, count) = (k as usize, nodes.len());

  let mut merage = |range: std::ops::Range<usize>, reverse| {
    let merage_val: Option<Vec<i32>> = nodes.get(range).map(|v| v.into());
    if let Some(mut v) = merage_val {
      if reverse {
        v.reverse();
      }
      temp.append(&mut v);
    }
  };

  for i in (k..=count).step_by(k) {
    merage_size = i;
    merage((i - k)..i, true);
  }

  if merage_size < count {
    merage(merage_size..count, false);
  }

  temp.to_list_node(false)
}

#[test]
fn q25_test() {
  assert_eq!(reverse_k_group(None, 2), None);
  assert_eq!(
    reverse_k_group(12345.to_list_node(false), 2),
    21435.to_list_node(false)
  );
  assert_eq!(
    reverse_k_group(vec![10, 20, 1, 2, 3, 4, 5].to_list_node(false), 7),
    vec![5, 4, 3, 2, 1, 20, 10].to_list_node(false)
  );
}
