/**
 * 78. Subsets
 * https://leetcode.com/problems/subsets/
 *
 * Given a set of distinct integers, nums, return all possible subsets (the power set).
 *
 * Note: The solution set must not contain duplicate subsets.
 *
 * Example:
 *
 * Input: nums = [1,2,3]
 * Output:
 * [
 *   [3],
 *   [1],
 *   [2],
 *   [1,2,3],
 *   [1,3],
 *   [2,3],
 *   [1,2],
 *   []
 * ]
 */

/**
 * Runtime: 0 ms
 * Memory Usage: 2 MB
 */
fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
  nums.sort();

  let (mut temp, length) = (vec![vec![]], nums.len());
  // save the max index with target value
  let mut current: Vec<(Vec<i32>, usize)> = nums.iter().map(|v| vec![*v]).zip(0..length).collect();

  for _ in 1..length {
    let mut current_next = vec![];
    for (v, max_index) in &current {
      // check the saved max index, only loop and add the element wihch has larger index
      for i in (*max_index + 1)..length {
        let mut v = v.clone();
        v.push(nums[i]); // add the element by larger index
        current_next.push((v, i)); // save the new target with the new max index
      }
    }
    temp.append(&mut current.into_iter().map(|(v, _)| v).collect());
    current = current_next;
  }

  // add the last elements
  temp.append(&mut current.into_iter().map(|(v, _)| v).collect());

  temp
}

#[test]
fn q78_test() {
  use super::check_element_eq;
  check_element_eq(subsets(vec![]), vec![vec![]]);
  check_element_eq(subsets(vec![1]), vec![vec![1], vec![]]);
  check_element_eq(
    subsets(vec![1, 2, 3]),
    vec![
      vec![3],
      vec![1],
      vec![2],
      vec![1, 2, 3],
      vec![1, 3],
      vec![2, 3],
      vec![1, 2],
      vec![],
    ],
  );
  check_element_eq(
    subsets(vec![-1, 3, 777, 0]),
    vec![
      vec![],
      vec![-1],
      vec![3],
      vec![-1, 3],
      vec![777],
      vec![-1, 777],
      vec![3, 777],
      vec![-1, 3, 777],
      vec![0],
      vec![-1, 0],
      vec![0, 3],
      vec![-1, 0, 3],
      vec![0, 777],
      vec![-1, 0, 777],
      vec![0, 3, 777],
      vec![-1, 0, 3, 777],
    ],
  );
}
