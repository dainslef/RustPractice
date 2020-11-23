/**
 * 80. Remove Duplicates from Sorted Array II
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
 *
 * Given a sorted array nums, remove the duplicates in-place such that duplicates appeared at most twice and return the new length.
 *
 * Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.
 *
 * Example 1:
 *
 * Given nums = [1,1,1,2,2,3],
 *
 * Your function should return length = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
 *
 * It doesn't matter what you leave beyond the returned length.
 * Example 2:
 *
 * Given nums = [0,0,1,1,1,1,2,3,3],
 *
 * Your function should return length = 7, with the first seven elements of nums being modified to 0, 0, 1, 1, 2, 3 and 3 respectively.
 *
 * It doesn't matter what values are set beyond the returned length.
 * Clarification:
 *
 * Confused why the returned value is an integer but your answer is an array?
 *
 * Note that the input array is passed in by reference, which means modification to the input array will be known to the caller as well.
 *
 * Internally you can think of this:
 *
 * // nums is passed in by reference. (i.e., without making a copy)
 * int len = removeDuplicates(nums);
 *
 * // any modification to nums in your function would be known by the caller.
 * // using the length returned by your function, it prints the first len elements.
 * for (int i = 0; i < len; i++) {
 *     print(nums[i]);
 * }
 */

/**
 * Runtime: 4 ms, faster than 8.33% of Rust online submissions for Remove Duplicates from Sorted Array II.
 * Memory Usage: 2.1 MB, less than 33.33% of Rust online submissions for Remove Duplicates from Sorted Array II.
 */
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  let (mut last, mut offset) = (None, 0);

  for i in 0..nums.len() {
    let index = i - offset;
    let v = nums[index];
    match last {
      Some((last_v, true)) if last_v == v => {
        nums.remove(index);
        offset += 1;
      }
      Some((last_v, false)) if last_v == v => last = Some((last_v, true)),
      _ => last = Some((v, false)),
    }
  }

  nums.len() as i32
}

// need to submit and check
fn remove_duplicates_faster(nums: &mut Vec<i32>) -> i32 {
  let mut i = 0;

  while i < nums.len() {
    if i >= 2 && nums[i] == nums[i - 2] {
      nums.remove(i);
    } else {
      i += 1;
    }
  }

  nums.len() as i32
}

#[test]
fn q80_test() {
  macro_rules! test {
    ($item1: expr, $item2: expr, $size: expr) => {
      let mut nums = $item1;
      let size = remove_duplicates(&mut nums);
      assert_eq!(nums, $item2);
      assert_eq!(size, $size);
    };
  }

  test!(vec![1, 1, 1, 2, 2, 3], vec![1, 1, 2, 2, 3], 5);
  test!(vec![1, 2, 3], vec![1, 2, 3], 3);
  test!(
    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3],
    vec![1, 1, 2, 3],
    4
  );
}
