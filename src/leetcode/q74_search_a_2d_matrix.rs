/**
 * 74. Search a 2D Matrix
 * https://leetcode.com/problems/search-a-2d-matrix/
 *
 * Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:
 *
 * Integers in each row are sorted from left to right.
 * The first integer of each row is greater than the last integer of the previous row.
 * Example 1:
 *
 * Input:
 * matrix = [
 *   [1,   3,  5,  7],
 *   [10, 11, 16, 20],
 *   [23, 30, 34, 50]
 * ]
 * target = 3
 * Output: true
 * Example 2:
 *
 * Input:
 * matrix = [
 *   [1,   3,  5,  7],
 *   [10, 11, 16, 20],
 *   [23, 30, 34, 50]
 * ]
 * target = 13
 * Output: false
 */

fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
  if matrix.is_empty() || matrix[0].is_empty() {
    return false;
  }

  let column = matrix[0].len() - 1;

  for row in 0..matrix.len() {
    if target < matrix[row][0] {
      return false;
    }
    if target >= matrix[row][0] && target <= matrix[row][column] {
      return matrix[row].contains(&target);
    }
  }

  false
}

#[test]
fn q74_test() {
  assert_eq!(search_matrix(vec![vec![1], vec![3]], 2), false);
  assert_eq!(search_matrix(vec![vec![]], 3), false);
  assert_eq!(search_matrix(vec![vec![1]], 1), true);
  assert_eq!(search_matrix(vec![vec![1]], 2), false);
  assert_eq!(
    search_matrix(
      vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
      23
    ),
    true
  );
  assert_eq!(
    search_matrix(
      vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
      3
    ),
    true
  );
  assert_eq!(
    search_matrix(
      vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
      13
    ),
    false
  );
}
