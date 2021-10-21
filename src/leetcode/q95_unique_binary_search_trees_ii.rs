/**
 * 95. Unique Binary Search Trees II
 * https://leetcode.com/problems/unique-binary-search-trees-ii/
 *
 * Given an integer n, return all the structurally unique BST's (binary search trees), which has exactly n nodes of unique values from 1 to n. Return the answer in any order.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: n = 3
 * Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
 * Example 2:
 *
 * Input: n = 1
 * Output: [[1]]
 *
 *
 * Constraints:
 *
 * 1 <= n <= 8
 */
use super::*;

/**
Runtime: 0 ms
Memory Usage: 2.6 MB

BTS Tree means root node value is bigger than left node and less than right node.

Use dynamic programming as solution.

definitions:
G(n): the number of unique BST tree in 1 to n sequence
F(i, n), 1 <= i <= n: the number of unique BST tree subsequence (value range from 1 to n) when root node value is i

G(n) = F(1, n) * F(2, n) * ... * F(n, n)

and F(i, n) can be splited to subsequence, follow rules:
F(i, n) = G(i - 1) * G(i + 1)

combine above formules, we can get:
G(n) = 1 * (G(1) * G(3)) * (G(2) * G(4)) * ... * G(n - 1)
*/
fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
  fn recursion(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut results = vec![];

    if start > end {
      results.push(None);
      return results;
    }

    for i in start..=end {
      // build the left tree (less than current node)
      let left_tree = recursion(start, i - 1);
      // build the right tree (bigger than current node)
      let right_tree = recursion(i + 1, end);

      for left in left_tree {
        for right in right_tree.clone() {
          // build the new node
          let root = Some(Rc::new(RefCell::new(TreeNode {
            val: i,
            left: left.clone(),
            right,
          })));
          results.push(root);
        }
      }
    }

    results
  }

  recursion(1, n)
}

#[test]
fn q95_test() {
  assert_eq!(generate_trees(1), vec![build_tree_node![1]]);
  assert_eq!(
    generate_trees(2),
    vec![build_tree_node![1, null, 2], build_tree_node![2, 1]]
  );
  assert_eq!(
    generate_trees(3),
    vec![
      build_tree_node![1, null, 2, null, 3],
      build_tree_node![1, null, 3, 2],
      build_tree_node![2, 1, 3],
      build_tree_node![3, 1, null, null, 2],
      build_tree_node![3, 2, null, 1]
    ]
  );
}
