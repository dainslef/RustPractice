/**
 * 64. Minimum Path Sum
 * https://leetcode.com/problems/minimum-path-sum/
 *
 * Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.
 *
 * Note: You can only move either down or right at any point in time.
 *
 * Example:
 *
 * Input:
 * [
 *   [1,3,1],
 *   [1,5,1],
 *   [4,2,1]
 * ]
 * Output: 7
 * Explanation: Because the path 1→3→1→1→1 minimizes the sum.
 */

fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
  if grid.is_empty() || grid[0].is_empty() {
    return 0;
  }

  let (row, column) = (grid[0].len(), grid.len());
  for y in 0..column {
    for x in 0..row {
      if x == 0 && y == 0 {
        continue;
      }
      let (mut left, mut top) = (i32::MAX, i32::MAX);
      if x != 0 {
        left = grid[y][x - 1]; // get the left value if the column is greater than 0
      }
      if y != 0 {
        top = grid[y - 1][x]; // get the top value if the row is greater than 0
      }
      // the current path value influenced by left and top square, compare and select the minimize value
      grid[y][x] += if left < top { left } else { top };
    }
  }

  grid[column - 1][row - 1]
}

#[test]
fn q64_test() {
  assert_eq!(min_path_sum(vec![]), 0);
  assert_eq!(min_path_sum(vec![vec![1]]), 1);
  assert_eq!(
    min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
    7
  );
}
