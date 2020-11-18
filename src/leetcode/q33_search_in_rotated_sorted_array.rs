/**
 * 33. Search in Rotated Sorted Array
 * https://leetcode.com/problems/search-in-rotated-sorted-array/
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
 *
 * You are given a target value to search. If found in the array return its index, otherwise return -1.
 *
 * You may assume no duplicate exists in the array.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * Example 1:
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 * Example 2:
 *
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 */

// O(n) solution
fn search(nums: Vec<i32>, target: i32) -> i32 {
  for i in 0..nums.len() {
    if nums[i] == target {
      return i as i32;
    }
  }
  -1
}

// O(log n) solution, use binary search
fn search_binary(nums: Vec<i32>, target: i32) -> i32 {
  // check the range of nums, return if target isn't match the range
  if nums.first().map(|v| target < *v).unwrap_or(true)
    && nums.last().map(|v| target > *v).unwrap_or(true)
  {
    return -1;
  }

  let (mut i_left, mut i_right) = (0, nums.len() - 1);
  loop {
    let i_center = (i_left + i_right) / 2;
    match (nums[i_left], nums[i_center], nums[i_right]) {
      // compare the value of left/center/right index
      (_, center, _) if center == target => break i_center as i32,
      (left, _, _) if left == target => break i_left as i32,
      (_, _, right) if right == target => break i_right as i32,
      _ if i_left + 1 == i_right || i_left == i_right => break -1,
      (left, center, right) => {
        // if center is larger than target, only should make center index as new left index when center and target in different side
        let condition1 = center > target && center > right && target < right;
        // if center is smaller than target, make center index as new left index when when center and target in same side
        let condition2 = center < target && (center > left || target < right);
        if condition1 || condition2 {
          i_left = i_center;
        } else {
          i_right = i_center;
        }
      }
    }
  }
}

#[test]
fn q33_test() {
  fn test(search: impl Fn(Vec<i32>, i32) -> i32) {
    assert_eq!(search(vec![7, 8, 9, 1, 2], 5), -1);
    assert_eq!(search(vec![3, 6, 9, 10], 8), -1);
    assert_eq!(search(vec![1, 3], 2), -1);
    assert_eq!(search(vec![1, 2], 2), 1);
    assert_eq!(search(vec![1], 0), -1);
    assert_eq!(search(vec![], 5), -1);
    assert_eq!(search(vec![0, 1, 2, 4, 5, 6, 7], 6), 5);
    assert_eq!(search(vec![0, 1, 2, 4, 5, 6, 7], 4), 3);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 1), 5);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 2), 6);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 4), 0);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 5), 1);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 6), 2);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 7), 3);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 8), -1);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 9), -1);
  }

  test(search);
  test(search_binary);
}
