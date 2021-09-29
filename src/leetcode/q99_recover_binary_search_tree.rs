/*!
[99. Recover Binary Search Tree](https://leetcode.com/problems/recover-binary-search-tree/)

You are given the root of a binary search tree (BST), where **exactly** two nodes of the tree were swapped by mistake. Recover the tree without changing its structure.

Follow up: A solution using O(n) space is pretty straight forward. Could you devise a constant space solution?

Example 1:

![Example 1](https://assets.leetcode.com/uploads/2020/10/28/recover1.jpg)

```html
Input: root = [1,3,null,null,2]
Output: [3,1,null,null,2]
Explanation: 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.
```

Example 2:

![Example 2](https://assets.leetcode.com/uploads/2020/10/28/recover2.jpg)

```html
Input: root = [3,1,4,null,null,2]
Output: [2,1,4,null,null,3]
Explanation: 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.
```

Constraints:

- The number of nodes in the tree is in the range `[2, 1000]`.
- `-2^31` <= Node.val <= `2^31 - 1`
*/

use super::*;

/**
- Runtime: 4 ms, faster than 71.43% of Rust online submissions for Recover Binary Search Tree.
- Memory Usage: 2.3 MB, less than 42.86% of Rust online submissions for Recover Binary Search Tree.
*/
fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
  use std::ptr::null_mut;
  static mut TEMP_1: *mut TreeNode = null_mut();
  static mut TEMP_2: *mut TreeNode = null_mut();
  static mut LATEST: *mut TreeNode = null_mut();

  // use INORDER(Left-Root-Right) to traverse the whole tree
  unsafe fn recurse(root: *mut TreeNode) {
    if let Some(l) = &(*root).left {
      recurse(l.as_ptr());
    }

    // in Binary Search Tree, use INORDER to traverse the tree,
    // current value should be greater than previous value
    if !LATEST.is_null() {
      // because of the exactly two nodes mistake, just record the first and the last error node
      if TEMP_1.is_null() && (*root).val < (*LATEST).val {
        TEMP_1 = LATEST;
      }
      if (*root).val < (*LATEST).val {
        TEMP_2 = root;
      }
    }
    LATEST = root;

    if let Some(r) = &(*root).right {
      recurse(r.as_ptr());
    }
  }

  unsafe {
    if let Some(v) = root {
      // state variable has only one instance, need be init in each function call
      TEMP_1 = null_mut();
      TEMP_2 = null_mut();
      LATEST = null_mut();
      recurse(v.as_ptr());
    }
    if !TEMP_1.is_null() && !TEMP_2.is_null() {
      let back = (*TEMP_1).val;
      (*TEMP_1).val = (*TEMP_2).val;
      (*TEMP_2).val = back;
    }
  }
}

#[test]
fn q99_test() {
  fn check(origin: Vec<Option<i32>>, output: Vec<Option<i32>>) {
    let mut root = TreeNode::from(origin);
    recover_tree(&mut root);
    assert_eq!(root, TreeNode::from(output));
  }

  check(
    vec![Some(1), Some(3), None, None, Some(2)],
    vec![Some(3), Some(1), None, None, Some(2)],
  );
  check(
    vec![Some(2), Some(3), Some(1)],
    vec![Some(2), Some(1), Some(3)],
  );
  check(
    vec![Some(3), Some(1), Some(4), None, None, Some(2)],
    vec![Some(2), Some(1), Some(4), None, None, Some(3)],
  );
}
