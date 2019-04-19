/**
 * Merge k sorted linked lists and return it as one sorted list. Analyze and describe its complexity.
 *
 * Example:
 *
 * Input:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * Output: 1->1->2->3->4->4->5->6
 */
use super::*;

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
  let mut temp = vec![];

  for v in lists {
    temp.append(&mut nodes_to_num_vec(v));
  }
  temp.sort();

  num_vec_to_nodes(temp, false)
}

#[test]
fn test_merge_k_lists() {
  assert_eq!(merge_k_lists(vec![None, None, None]), None);
  assert_eq!(
    merge_k_lists(vec![
      num_to_nodes(145, false),
      num_to_nodes(134, false),
      num_to_nodes(26, false),
    ]),
    num_to_nodes(11234456, false)
  );
}
