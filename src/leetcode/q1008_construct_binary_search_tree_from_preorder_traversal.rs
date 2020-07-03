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

fn bst_from_preorder_loop(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
  let mut root = None;
  let mut nodes: Vec<Rc<RefCell<TreeNode>>> = vec![];

  for v in preorder {
    let mut match_index = nodes.len();
    let tree_node = Rc::new(RefCell::new(TreeNode::new(v)));

    if root.is_none() {
      // set the root node
      root = Some(tree_node.clone());
    }

    for i in 0..nodes.len() {
      if nodes[i].borrow().val > v {
        // find the first index of value which larger than target value in the node list
        match_index = i;
        break;
      }
    }

    if !nodes.is_empty() {
      if match_index < nodes.len() // check the index range and if the left child node is empty
        && (match_index == 0 || nodes[match_index].borrow().left.is_none())
      {
        nodes[match_index].borrow_mut().left = Some(tree_node.clone());
      } else if match_index > 0 // if the left child of the match value isn't empty, then set the target to the right child node of the last node
        && (match_index == nodes.len() || nodes[match_index - 1].borrow().right.is_none())
      {
        nodes[match_index - 1].borrow_mut().right = Some(tree_node.clone());
      }
    }

    nodes.insert(match_index, tree_node); // place the new node in correct place
  }

  root
}

fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
  fn recurse(preorder: &Vec<i32>, max: i32, i: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
    if *i == preorder.len() || preorder[*i] > max {
      // check if the index and value match the condition (less than max value)
      return None;
    }

    let mut n = TreeNode::new(preorder[*i]);
    *i += 1; // update the index in preorder, use reference can influence the outside
    n.left = recurse(preorder, n.val, i);
    n.right = recurse(preorder, max, i);

    Some(Rc::new(RefCell::new(n)))
  }

  recurse(&preorder, i32::MAX, &mut 0)
}

#[test]
fn q1008_test() {
  fn test(bst_from_preorder: impl Fn(Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>) {
    assert_eq!(
      bst_from_preorder(vec![8, 3, 1, 5, 4, 10, 9, 32]),
      TreeNode::from(vec![
        Some(8),
        Some(3),
        Some(10),
        Some(1),
        Some(5),
        Some(9),
        Some(32),
        None,
        None,
        Some(4)
      ])
    );
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

  test(bst_from_preorder);
  test(bst_from_preorder_loop);
}
