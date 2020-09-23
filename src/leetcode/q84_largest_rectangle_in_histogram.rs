/**
 * 84. Largest Rectangle in Histogram
 * https://leetcode.com/problems/largest-rectangle-in-histogram/
 *
 * Given n non-negative integers representing the histogram's bar height where the width of each bar is 1, find the area of largest rectangle in the histogram.
 *
 *
 *
 *
 * Above is a histogram where width of each bar is 1, given height = [2,1,5,6,2,3].
 *
 *
 *
 *
 * The largest rectangle is shown in the shaded area, which has area = 10 unit.
 *
 *
 *
 * Example:
 *
 * Input: [2,1,5,6,2,3]
 * Output: 10
 */

// TLE when the input arguement "heights" has a large size (when greator than 10k)
fn largest_rectangle_area_tle(heights: Vec<i32>) -> i32 {
  use std::collections::BTreeMap;

  let mut min_height = i32::MAX;
  let size = heights.len();

  // use Map to save the indexes of each height: <key, (index, current_size, all_size)>
  let mut height_to_index: BTreeMap<i32, (usize, usize, usize)> = Default::default();

  for i in 0..size {
    let mut back_up = None;
    let height = heights[i];

    if height < min_height {
      min_height = height;
    }

    // count the size info by key
    for key in height_to_index.keys().map(|v| *v).collect::<Vec<i32>>() {
      if i > 0 && height >= key {
        let mut state = height_to_index.entry(key).or_default();
        if state.0 == i - 1 {
          state.1 += 1; // update the current size
        } else {
          state.1 = 1;
        }
        state.0 = i;
        state.2 = state.1.max(state.2); // compare and record the max size
      }
      if back_up.is_none() && height < key && height_to_index[&key].0 == i - 1 {
        back_up = Some(height_to_index.entry(key).or_default().clone());
      }
    }

    let state = height_to_index.entry(height).or_default();
    if state.2 == 0 {
      *state = if let Some((_, current_size, all_size)) = back_up {
        (i, current_size + 1, all_size.max(current_size + 1))
      } else {
        (i, 1, 1)
      }
    }
  }

  let mut max_area = min_height * size as i32;
  for v in heights {
    let area = v * height_to_index[&v].2 as i32;
    if area > max_area {
      max_area = area;
    }
  }

  max_area
}

/**
 * Runtime: 0 ms
 * Memory Usage: 2.6 MB
 */
fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
  let (mut i, mut max_area, mut index_records) = (0, 0, vec![]);
  loop {
    if i < heights.len()
      && index_records
        .last()
        .map(|last| heights[i] > heights[*last])
        .unwrap_or(index_records.is_empty())
    {
      index_records.push(i); // only add the greater index to the top of the stack
      i += 1; // update the current index
    } else if let Some(last) = index_records.pop() {
      // when the current value is less than the top of the stack, pop the top out
      let start = index_records.last().map(|v| v + 1).unwrap_or(0); // caculate the start index
      max_area = max_area.max((i - start) * heights[last] as usize); // compare the max area
    } else {
      break max_area as i32;
    }
  }
}

#[test]
fn q84_test() {
  assert_eq!(largest_rectangle_area(vec![1, 0, 1, 0, 0]), 1);
  assert_eq!(largest_rectangle_area(vec![10, 9, 9, 6, 9, 5, 4, 1, 2]), 30);
  assert_eq!(largest_rectangle_area(vec![3, 6, 5, 7, 4, 8, 1, 0]), 20);
  assert_eq!(largest_rectangle_area(vec![5, 4, 1, 2]), 8);
  assert_eq!(largest_rectangle_area(vec![1]), 1);
  assert_eq!(largest_rectangle_area(vec![1, 1]), 2);
  assert_eq!(largest_rectangle_area(vec![1, 1, 2]), 3);
  assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
  assert_eq!(
    largest_rectangle_area(vec![2, 1, 5, 6, 2, 3, 3, 1, 11, 1, 6, 3, 4]),
    13
  );
  assert_eq!(
    largest_rectangle_area(vec![3, 2, 8, 9, 1, 0, 0, 8, 6, 4, 8, 0, 7, 9, 5]),
    16
  );
}
