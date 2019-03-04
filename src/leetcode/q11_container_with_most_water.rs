/**
 * Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai). n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0). Find two lines, which together with x-axis forms a container, such that the container contains the most water.
 *
 * Note: You may not slant the container and n is at least 2.
 *
 * The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
 *
 * Example:
 *
 * Input: [1,8,6,2,5,4,8,3,7]
 * Output: 49
 */

fn max_area(height: Vec<i32>) -> i32 {
  let len = height.len();
  let mut area = 0;

  // test all combinations to find the largest area
  for width in 1..len {
    for i in 0..len - width {
      let result = std::cmp::min(height[i], height[i + width]) * width as i32;
      if area < result {
        area = result;
      }
    }
  }

  area
}

fn max_area_two_side(height: Vec<i32>) -> i32 {
  let len = height.len();
  let (mut area, mut left, mut right) = (0, 0, len - 1);

  while right > left {
    let width = right - left;
    let (left_height, right_height) = (height[left], height[right]);
    // compare the height of left and right line, increase the index on the smaller side(the area is determined by the height of the smaller side)
    let new_area = if left_height > right_height {
      right -= 1;
      right_height
    } else {
      left += 1;
      left_height
    } * width as i32;
    // update the area when the new area is larger than old
    if new_area > area {
      area = new_area;
    }
  }

  area
}

#[test]
fn test_max_area() {
  assert_eq!(max_area(vec![1, 0]), 0);
  assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}
