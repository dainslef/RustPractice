/**
 * 53. Maximum Subarray
 * https://leetcode.com/problems/maximum-subarray/
 *
 * Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
 *
 * Example:
 *
 * Input: [-2,1,-3,4,-1,2,1,-5,4],
 * Output: 6
 * Explanation: [4,-1,2,1] has the largest sum = 6.
 * Follow up:
 *
 * If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
 */

fn max_sub_array(nums: Vec<i32>) -> i32 {
  let (mut max, mut current) = (nums[0], 0);

  for v in nums {
    current += v;
    if current < v {
      current = v; // if the current sum is less than current value, use current value as new current sum
    }
    if current > max {
      max = current; // if the curent sum is greater than current value, update the max value
    }
  }

  max
}

#[test]
fn q53_test() {
  assert_eq!(max_sub_array(vec![-1]), -1);
  assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}
