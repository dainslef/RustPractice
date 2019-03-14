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
  let (mut temp, nodes) = (vec![], nodes_to_vec(head));
  let (k, count) = (k as usize, nodes.len());

  for end in (k..=nodes.len()).step_by(k) {
    let o: Option<Vec<i32>> = nodes.get((end - k)..end).map(|v| v.into());
    if let Some(mut v) = o {
      v.reverse();
      temp.append(&mut v);
    }
  }

  if temp.len() < count {
    let o: Option<Vec<i32>> = nodes.get(temp.len()..count).map(|v| v.into());
    if let Some(mut v) = o {
      temp.append(&mut v);
    }
  }

  vec_to_nodes(temp, false)
}

#[test]
fn test_reverse_k_group() {
  assert_eq!(reverse_k_group(None, 2), None);
  assert_eq!(
    reverse_k_group(num_to_nodes(12345, false), 2),
    num_to_nodes(21435, false)
  );
  assert_eq!(
    reverse_k_group(vec_to_nodes(vec![10, 20, 1, 2, 3, 4, 5], false), 7),
    vec_to_nodes(vec![5, 4, 3, 2, 1, 20, 10], false)
  );
}
