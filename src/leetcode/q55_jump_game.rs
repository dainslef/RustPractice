/**
 * 55. Jump Game
 * https://leetcode.com/problems/jump-game/
 *
 * Given an array of non-negative integers, you are initially positioned at the first index of the array.
 *
 * Each element in the array represents your maximum jump length at that position.
 *
 * Determine if you are able to reach the last index.
 *
 * Example 1:
 *
 * Input: [2,3,1,1,4]
 * Output: true
 * Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
 * Example 2:
 *
 * Input: [3,2,1,0,4]
 * Output: false
 * Explanation: You will always arrive at index 3 no matter what. Its maximum
 *              jump length is 0, which makes it impossible to reach the last index.
 */

fn can_jump(nums: Vec<i32>) -> bool {
  let (mut last_reach, mut can_jump) = (0, true);

  for i in 0..nums.len() {
    let v = nums[i];
    if v == 0 && last_reach <= i && i != nums.len() - 1 {
      can_jump = false;
      break;
    }
    last_reach = last_reach.max(v as usize + i);
  }

  can_jump
}

#[test]
fn test_q55() {
  assert_eq!(can_jump(vec![0]), true);
  assert_eq!(can_jump(vec![0, 0, 0, 0, 0]), false);
  assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
  assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
}
