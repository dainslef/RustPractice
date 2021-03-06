/**
 * Given an unsorted integer array, find the smallest missing positive integer.
 *
 * Example 1:
 *
 * Input: [1,2,0]
 * Output: 3
 * Example 2:
 *
 * Input: [3,4,-1,1]
 * Output: 2
 * Example 3:
 *
 * Input: [7,8,9,11,12]
 * Output: 1
 * Note:
 *
 * Your algorithm should run in O(n) time and uses constant extra space.
 */

// "constant extra space" means the soultion shouldn't use container from std library,
// like Vec/Set/List and so on which dynamic allocate memory
fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
  // make the index as the number indicator (nums[i] = i + 1 or nums[i - 1] = i)
  // for example "nums[0] = 1, nums[1] = 2, ..."
  for i in 0..nums.len() {
    let mut v = nums[i];
    // check if the current number is valid (in range 1..=num.len()),
    // and match the rule (v = nums[v - 1])
    while v > 0 && v <= nums.len() as i32 && v != nums[v as usize - 1] {
      // if the current number is valid but doesn't match the rule, place the number on correct index
      let temp = nums[v as usize - 1];
      nums[v as usize - 1] = v;
      nums[i] = temp;
      v = temp;
    }
  }

  // traverse elements, find the first value doesn't match "num[i] = i + 1"
  for i in 0..nums.len() {
    let v = i as i32 + 1;
    if nums[i] != v {
      // if the element doesn't match the rule, it's the first missing positive integer
      return v;
    }
  }

  // if all the elements match the rule, the first missing positive integer is the next integer
  nums.len() as i32 + 1
}

#[test]
fn q41_test() {
  assert_eq!(first_missing_positive(vec![4, 3, 3, 2, 2, 1]), 5);
  assert_eq!(first_missing_positive(vec![1, 1]), 2);
  assert_eq!(first_missing_positive(vec![1, 2, 2, 3, 4]), 5);
  assert_eq!(first_missing_positive(vec![1, 2, 3, 4]), 5);
  assert_eq!(first_missing_positive(vec![-1, -1, -1, 3]), 1);
  assert_eq!(first_missing_positive(vec![-1, -2, -3, -1, 1, 10]), 2);
  assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
  assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
  assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 9, 12, 11]), 1);
  assert_eq!(first_missing_positive(vec![7, 8, 100, 9, 11, 1]), 2);
}
