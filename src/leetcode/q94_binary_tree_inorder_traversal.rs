/*!
[94. Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/)

Given the root of a binary tree, return the inorder traversal of its nodes' values.

Example 1:

```html
Input: root = [1,null,2,3]
Output: [1,3,2]
```

Example 2:

```html
Input: root = []
Output: []
```

Example 3:

```html
Input: root = [1]
Output: [1]
```

Example 4:

```html
Input: root = [1,2]
Output: [2,1]
```

Example 5:

```html
Input: root = [1,null,2]
Output: [1,2]
```

Constraints:

- The number of nodes in the tree is in the range [0, 100].
- -100 <= Node.val <= 100
*/
use super::*;

/**
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Binary Tree Inorder Traversal.<br>
Memory Usage: 1.9 MB, less than 93.55% of Rust online submissions for Binary Tree Inorder Traversal.

Solution based on recursion.

Depth First Traversals:

```html
Inorder (Left, Root, Right)
Preorder (Root, Left, Right)
Postorder (Left, Right, Root)
```
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

/**
Runtime: 1 ms, faster than 73.04% of Rust online submissions for Binary Tree Inorder Traversal.<br>
Memory Usage: 2.2 MB, less than 33.91% of Rust online submissions for Binary Tree Inorder Traversal.

Loop-based solution.
*/
fn inorder_traversal_loop(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let (mut temp, mut nodes) = (vec![], vec![root]);

  while let Some(Some(v)) = nodes.last_mut() {
    // it must use unafe to get the raw pointer here.
    // if you use "borrow_mut()" to get the safe mut pointer here,
    // it will against the onwership rules. ("v" use the mutable reference of "nodes" in line 74,
    // so "nodes" can't be used for other mutable operate, like "nodes.push(xxx)" in line 85)
    let left = unsafe { &mut (*v.as_ptr()).left };
    if left.is_some() {
      nodes.push(left.take()); // if the left node is exist, access all left node first
    } else if let Some(Some(n)) = nodes.pop() {
      let mut n = n.borrow_mut();
      temp.push(n.val); // if the left node is not exist, then push the value
      if n.right.is_some() {
        // if current node has right child, set the node as a new access point,
        // and continue to access the next left node
        nodes.push(n.right.take());
      }
    }
  }

  temp
}

#[test]
fn q94_test() {
  fn test(inorder_traversal: impl Fn(Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>) {
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

  test(inorder_traversal);
  test(inorder_traversal_loop);
}
