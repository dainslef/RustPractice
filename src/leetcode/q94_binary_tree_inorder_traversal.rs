/**
 * 94. Binary Tree Inorder Traversal
 * https://leetcode.com/problems/binary-tree-inorder-traversal/
 *
 * Given the root of a binary tree, return the inorder traversal of its nodes' values.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: root = [1,null,2,3]
 * Output: [1,3,2]
 * Example 2:
 *
 * Input: root = []
 * Output: []
 * Example 3:
 *
 * Input: root = [1]
 * Output: [1]
 * Example 4:
 *
 *
 * Input: root = [1,2]
 * Output: [2,1]
 * Example 5:
 *
 *
 * Input: root = [1,null,2]
 * Output: [1,2]
 *
 *
 * Constraints:
 *
 * The number of nodes in the tree is in the range [0, 100].
 * -100 <= Node.val <= 100
 */
use super::*;

/**
 * Runtime: 0 ms, faster than 100.00% of Rust online submissions for Binary Tree Inorder Traversal.
 * Memory Usage: 1.9 MB, less than 93.55% of Rust online submissions for Binary Tree Inorder Traversal.
 *
 * Depth First Traversals:
 * Inorder (Left, Root, Right)
 * Preorder (Root, Left, Right)
 * Postorder (Left, Right, Root)
 */
fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let mut temp = vec![];

  if let Some(v) = root {
    let mut content = v.borrow_mut();
    let (left, right) = (content.left.take(), content.right.take());
    // inorder means left-root-right
    temp.append(&mut inorder_traversal(left)); // left
    temp.push(content.val); // root
    temp.append(&mut inorder_traversal(right)); // right
  }

  temp
}

#[test]
fn q94_test() {
  assert_eq!(
    inorder_traversal(TreeNode::from(vec![Some(1), None, Some(2), Some(3)])),
    [1, 3, 2]
  );
  assert_eq!(inorder_traversal(TreeNode::from(vec![])), []);
  assert_eq!(inorder_traversal(TreeNode::from(vec![Some(1)])), [1]);
  assert_eq!(
    inorder_traversal(TreeNode::from(vec![Some(1), Some(2)])),
    [2, 1]
  );
  assert_eq!(
    inorder_traversal(TreeNode::from(vec![Some(1), None, Some(2)])),
    [1, 2]
  );
}
