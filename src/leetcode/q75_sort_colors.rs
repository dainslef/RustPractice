/**
 * 75. Sort Colors
 * https://leetcode.com/problems/sort-colors/
 *
 * Given an array with n objects colored red, white or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white and blue.
 *
 * Here, we will use the integers 0, 1, and 2 to represent the color red, white, and blue respectively.
 *
 * Note: You are not suppose to use the library's sort function for this problem.
 *
 * Example:
 *
 * Input: [2,0,2,1,1,0]
 * Output: [0,0,1,1,2,2]
 * Follow up:
 *
 * A rather straight forward solution is a two-pass algorithm using counting sort.
 * First, iterate the array counting number of 0's, 1's, and 2's, then overwrite array with total number of 0's, then 1's and followed by 2's.
 * Could you come up with a one-pass algorithm using only constant space?
 */

fn sort_colors(nums: &mut Vec<i32>) {
  // two pass solution
  let (n0, n1) = nums.iter().fold((0, 0), |(n0, n1), v| match v {
    0 => (n0 + 1, n1),
    1 => (n0, n1 + 1),
    _ => (n0, n1),
  });
  for i in 0..nums.len() {
    match i {
      i if i < n0 => nums[i] = 0,
      i if i < n0 + n1 => nums[i] = 1,
      i => nums[i] = 2,
    }
  }
}

fn sort_colors_one_pass(nums: &mut Vec<i32>) {
  // one pass solution
  let length = nums.len();
  let (mut n0, mut n1, mut n2) = (0, 0, 0);
  for i in 0..length {
    let v = nums[i];
    if v <= 2 {
      nums[n2] = 2;
      n2 += 1;
    }
    if v <= 1 {
      nums[n1] = 1;
      n1 += 1;
    }
    if v == 0 {
      nums[n0] = 0;
      n0 += 1;
    }
  }
}

#[test]
fn q75_test() {
  fn test(sort_colors: impl Fn(&mut Vec<i32>)) {
    let mut nums = vec![0];
    sort_colors(&mut nums);
    assert_eq!(nums, [0]);

    let mut nums = vec![1];
    sort_colors(&mut nums);
    assert_eq!(nums, [1]);

    let mut nums = vec![2];
    sort_colors(&mut nums);
    assert_eq!(nums, [2]);

    let mut nums = vec![2, 0, 1];
    sort_colors(&mut nums);
    assert_eq!(nums, [0, 1, 2]);

    let mut nums = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut nums);
    assert_eq!(nums, [0, 0, 1, 1, 2, 2]);
  }

  test(sort_colors);
  test(sort_colors_one_pass);
}
