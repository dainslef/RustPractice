/**
 * 543. Diameter of Binary Tree
 * https://leetcode.com/problems/diameter-of-binary-tree/
 *
 * Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
 *
 * Example:
 * Given a binary tree
 *           1
 *          / \
 *         2   3
 *        / \
 *       4   5
 * Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].
 *
 * Note: The length of path between two nodes is represented by the number of edges between them.
 */
use super::*;

fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
  // return (height, distance)
  fn height_and_distance(node: Rc<RefCell<TreeNode>>, height: i32) -> (i32, i32) {
    // compute the info of child node
    macro_rules! compute {
      ($node: expr) => {{
        let mut result = (height, 0); // if no child node, the height doesn't change and the distance is zero
        if let Some(v) = $node {
          result = height_and_distance(v.clone(), height + 1);
        }
        result
      }};
    }
    let (left_height, left_distance) = compute!(&node.borrow().left);
    let (right_height, right_distance) = compute!(&node.borrow().right);
    (
      left_height.max(right_height),
      // current distance is (left child height - current height) + (right child height - current hight)
      // the max distance is the max of current distance and child distance
      (left_height + right_height - 2 * height)
        .max(left_distance)
        .max(right_distance),
    )
  }
  root.map(|v| height_and_distance(v, 0).1).unwrap_or(0)
}

#[test]
fn q543_test() {
  assert_eq!(
    diameter_of_binary_tree(TreeNode::from(vec![
      Some(1),
      Some(2),
      Some(3),
      Some(4),
      Some(5)
    ])),
    3
  );
  assert_eq!(
    diameter_of_binary_tree(TreeNode::from(vec![
      Some(4),
      Some(-7),
      Some(-3),
      None,
      None,
      Some(-9),
      Some(-3),
      Some(9),
      Some(-7),
      Some(-4),
      None,
      Some(6),
      None,
      Some(-6),
      Some(-6),
      None,
      None,
      Some(0),
      Some(6),
      Some(5),
      None,
      Some(9),
      None,
      None,
      Some(-1),
      Some(-4),
      None,
      None,
      None,
      Some(-2)
    ])),
    8
  );
}
