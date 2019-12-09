/**
 * Given an array of non-negative integers, you are initially positioned at the first index of the array.
 *
 * Each element in the array represents your maximum jump length at that position.
 *
 * Your goal is to reach the last index in the minimum number of jumps.
 *
 * Example:
 *
 * Input: [2,3,1,1,4]
 * Output: 2
 * Explanation: The minimum number of jumps to reach the last index is 2.
 *     Jump 1 step from index 0 to 1, then 3 steps to the last index.
 * Note:
 *
 * You can assume that you can always reach the last index.
 */

fn jump(nums: Vec<i32>) -> i32 {
  let (mut last_reach, mut current_reach, mut step) = (0, 0, 0);

  for i in 0..nums.len() {
    if last_reach < i {
      last_reach = current_reach;
      step += 1;
    }

    current_reach = std::cmp::max(current_reach, nums[i] as usize + i);
  }

  step
}

#[test]
fn q45_test() {
  assert_eq!(
    jump(vec![
      5, 6, 4, 4, 6, 9, 4, 4, 7, 4, 4, 8, 2, 6, 8, 1, 5, 9, 6, 5, 2, 7, 9, 7, 9, 6, 9, 4, 1, 6, 8,
      8, 4, 4, 2, 0, 3, 8, 5
    ]),
    5
  );
  assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
  assert_eq!(jump(vec![1, 1, 1, 1, 1]), 4);
  assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
  assert_eq!(jump(vec![1, 2, 1, 1, 1]), 3);
  assert_eq!(jump(vec![2, 1]), 1);
  assert_eq!(jump(vec![2]), 0);
  assert_eq!(jump(vec![0]), 0);
}
