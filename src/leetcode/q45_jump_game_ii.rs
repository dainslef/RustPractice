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

// TLE, use DFS cost a lot of time
fn jump_recursion(nums: Vec<i32>) -> i32 {
  fn recurse(
    nums: &Vec<i32>,
    index: usize,
    step: usize,
    min_step: Option<usize>,
  ) -> Option<usize> {
    use std::{cmp::Reverse, collections::BinaryHeap};

    let mut mins = BinaryHeap::new();
    if index + 1 == nums.len() {
      return Some(index);
    } else if min_step.map(|v| step < v).unwrap_or(true) {
      for i in 1..=nums[index] as usize {
        let index_next = index + i;
        if index_next >= nums.len() - 1 {
          mins.push(Reverse(step + 1));
        } else if let Some(v) = recurse(nums, index_next, step + 1, mins.peek().map(|v| v.0)) {
          mins.push(Reverse(v));
        }
      }
    }

    mins.peek().map(|v| v.0).or(min_step)
  }

  recurse(&nums, 0, 0, None).map(|v| v as i32).unwrap_or(0)
}

fn jump(nums: Vec<i32>) -> i32 {
  let (mut last_reach, mut current_reach, mut step) = (0, 0, 0);

  for i in 0..nums.len() {
    // check if the last reach index is less than current index
    if last_reach < i {
      // if the last max reach index is smaller than current index, it means should cost a step to jump index
      last_reach = current_reach;
      // update the step size
      step += 1;
    }

    // compare with the new reach index, update the current reach index if the new one is larger than current
    // always make current reach index to be the biggest reach index now
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
