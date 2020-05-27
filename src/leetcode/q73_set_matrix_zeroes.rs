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

  let mut zero_in_first_row = false;
  for y in 0..matrix.len() {
    let mut has_zero = false;
    for x in 0..matrix[y].len() {
      if matrix[y][x] == 0 {
        matrix[0][x] = 0; // use the first row to record which cloumn should set zeros
        if y == 0 {
          zero_in_first_row = true; // record if the first row should set zeros
        } else {
          has_zero = true;
        }
      }
      // if the current row is the last row, set zeros for record columns
      if y == matrix.len() - 1 && matrix[0][x] == 0 {
        for i in 0..matrix.len() {
          matrix[i][x] = 0; // set zeros for record column
        }
      }
    }
    if has_zero {
      for i in 0..matrix[y].len() {
        matrix[y][i] = 0; // set the zreos for the target row
      }
    }
  }

  if zero_in_first_row {
    for x in 0..matrix[0].len() {
      matrix[0][x] = 0;
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

  let mut matrix = vec![vec![0, 1, 2, 1], vec![3, 0, 2, 0], vec![1, 3, 1, 5]];
  set_zeroes(&mut matrix);
  assert_eq!(matrix, [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 1, 0]]);
}
