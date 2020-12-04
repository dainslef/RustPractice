/**
 * 81. Search in Rotated Sorted Array II
 * https://leetcode.com/problems/search-in-rotated-sorted-array-ii/
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 *
 * (i.e., [0,0,1,2,2,5,6] might become [2,5,6,0,0,1,2]).
 *
 * You are given a target value to search. If found in the array return true, otherwise return false.
 *
 * Example 1:
 *
 * Input: nums = [2,5,6,0,0,1,2], target = 0
 * Output: true
 * Example 2:
 *
 * Input: nums = [2,5,6,0,0,1,2], target = 3
 * Output: false
 * Follow up:
 *
 * This is a follow up problem to Search in Rotated Sorted Array, where nums may contain duplicates.
 * Would this affect the run-time complexity? How and why?
 */

fn search(nums: Vec<i32>, target: i32) -> bool {
  fn recurse(nums: &Vec<i32>, i_left: usize, i_right: usize, target: i32) -> bool {
    let i_center = (i_left + i_right) / 2;
    let (left, center, right) = (nums[i_left], nums[i_center], nums[i_right]);
    target == left
      || target == center
      || target == right
      || i_left + 1 != i_right
        && i_left != i_right
        && if center == left || center == right {
          recurse(nums, i_left, i_center, target) || recurse(nums, i_center, i_right, target)
        } else if center > target {
          if target > left || center < right {
            recurse(nums, i_left, i_center, target)
          } else {
            recurse(nums, i_center, i_right, target)
          }
        } else {
          if center > left || target < right {
            recurse(nums, i_center, i_right, target)
          } else {
            recurse(nums, i_left, i_center, target)
          }
        }
  }

  nums.first().map(|v| target >= *v).unwrap_or(false)
    && nums.last().map(|v| target <= *v).unwrap_or(false)
    || recurse(&nums, 0, nums.len() - 1, target)
}

#[test]
fn q81_test() {
  assert_eq!(search(vec![0, 0, 1, 1, 2, 0], 2), true);
  assert_eq!(search(vec![1, 3, 1, 1, 1], 3), true);
  assert_eq!(search(vec![1, 1, 3, 1], 3), true);
  assert_eq!(search(vec![7, 8, 9, 9, 9, 1, 2], 10), false);
  assert_eq!(search(vec![7, 8, 9, 1, 1, 2], 5), false);
  assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
  assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 1), true);
  assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 2), true);
  assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
  assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 4), false);
  assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 5), true);
  assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 6), true);
  assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 7), false);
}
