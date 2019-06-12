/**
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 *
 * If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
 *
 * The replacement must be in-place and use only constant extra memory.
 *
 * Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
 *
 * 1,2,3 → 1,3,2
 * 3,2,1 → 1,2,3
 * 1,1,5 → 1,5,1
 */

fn next_permutation(nums: &mut Vec<i32>) {
  let (mut temp, mut is_max) = (vec![], true);

  for i in (1..nums.len()).rev() {
    let (next, current) = (nums[i - 1], nums[i]);
    temp.push(current);

    // find the fist value which less than "current"
    if next < current {
      while nums.len() > i {
        // remove the values after the index of "current"
        nums.pop();
      }

      // find the first num bigger than "next"(nums[i - 1]) in "temp", exchange their value
      for t in 0..temp.len() {
        if temp[t] > next {
          nums[i - 1] = temp[t];
          temp[t] = next;
          break;
        }
      }

      // sort the nums, make the nums numerically closest to the origin value
      temp.sort();
      nums.append(&mut temp);

      is_max = false;
      break;
    }
  }

  if is_max {
    nums.reverse();
  }
}

#[test]
fn test_next_permutation() {
  let nums = &mut vec![1, 2, 3];
  next_permutation(nums);
  assert_eq!(nums, &vec![1, 3, 2]);

  let nums = &mut vec![3, 2, 1];
  next_permutation(nums);
  assert_eq!(nums, &vec![1, 2, 3]);

  let nums = &mut vec![5, 1, 4, 3, 2];
  next_permutation(nums);
  assert_eq!(nums, &vec![5, 2, 1, 3, 4]);

  let nums = &mut vec![8, 0, 4, 2, 7, 6];
  next_permutation(nums);
  assert_eq!(nums, &vec![8, 0, 4, 6, 2, 7]);

  let nums = &mut vec![1, 2, 9, 5, 8, 7, 3, 2];
  next_permutation(nums);
  assert_eq!(nums, &vec![1, 2, 9, 7, 2, 3, 5, 8]);

  let nums = &mut vec![9, 4, 8, 9, 7, 5];
  next_permutation(nums);
  assert_eq!(nums, &vec![9, 4, 9, 5, 7, 8]);
}
