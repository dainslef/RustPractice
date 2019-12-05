/**
 * You are given an n x n 2D matrix representing an image.
 *
 * Rotate the image by 90 degrees (clockwise).
 *
 * Note:
 *
 * You have to rotate the image in-place, which means you have to modify the input 2D  * matrix directly. DO NOT allocate another 2D matrix and do the rotation.
 *
 * Example 1:
 *
 * Given input matrix =
 * [
 *   [1,2,3],
 *   [4,5,6],
 *   [7,8,9]
 * ],
 *
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [7,4,1],
 *   [8,5,2],
 *   [9,6,3]
 * ]
 * Example 2:
 *
 * Given input matrix =
 * [
 *   [ 5, 1, 9,11],
 *   [ 2, 4, 8,10],
 *   [13, 3, 6, 7],
 *   [15,14,12,16]
 * ],
 *
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [15,13, 2, 5],
 *   [14, 3, 4, 1],
 *   [12, 6, 8, 9],
 *   [16, 7,10,11]
 * ]
 */

fn rotate(matrix: &mut Vec<Vec<i32>>) {
  let max = matrix.len() - 1;

  // range: [0, matrix length - 2], the last index is changed by first index
  for y in 0..max {
    // don't change item you have changed before (based on index y)
    for x in y..max - y {
      let (r_x, r_y) = (max - x, max - y);
      // backup the old values
      let (a, b, c, d) = (
        matrix[y][x],
        matrix[x][r_y],
        matrix[r_y][r_x],
        matrix[r_x][y],
      );
      // change the items
      matrix[y][x] = d;
      matrix[x][r_y] = a;
      matrix[r_y][r_x] = b;
      matrix[r_x][y] = c;
    }
  }
}

#[test]
fn test_q48() {
  let mut matrix = vec![vec![]];
  rotate(&mut matrix);
  assert_eq!(matrix, vec![vec![]]);

  let mut matrix = vec![vec![1]];
  rotate(&mut matrix);
  assert_eq!(matrix, vec![vec![1]]);

  let mut matrix = vec![vec![1, 2], vec![3, 4]];
  rotate(&mut matrix);
  assert_eq!(matrix, vec![vec![3, 1], vec![4, 2]]);

  let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
  rotate(&mut matrix);
  assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

  let mut matrix = vec![
    vec![5, 1, 9, 11],
    vec![2, 4, 8, 10],
    vec![13, 3, 6, 7],
    vec![15, 14, 12, 16],
  ];
  rotate(&mut matrix);
  assert_eq!(
    matrix,
    vec![
      vec![15, 13, 2, 5],
      vec![14, 3, 4, 1],
      vec![12, 6, 8, 9],
      vec![16, 7, 10, 11]
    ]
  );
}
