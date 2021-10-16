/*!
[102. Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)

Given the `root` of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

Example 1:

![Example 1](https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg)

```html
Input: root = [3,9,20,null,null,15,7]
Output: [[3],[9,20],[15,7]]
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
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Binary Tree Level Order Traversal.
Memory Usage: 2.1 MB, less than 60.98% of Rust online submissions for Binary Tree Level Order Traversal.
*/
fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
  let mut results = vec![];
  let mut temp: Vec<_> = root.map(|v| vec![v]).unwrap_or_default();

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
    results.push(result);
    temp = next;
  }

  results
}

#[test]
fn q102_test() {
  assert_eq!(
    level_order(TreeNode::from(vec![
      Some(-2),
      Some(-9),
      Some(0),
      Some(3),
      Some(5),
      Some(-1),
      Some(9),
      Some(5),
      Some(2),
      None,
      None,
      Some(-3),
      None,
      Some(-7),
      Some(6),
      Some(-6),
      None,
      None,
      None,
      Some(-1),
      None,
      None,
      None,
      Some(-9),
      Some(9),
      None,
      None,
      None,
      None,
      Some(8),
      None,
      Some(-2),
      Some(5)
    ])),
    vec![
      vec![-2],
      vec![-9, 0],
      vec![3, 5, -1, 9],
      vec![5, 2, -3, -7, 6],
      vec![-6, -1, -9, 9],
      vec![8, -2, 5]
    ]
  );
  assert_eq!(level_order(build_tree_node![]), vec![] as Vec<Vec<i32>>);
  assert_eq!(level_order(build_tree_node![1]), vec![vec![1]]);
  assert_eq!(level_order(build_tree_node![1, 2]), vec![vec![1], vec![2]]);
  assert_eq!(
    level_order(build_tree_node![1, null, 2]),
    vec![vec![1], vec![2]]
  );
  assert_eq!(
    level_order(build_tree_node![3, 9, 20, null, null, 15, 7]),
    vec![vec![3], vec![9, 20], vec![15, 7]]
  );
  assert_eq!(
    level_order(build_tree_node![3, 9, 20, 1, 5, 15, 7, 1, null, 3]),
    vec![vec![3], vec![9, 20], vec![1, 5, 15, 7], vec![1, 3]]
  );
}
