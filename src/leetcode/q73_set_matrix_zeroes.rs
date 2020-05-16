/**
 * 73. Set Matrix Zeroes
 * https://leetcode.com/problems/set-matrix-zeroes/
 *
 * Given a m x n matrix, if an element is 0, set its entire row and column to 0. Do it in-place.
 *
 * Example 1:
 *
 * Input:
 * [
 *   [1,1,1],
 *   [1,0,1],
 *   [1,1,1]
 * ]
 * Output:
 * [
 *   [1,0,1],
 *   [0,0,0],
 *   [1,0,1]
 * ]
 * Example 2:
 *
 * Input:
 * [
 *   [0,1,2,0],
 *   [3,4,5,2],
 *   [1,3,1,5]
 * ]
 * Output:
 * [
 *   [0,0,0,0],
 *   [0,4,5,0],
 *   [0,3,1,0]
 * ]
 * Follow up:
 *
 * A straight forward solution using O(mn) space is probably a bad idea.
 * A simple improvement uses O(m + n) space, but still not the best solution.
 * Could you devise a constant space solution?
 */

fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
  if matrix.is_empty() || matrix[0].is_empty() {
    return;
  }

  let (mut zero_in_first_row, mut zero_in_first_column) = (false, false);
  for y in 0..matrix.len() {
    for x in 0..matrix[y].len() {
      if matrix[y][x] == 0 {
        if y == 0 {
          // record if the first row should set zeros
          zero_in_first_row = true;
        }
        if x == 0 {
          // record if the first column should set zeros
          zero_in_first_column = true;
        }
        // use the first row and first column to record which row/cloumn should set zeros
        matrix[y][0] = 0;
        matrix[0][x] = 0;
      }
    }
  }

  for y in 1..matrix.len() {
    if matrix[y][0] == 0 {
      // set zeros for record row
      for x in 0..matrix[y].len() {
        matrix[y][x] = 0;
      }
    }
  }

  for x in 1..matrix[0].len() {
    if matrix[0][x] == 0 {
      // set zeros for record column
      for y in 0..matrix.len() {
        matrix[y][x] = 0;
      }
    }
  }

  if zero_in_first_row {
    for x in 0..matrix[0].len() {
      matrix[0][x] = 0;
    }
  }

  if zero_in_first_column {
    for y in 0..matrix.len() {
      matrix[y][0] = 0;
    }
  }
}

#[test]
fn q73_test() {
  let mut matrix = vec![vec![1, 0, 3]];
  set_zeroes(&mut matrix);
  assert_eq!(matrix, [[0, 0, 0]]);

  let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
  set_zeroes(&mut matrix);
  assert_eq!(matrix, [[1, 0, 1], [0, 0, 0], [1, 0, 1]]);

  let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
  set_zeroes(&mut matrix);
  assert_eq!(matrix, [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
}
