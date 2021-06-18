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
    temp.append(&mut v.to_num_vec());
  }
  temp.sort();

  temp.to_list_node(false)
}

#[test]
fn q23_test() {
  assert_eq!(merge_k_lists(vec![None, None, None]), None);
  assert_eq!(
    merge_k_lists(vec![
      145.to_list_node(false),
      134.to_list_node(false),
      26.to_list_node(false),
    ]),
    11234456.to_list_node(false)
  );
}
