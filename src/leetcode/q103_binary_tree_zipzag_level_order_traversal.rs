/*!
[103. Binary Tree Zigzag Level Order Traversal](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/)

Given the `root` of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).

Example 1:

[Example 1](https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg)

```html
Input: root = [3,9,20,null,null,15,7]
Output: [[3],[20,9],[15,7]]
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
- `-100 <= Node.val <= 100`
*/

use super::*;

/**
Runtime: 2 ms, faster than 25.00% of Rust online submissions for Binary Tree Zigzag Level Order Traversal.
Memory Usage: 2.2 MB, less than 33.33% of Rust online submissions for Binary Tree Zigzag Level Order Traversal.
*/
fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
  let (mut count, mut results) = (0, vec![]);
  let mut temp = root.map(|v| vec![v]).unwrap_or_default();

  while !temp.is_empty() {
    use std::collections::VecDeque;
    let mut result = VecDeque::new();
    let mut next = vec![];
    for v in temp {
      if count % 2 == 0 {
        result.push_back(v.borrow().val);
      } else {
        result.push_front(v.borrow().val);
      } // check the parity
      if let Some(l) = &v.borrow().left {
        next.push(l.clone());
      }
      if let Some(r) = &v.borrow().right {
        next.push(r.clone());
      }
    }
    results.push(result.into());
    temp = next;
    count += 1;
  }

  results
}

#[test]
fn q103_test() {
  assert_eq!(
    zigzag_level_order(build_tree_node![3, 9, 20, null, null, 15, 7]),
    vec![vec![3], vec![20, 9], vec![15, 7]]
  );
  assert_eq!(
    zigzag_level_order(build_tree_node![1, 2, 3, 4, null, null, 5]),
    vec![vec![1], vec![3, 2], vec![4, 5]]
  );
  assert_eq!(
    zigzag_level_order(build_tree_node![1, 2, 3, 4, 5, 6]),
    vec![vec![1], vec![3, 2], vec![4, 5, 6]]
  );
}
