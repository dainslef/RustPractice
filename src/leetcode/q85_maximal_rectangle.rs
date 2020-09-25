/**
 * 85. Maximal Rectangle
 * https://leetcode.com/problems/maximal-rectangle/
 *
 * Given a 2D binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.
 *
 * Example:
 *
 * Input:
 * [
 *   ["1","0","1","0","0"],
 *   ["1","0","1","1","1"],
 *   ["1","1","1","1","1"],
 *   ["1","0","0","1","0"]
 * ]
 * Output: 6
 */

/**
 * Runtime: 16 ms, faster than 28.57% of Rust online submissions for Maximal Rectangle.
 * Memory Usage: 4.8 MB, less than 28.57% of Rust online submissions for Maximal Rectangle.
 *
 * This problem can be treated as a "N row" version of "#84 Largest Rectangle in Histogram".
 * Caculate the heights of each row, then find the largest rectangle.
 */
fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
  // check if the matrix is empty
  if matrix.is_empty() || matrix[0].is_empty() {
    return 0;
  }

  let (row, column) = (matrix.len(), matrix[0].len());
  let mut heights = vec![0; column]; // the heights of current row
  let mut max_area = 0;

  for y in 0..row {
    for x in 0..column {
      // compute the heights in current row
      heights[x] = if matrix[y][x] == '1' {
        heights[x] + 1
      } else {
        0
      };
    }
    let (mut i, mut index_records) = (0, vec![]);
    loop {
      // compute the max area of the heights in current row
      if i < heights.len()
        && index_records
          .last()
          .map(|last| heights[i] > heights[*last])
          .unwrap_or(index_records.is_empty())
      {
        index_records.push(i);
        i += 1;
      } else if let Some(last) = index_records.pop() {
        let start = index_records.last().map(|v| v + 1).unwrap_or(0);
        max_area = max_area.max((i - start) * heights[last]);
      } else {
        break;
      }
    }
  }

  max_area as i32
}

#[test]
fn q85_test() {
  assert_eq!(
    maximal_rectangle(vec![
      vec!['1', '0', '1', '0', '0'],
      vec!['1', '0', '1', '1', '1'],
      vec!['1', '1', '1', '1', '1'],
      vec!['1', '0', '0', '1', '0']
    ]),
    6
  );
  assert_eq!(maximal_rectangle(vec![]), 0);
  assert_eq!(maximal_rectangle(vec![vec!['0']]), 0);
  assert_eq!(maximal_rectangle(vec![vec!['1']]), 1);
  assert_eq!(maximal_rectangle(vec![vec!['0', '0']]), 0);
  assert_eq!(maximal_rectangle(vec![vec!['1', '1']]), 2);
}
