/*!
[98. Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/)

Given the `root` of a binary tree, determine if it is a valid binary search tree (BST).

A valid BST is defined as follows:

The left subtree of a node contains only nodes with keys less than the node's key.
The right subtree of a node contains only nodes with keys greater than the node's key.
Both the left and right subtrees must also be binary search trees.

Example 1:

![Example 1](https://assets.leetcode.com/uploads/2020/12/01/tree1.jpg)

```html
Input: root = [2,1,3]
Output: true
```

Example 2:

![Example 2](https://assets.leetcode.com/uploads/2020/12/01/tree2.jpg)

```html
Input: root = [5,1,4,null,null,3,6]
Output: false
Explanation: The root node's value is 5 but its right child's value is 4.
```

Constraints:

- The number of nodes in the tree is in the range `[1, 10^4]`.
- `-2^31 <= Node.val <= 2^31 - 1`
*/

use super::*;

/**
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Validate Binary Search Tree.
Memory Usage: 3 MB, less than 78.05% of Rust online submissions for Validate Binary Search Tree.
*/
fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
  fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
    if let Some(v) = root {
      let v = v.borrow();
      let (left, right) = (&v.left, &v.right);
      let is_left_valid = left
        .as_ref()
        .map(|l| {
          let val = l.borrow().val;
          val < v.val && min.map(|min| val > min).unwrap_or(true)
        })
        .unwrap_or(true);
      let is_right_valid = right
        .as_ref()
        .map(|r| {
          let val = r.borrow().val;
          val > v.val && max.map(|max| val < max).unwrap_or(true)
        })
        .unwrap_or(true);
      is_left_valid
        && is_right_valid
        && recurse(left, min, Some(v.val))
        && recurse(right, Some(v.val), max)
    } else {
      true
    }
  }

  recurse(&root, None, None)
}

#[test]
fn q98_test() {
  assert!(is_valid_bst(TreeNode::from(vec![Some(1)])));
  assert!(is_valid_bst(TreeNode::from(vec![
    Some(2),
    Some(1),
    Some(3)
  ])));
  assert!(!is_valid_bst(TreeNode::from(vec![
    Some(5),
    Some(1),
    Some(4),
    None,
    None,
    Some(3),
    Some(6)
  ])));
  assert!(!is_valid_bst(TreeNode::from(vec![
    Some(10),
    Some(5),
    Some(11),
    Some(4),
    None,
    Some(8),
    Some(13),
    Some(2),
    None,
    None,
    None,
    Some(12)
  ])));
  assert!(is_valid_bst(TreeNode::from(vec![
    Some(7),
    Some(5),
    Some(11),
    Some(4),
    None,
    Some(8),
    Some(13),
    Some(2),
    None,
    None,
    None,
    Some(12)
  ])));
  assert!(!is_valid_bst(TreeNode::from(vec![
    Some(120),
    Some(70),
    Some(140),
    Some(50),
    Some(100),
    Some(130),
    Some(160),
    Some(20),
    Some(55),
    Some(75),
    Some(110),
    Some(119),
    Some(135),
    Some(150),
    Some(200)
  ])));
}
