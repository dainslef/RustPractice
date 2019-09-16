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

// need refactoring and add comments
fn trap(height: Vec<i32>) -> i32 {
  use std::collections::HashMap;

  if height.len() == 0 {
    return 0;
  }

  let mut trap = 0;
  let (mut last, mut last_top, mut last_top_index, mut last_is_up) =
    (height[0], height[0], 0, true);

  let mut temp: HashMap<usize, i32> = HashMap::new();

  for i in 1..height.len() {
    let current = height[i];
    let is_up = current > last;
    if !is_up && last_is_up {
      let min = std::cmp::min(last_top, last);
      for i in last_top_index..i {
        if let Some(v) = temp.get(&i) {
          if &min > v {
            temp.insert(i, min);
          }
        } else {
          temp.insert(i, min);
        }
      }
      if last > last_top {
        last_top = last;
        last_top_index = i;
      }
    };
    if i == height.len() - 1 && is_up {
      let min = std::cmp::min(last_top, current);
      for i in last_top_index..=i {
        if let Some(v) = temp.get(&i) {
          if &min > v {
            temp.insert(i, min);
          }
        } else {
          temp.insert(i, min);
        }
      }
    };
    last_is_up = is_up;
    last = current;
  }

  for i in 0..height.len() {
    if let Some(min) = temp.get(&i) {
      let offset = min - height[i];
      if offset > 0 {
        trap += offset;
      }
    }
  }

  trap
}

#[test]
fn test_trap() {
  assert_eq!(trap(vec![2, 0, 2]), 2);
  assert_eq!(trap(vec![2, 0, 3]), 2);
  assert_eq!(trap(vec![8, 0, 7, 0, 10]), 17);
  assert_eq!(trap(vec![5, 2, 1, 2, 1, 5]), 14);
  assert_eq!(trap(vec![5, 8, 9, 4, 9, 6, 1, 4]), 8);
  assert_eq!(trap(vec![0, 1, 8, 2, 1, 0, 1, 3, 7, 1, 2, 1]), 29);
  assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
  assert_eq!(trap(vec![8, 7, 6, 5, 4, 3, 2, 1, 9, 2, 3, 4, 10, 6]), 46);
}
