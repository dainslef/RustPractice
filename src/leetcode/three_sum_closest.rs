/**
 * Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.
 *
 * Example:
 *
 * Given array nums = [-1, 2, 1, -4], and target = 1.
 *
 * The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 */

fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
  let mut nums = nums;
  nums.sort();

  let (mut close_target, mut offset, length) = (0, std::i32::MAX, nums.len());

  for i in 0..length - 2 {
    let a = nums[i];
    let (mut start, mut end) = (i + 1, length - 1);

    while start < end {
      let (b, c) = (nums[start], nums[end]);
      let sum = a + b + c;
      let offset_now = (sum - target).abs();

      let mut update_offset = || {
        if offset_now < offset {
          offset = offset_now;
          close_target = sum;
        }
      };

      if sum > target {
        update_offset();
        end -= 1;
      } else if sum < target {
        update_offset();
        start += 1;
      } else {
        // return the answer right now, "break" can only jump out of one round of loop
        return sum;
      }
    }
  }

  close_target
}

#[test]
fn three_sum_closest_test() {
  assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
  assert_eq!(three_sum_closest(vec![4, -1, -4, 4], -1), -1);
  assert_eq!(three_sum_closest(vec![-1, 2, 1, -4, 5, 9, 20, -100], 1), 0);
  assert_eq!(
    three_sum_closest(vec![-1, 2, 1, -4, 5, 9, 20, -100], 10),
    10
  );
}
