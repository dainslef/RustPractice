/**
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * If the target is not found in the array, return [-1, -1].
 *
 * Example 1:
 *
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * Example 2:
 *
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 */

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
  if nums.first().map(|v| target < *v).unwrap_or(true)
    && nums.last().map(|v| target > *v).unwrap_or(true)
  {
    return vec![-1, -1];
  }

  let (mut i_left, mut i_right) = (0, nums.len() - 1);
  loop {
    let i_center = (i_left + i_right) / 2;
    let center = nums[i_center];

    if let Some((index, value)) = match (nums[i_left], nums[i_center], nums[i_right]) {
      (_, center, _) if center == target => Some((i_center, center)),
      (left, _, _) if left == target => Some((i_left, left)),
      (_, _, right) if right == target => Some((i_right, right)),
      _ => None,
    } {
      let check_range = |range: &mut dyn Iterator<Item = usize>, v: &mut i32| {
        for i in range {
          if nums[i] == value {
            *v = i as i32;
          } else {
            break;
          }
        }
      };
      let mut i_target_min = index as i32;
      check_range(&mut (0..index).rev(), &mut i_target_min);
      let mut i_target_max = i_target_min;
      check_range(&mut (index..nums.len()), &mut i_target_max);
      break vec![i_target_min, i_target_max];
    }

    if i_left == i_right || i_left + 1 == i_right {
      break vec![-1, -1];
    } else if center > target {
      i_right = i_center;
    } else if center < target {
      i_left = i_center;
    }
  }
}

#[test]
fn q34_test() {
  assert_eq!(search_range(vec![1, 1, 2], 1), [0, 1]);
  assert_eq!(search_range(vec![1, 3], 2), [-1, -1]);
  assert_eq!(search_range(vec![1, 2], 2), [1, 1]);
  assert_eq!(search_range(vec![1], 0), [-1, -1]);
  assert_eq!(search_range(vec![], 5), [-1, -1]);
  assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 0), [-1, -1]);
  assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), [-1, -1]);
  assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 5), [0, 0]);
  assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 7), [1, 2]);
  assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), [3, 4]);
  assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 9), [-1, -1]);
  assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 10), [5, 5]);
  assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 11), [-1, -1]);
}
