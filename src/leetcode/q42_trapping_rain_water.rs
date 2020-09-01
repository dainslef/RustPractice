/**
 * Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it is able to trap after raining.
 *
 *
 * The above elevation map is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped. Thanks Marcos for contributing this image!
 *
 * Example:
 *
 * Input: [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 */

fn trap(height: Vec<i32>) -> i32 {
  use std::collections::HashMap;

  if height.is_empty() {
    return 0;
  }

  let (mut last, mut last_top, mut last_top_index, mut last_is_up) =
    (height[0], height[0], 0, true);

  // record the filled height
  let mut temp: HashMap<usize, i32> = HashMap::new();

  for i in 1..height.len() {
    macro_rules! record_height {
      ($min:expr) => {
        for i in last_top_index..i {
          if temp.get(&i).map(|v| $min > *v).unwrap_or(true) {
            temp.insert(i, $min);
          }
        }
      };
    }

    let current = height[i];
    let is_up = current > last;

    // find the first index after height stop grow
    if last_is_up && !is_up {
      // save the filled height during the indexes
      record_height!(last_top.min(last));
      if last > last_top {
        last_top = last;
        last_top_index = i;
      }
    };

    // check if the index is the last
    if i == height.len() - 1 && is_up {
      // save the filled height before the last
      record_height!(last_top.min(current));
    };

    last_is_up = is_up;
    last = current;
  }

  // some index which at last might not have filled height are recorded
  (0..height.len())
    .filter(|i| temp.contains_key(&i)) // filter value doesn't have filled height
    .map(|i| temp[&i] - height[i]) // compute the offset
    .filter(|offset| *offset > 0) // filter the invalid offset
    .sum()
}

fn trap_two_side(height: Vec<i32>) -> i32 {
  let mut water = 0;
  if height.is_empty() {
    return water;
  }

  let length = height.len();
  let (mut left_max, mut right_max) = (vec![0; length], vec![0; length]);
  left_max[0] = height[0];
  right_max[length - 1] = height[length - 1];

  for l in 1..length - 1 {
    left_max[l] = left_max[l - 1].max(height[l]);
    let r = length - l - 1;
    right_max[r] = right_max[r + 1].max(height[r]);
  }

  for i in 1..length - 1 {
    let min = left_max[i].min(right_max[i]);
    if min > height[i] {
      water += min - height[i];
    }
  }

  water
}

#[test]
fn test_q42() {
  fn test(trap: impl Fn(Vec<i32>) -> i32) {
    assert_eq!(trap(vec![]), 0);
    assert_eq!(trap(vec![2, 0, 2]), 2);
    assert_eq!(trap(vec![2, 0, 3]), 2);
    assert_eq!(trap(vec![8, 0, 7, 0, 10]), 17);
    assert_eq!(trap(vec![5, 2, 1, 2, 1, 5]), 14);
    assert_eq!(trap(vec![5, 8, 9, 4, 9, 6, 1, 4]), 8);
    assert_eq!(trap(vec![0, 1, 8, 2, 1, 0, 1, 3, 7, 1, 2, 1]), 29);
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![8, 7, 6, 5, 4, 3, 2, 1, 9, 2, 3, 4, 10, 6]), 46);
  }

  test(trap);
  test(trap_two_side);
}
