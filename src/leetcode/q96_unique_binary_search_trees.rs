/**
96. Unique Binary Search Trees
https://leetcode.com/problems/unique-binary-search-trees/

Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.



Example 1:


Input: n = 3
Output: 5
Example 2:

Input: n = 1
Output: 1


Constraints:

1 <= n <= 19
*/

/**
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Unique Binary Search Trees.
Memory Usage: 2 MB, less than 62.50% of Rust online submissions for Unique Binary Search Trees.

Use dynamic programming, split the big BST tree into small tree,
count the small tree and save data for reuse.
*/
fn num_trees(n: i32) -> i32 {
  use std::collections::HashMap;
  let mut bst_count: HashMap<i32, i32> = vec![(1, 1)].into_iter().collect();

  // "range size" means how many elements the sub tree can hold
  for range_size in 2..=n {
    let mut count = 0;
    for i in 1..=range_size {
      let left = bst_count.get(&(i - 1)).unwrap_or(&1);
      let right = bst_count.get(&(range_size - i)).unwrap_or(&1);
      count += left * right;
    }
    bst_count.entry(range_size).or_insert(count);
  }

  bst_count[&n]
}

#[test]
fn q96_test() {
  assert_eq!(num_trees(1), 1);
  assert_eq!(num_trees(2), 2);
  assert_eq!(num_trees(3), 5);
  assert_eq!(num_trees(4), 14);
  assert_eq!(num_trees(5), 42);
  assert_eq!(num_trees(6), 132);
  assert_eq!(num_trees(7), 429);
  assert_eq!(num_trees(19), 1767263190);
}
