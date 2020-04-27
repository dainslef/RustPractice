/**
 * 1008. Construct Binary Search Tree from Preorder Traversal
 * https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/
 *
 * Return the root node of a binary search tree that matches the given preorder traversal.
 *
 * (Recall that a binary search tree is a binary tree where for every node, any descendant of node.left has a value < node.val, and any descendant of node.right has a value > node.val.  Also recall that a preorder traversal displays the value of the node first, then traverses node.left, then traverses node.right.)
 *
 *
 *
 * Example 1:
 *
 * Input: [8,5,1,7,10,12]
 * Output: [8,5,10,1,7,null,12]
 *
 *
 *
 * Note:
 *
 * 1 <= preorder.length <= 100
 * The values of preorder are distinct.
 */
use super::*;

fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
  fn recurse(preorder: &Vec<i32>, max: i32, i: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
    if *i == preorder.len() || preorder[*i] > max {
      return None;
    }
    let mut n = TreeNode::new(preorder[*i]);
    *i += 1;
    n.left = recurse(preorder, n.val, i);
    n.right = recurse(preorder, max, i);
    Some(Rc::new(RefCell::new(n)))
  }
  recurse(&preorder, i32::MAX, &mut 0)
}

#[test]
fn q1008_test() {
  assert_eq!(
    bst_from_preorder(vec![8, 5, 1, 7, 10, 12]),
    TreeNode::from(vec![
      Some(8),
      Some(5),
      Some(10),
      Some(1),
      Some(7),
      None,
      Some(12)
    ])
  );
}
