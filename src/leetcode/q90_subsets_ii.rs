/*!
[90. Subsets II](https://leetcode.com/problems/subsets-ii/)

Given a collection of integers that might contain duplicates, nums, return all possible subsets (the power set).

Note: The solution set must not contain duplicate subsets.

Example:

Input: `[1,2,2]`
Output:
```txt
[
  [2],
  [1],
  [1,2,2],
  [2,2],
  [1,2],
  []
]
```
*/

/**
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Subsets II.<br>
Memory Usage: 2.2 MB, less than 55.56% of Rust online submissions for Subsets II.
*/
fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
  use std::collections::*;
  nums.sort(); // sub SET means the result is no-order-relative

  let (size, mut result, mut temp) = (nums.len(), HashSet::new(), HashMap::new());
  for (v, i) in nums.iter().zip(0..size) {
    let value = vec![*v];
    temp.entry(value.clone()).or_insert(i); // record the subset with the last index of the subset
    result.insert(value); // add the subset to result, to avoid the duplicate subsets
  }

  for _ in 0..size {
    let mut temp_next = HashMap::new();
    // get the saved subset and the last index
    for (v, current) in temp {
      // traverse the remaining subset elements
      for i in current + 1..size {
        let mut v = v.clone();
        v.push(nums[i]); // copy and add the new element to build a new subset
        temp_next.entry(v.clone()).or_insert(i); // record the new subset with the new index
        result.insert(v); // add the new subset to the result
      }
    }
    temp = temp_next;
  }

  result.insert(vec![]); // empty set should be a subset of all the set
  result.into_iter().collect()
}

/**
Runtime: 3 ms, faster than 33.33% of Rust online submissions for Subsets II.<br>
Memory Usage: 2.3 MB, less than 19.05% of Rust online submissions for Subsets II.
*/
fn subsets_with_dup_recursion(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
  nums.sort();

  fn recurse(nums: &Vec<i32>, last: Vec<(Vec<i32>, usize)>) -> Vec<Vec<i32>> {
    if last
      .first()
      .map(|(v, _)| v.len() == nums.len())
      .unwrap_or(false)
    {
      return vec![vec![]]; // empty set should be a subset of all the set, add to the final result
    }

    let (mut results, mut new_subsets, mut new_subsets_with_index) = (vec![], vec![], vec![]);
    for (v, start) in last {
      for i in start..nums.len() {
        let mut new_subset = v.clone();
        new_subset.push(nums[i]);
        let last = new_subsets_with_index.last();
        if last
          .map(|(last_subset, _)| last_subset != &new_subset)
          .unwrap_or(last.is_none())
        {
          new_subsets.push(new_subset.clone());
          new_subsets_with_index.push((new_subset, i + 1));
        }
      }
    }

    results.append(&mut new_subsets);
    results.append(&mut recurse(&nums, new_subsets_with_index));
    results
  }

  recurse(&nums, vec![(vec![], 0)])
}

#[test]
fn q90_test() {
  fn test(subsets_with_dup: impl Fn(Vec<i32>) -> Vec<Vec<i32>>) {
    use super::check_element_eq;
    assert!(check_element_eq(
      subsets_with_dup(vec![1, 2]),
      vec![vec![2], vec![1], vec![1, 2], vec![]]
    ));
    assert!(check_element_eq(subsets_with_dup(vec![]), vec![vec![]]));
    assert!(check_element_eq(
      subsets_with_dup(vec![1]),
      vec![vec![1], vec![]]
    ));
    assert!(check_element_eq(
      subsets_with_dup(vec![4, 4, 4, 1, 4]),
      vec![
        vec![],
        vec![1],
        vec![1, 4],
        vec![1, 4, 4],
        vec![1, 4, 4, 4],
        vec![1, 4, 4, 4, 4],
        vec![4],
        vec![4, 4],
        vec![4, 4, 4],
        vec![4, 4, 4, 4]
      ]
    ));
    assert!(check_element_eq(
      subsets_with_dup(vec![1, 2, 4, 6, 7, 7]),
      vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 4],
        vec![1, 2, 4, 6],
        vec![1, 2, 4, 6, 7],
        vec![1, 2, 4, 6, 7, 7],
        vec![1, 2, 4, 7],
        vec![1, 2, 4, 7, 7],
        vec![1, 2, 6],
        vec![1, 2, 6, 7],
        vec![1, 2, 6, 7, 7],
        vec![1, 2, 7],
        vec![1, 2, 7, 7],
        vec![1, 4],
        vec![1, 4, 6],
        vec![1, 4, 6, 7],
        vec![1, 4, 6, 7, 7],
        vec![1, 4, 7],
        vec![1, 4, 7, 7],
        vec![1, 6],
        vec![1, 6, 7],
        vec![1, 6, 7, 7],
        vec![1, 7],
        vec![1, 7, 7],
        vec![2],
        vec![2, 4],
        vec![2, 4, 6],
        vec![2, 4, 6, 7],
        vec![2, 4, 6, 7, 7],
        vec![2, 4, 7],
        vec![2, 4, 7, 7],
        vec![2, 6],
        vec![2, 6, 7],
        vec![2, 6, 7, 7],
        vec![2, 7],
        vec![2, 7, 7],
        vec![4],
        vec![4, 6],
        vec![4, 6, 7],
        vec![4, 6, 7, 7],
        vec![4, 7],
        vec![4, 7, 7],
        vec![6],
        vec![6, 7],
        vec![6, 7, 7],
        vec![7],
        vec![7, 7]
      ]
    ));
  }

  test(subsets_with_dup);
  test(subsets_with_dup_recursion);
}
