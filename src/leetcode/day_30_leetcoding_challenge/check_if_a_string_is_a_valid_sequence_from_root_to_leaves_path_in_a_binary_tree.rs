/**
 * Given a binary tree where each path going from the root to any leaf form a valid sequence, check if a given string is a valid sequence in such binary tree.
 *
 * We get the given string from the concatenation of an array of integers arr and the concatenation of all values of the nodes along a path results in a sequence in the given binary tree.
 *
 *
 *
 * Example 1:
 *
 *
 *
 * Input: root = [0,1,0,0,1,0,null,null,1,0,0], arr = [0,1,0,1]
 * Output: true
 * Explanation:
 * The path 0 -> 1 -> 0 -> 1 is a valid sequence (green color in the figure).
 * Other valid sequences are:
 * 0 -> 1 -> 1 -> 0
 * 0 -> 0 -> 0
 * Example 2:
 *
 *
 *
 * Input: root = [0,1,0,0,1,0,null,null,1,0,0], arr = [0,0,1]
 * Output: false
 * Explanation: The path 0 -> 0 -> 1 does not exist, therefore it is not even a sequence.
 * Example 3:
 *
 *
 *
 * Input: root = [0,1,0,0,1,0,null,null,1,0,0], arr = [0,1,1]
 * Output: false
 * Explanation: The path 0 -> 1 -> 1 is a sequence, but it is not a valid sequence.
 *
 *
 * Constraints:
 *
 * 1 <= arr.length <= 5000
 * 0 <= arr[i] <= 9
 * Each node's value is between [0 - 9].
 */
use super::super::*;

fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
  fn recurse(node: &Option<Rc<RefCell<TreeNode>>>, arr: &Vec<i32>, mut current: Vec<i32>) -> bool {
    if let Some(n) = node {
      let n = n.borrow();
      current.push(n.val);
      match (&n.left, &n.right) {
        (l @ Some(_), None) => recurse(l, arr, current),
        (None, r @ Some(_)) => recurse(r, arr, current),
        (l, r) => recurse(l, arr, current.clone()) || recurse(r, arr, current),
      }
    } else {
      arr == &current
    }
  }
  recurse(&root, &arr, Default::default())
}

#[test]
fn test_is_valid_sequence() {
  assert_eq!(
    is_valid_sequence(TreeNode::from(vec![Some(8)]), vec![8]),
    true
  );
  assert_eq!(
    is_valid_sequence(
      TreeNode::from(vec![
        Some(3),
        Some(0),
        None,
        Some(2),
        None,
        None,
        Some(2),
        Some(9),
        Some(3)
      ]),
      vec![3, 0]
    ),
    false
  );
  assert_eq!(
    is_valid_sequence(
      TreeNode::from(vec![
        Some(8),
        Some(3),
        None,
        Some(2),
        Some(1),
        Some(5),
        Some(4)
      ]),
      vec![8]
    ),
    false
  );
  assert_eq!(
    is_valid_sequence(
      TreeNode::from(vec![
        Some(0),
        Some(1),
        Some(0),
        Some(0),
        Some(1),
        Some(0),
        None,
        None,
        Some(1),
        Some(0),
        Some(0)
      ]),
      vec![0, 1, 0, 1]
    ),
    true
  );
  assert_eq!(
    is_valid_sequence(
      TreeNode::from(vec![
        Some(0),
        Some(1),
        Some(0),
        Some(0),
        Some(1),
        Some(0),
        None,
        None,
        Some(1),
        Some(0),
        Some(0)
      ]),
      vec![0, 0, 1]
    ),
    false
  );
  assert_eq!(
    is_valid_sequence(
      TreeNode::from(vec![
        Some(0),
        Some(1),
        Some(0),
        Some(0),
        Some(1),
        Some(0),
        None,
        None,
        Some(1),
        Some(0),
        Some(0)
      ]),
      vec![0, 1, 1]
    ),
    false
  );
}
