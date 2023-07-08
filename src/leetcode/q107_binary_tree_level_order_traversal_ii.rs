/*!
[107. Binary Tree Level Order Traversal II](https://leetcode.com/problems/binary-tree-level-order-traversal-ii/)

Given the `root` of a binary tree, return the bottom-up level order traversal of its nodes' values. (i.e., from left to right, level by level from leaf to root).

Example 1:

![Example 1](https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg)

```html
Input: root = [3,9,20,null,null,15,7]
Output: [[15,7],[9,20],[3]]
```

Example 2:

```html
Input: root = [1]
Output: [[1]]
```

Example 3:

```html
Input: root = []
Output: []
```

Constraints:

- The number of nodes in the tree is in the range `[0, 2000]`.
- `-1000 <= Node.val <= 1000`
*/

use super::*;

/**
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Binary Tree Level Order Traversal II.
Memory Usage: 2.3 MB, less than 18.18% of Rust online submissions for Binary Tree Level Order Traversal II.
*/
fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
  use std::collections::VecDeque;

  let mut results = VecDeque::new();
  let mut temp = root.map(|v| vec![v]).unwrap_or_default();

  while !temp.is_empty() {
    let mut result = vec![];
    let mut next = vec![];
    for v in temp {
      result.push(v.borrow().val);
      if let Some(l) = &v.borrow().left {
        next.push(l.clone());
      }
      if let Some(r) = &v.borrow().right {
        next.push(r.clone());
      }
    }
    results.push_front(result);
    temp = next;
  }

  results.into()
}

#[test]
fn q107_test() {
  assert_eq!(
    level_order_bottom(build_tree_node![]),
    vec![] as Vec<Vec<i32>>
  );
  assert_eq!(level_order_bottom(build_tree_node![1]), vec![vec![1]]);
  assert_eq!(
    level_order_bottom(build_tree_node![1, 2]),
    vec![vec![2], vec![1]]
  );
  assert_eq!(
    level_order_bottom(build_tree_node![1, null, 2]),
    vec![vec![2], vec![1]]
  );
  assert_eq!(
    level_order_bottom(build_tree_node![3, 9, 20, null, null, 15, 7]),
    vec![vec![15, 7], vec![9, 20], vec![3]]
  );
}
