/**
 * 525. Contiguous Array
 * https://leetcode.com/problems/contiguous-array/
 *
 * Given a binary array, find the maximum length of a contiguous subarray with equal number of 0 and 1.
 *
 * Example 1:
 * Input: [0,1]
 * Output: 2
 * Explanation: [0, 1] is the longest contiguous subarray with equal number of 0 and 1.
 * Example 2:
 * Input: [0,1,0]
 * Output: 2
 * Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
 * Note: The length of the given binary array will not exceed 50,000.
 */

// solution explanation see https://leetcode.com/problems/contiguous-array/discuss/99655/Python-O(n)-Solution-with-Visual-Explanation
fn find_max_length(nums: Vec<i32>) -> i32 {
  use std::collections::HashMap;

  let (mut sum, mut max_length) = (0, 0);
  // use map to save all the indexes grouped by sum
  let mut indexes_count: HashMap<i32, Vec<usize>> = Default::default();

  // add the first index
  indexes_count.entry(sum).or_default().push(0);

  for i in 0..nums.len() {
    sum += if nums[i] == 0 { -1 } else { 1 }; // compute the sum (0 => -1, 1 => 1)
    let indexes = indexes_count.entry(sum).or_default();
    indexes.push(i + 1); // add the corrent position
    if let (Some(first), Some(last)) = (indexes.first(), indexes.last()) {
      // the max length is the distance from the first position to the last position
      max_length = (last - first).max(max_length);
    }
  }

  max_length as i32
}

#[test]
fn q525_test() {
  assert_eq!(find_max_length(vec![0, 1, 1, 0, 1, 1, 1, 0]), 4);
  assert_eq!(find_max_length(vec![1, 0]), 2);
  assert_eq!(find_max_length(vec![0, 1, 0]), 2);
  assert_eq!(find_max_length(vec![1, 0, 1]), 2);
  assert_eq!(find_max_length(vec![1, 0, 1, 1, 0]), 4);
}
