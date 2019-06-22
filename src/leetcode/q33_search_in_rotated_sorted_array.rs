/**
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

fn search(nums: Vec<i32>, target: i32) -> i32 {
  if nums.is_empty() {
    return -1;
  }

  let (mut i_left, mut i_right) = (0, nums.len() - 1);
  loop {
    let i_center = (i_left + i_right) / 2;
    match (nums[i_left], nums[i_center], nums[i_right]) {
      (_, center, _) if center == target => break i_center as i32,
      (left, _, _) if left == target => break i_left as i32,
      (_, _, right) if right == target => break i_right as i32,
      _ if i_left + 1 == i_right || i_left == i_right => break -1,
      (left, center, right) => {
        if center > target {
          if target > left || center < right {
            i_right = i_center;
          } else {
            i_left = i_center;
          }
        } else {
          if center > left || target < right {
            i_left = i_center;
          } else {
            i_right = i_center;
          }
        }
      }
    }
  }
}

#[test]
fn test_search() {
  assert_eq!(search(vec![3, 6, 9, 10], 8), -1);
  assert_eq!(search(vec![1, 3], 2), -1);
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
