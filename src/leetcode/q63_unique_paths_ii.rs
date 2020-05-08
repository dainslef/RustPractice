/**
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 *
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 *
 * Now consider if some obstacles are added to the grids. How many unique paths would there be?
 *
 *
 *
 * An obstacle and empty space is marked as 1 and 0 respectively in the grid.
 *
 * Note: m and n will be at most 100.
 *
 * Example 1:
 *
 * Input:
 * [
 *   [0,0,0],
 *   [0,1,0],
 *   [0,0,0]
 * ]
 * Output: 2
 * Explanation:
 * There is one obstacle in the middle of the 3x3 grid above.
 * There are two ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down -> Down
 * 2. Down -> Down -> Right -> Right
 */

fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
  if obstacle_grid.is_empty() || obstacle_grid[0].is_empty() {
    return 0;
  }
  let (row, column) = (obstacle_grid[0].len(), obstacle_grid.len());

  for y in 0..column {
    for x in 0..row {
      if x == 0 && y == 0 {
        obstacle_grid[y][x] = 1 - obstacle_grid[y][x];
      } else if obstacle_grid[y][x] == 1 {
        obstacle_grid[y][x] = 0; // if the current grid is invalid, it should be set to 0
      } else {
        if x != 0 {
          // "x != 0" means you can add the value from left
          obstacle_grid[y][x] += obstacle_grid[y][x - 1];
        }
        if y != 0 {
          // "y != 0" means you can add the value from up
          obstacle_grid[y][x] += obstacle_grid[y - 1][x];
        }
      }
    }
  }

  obstacle_grid[column - 1][row - 1]
}

#[test]
fn q63_test() {
  assert_eq!(
    unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
    2
  );
  assert_eq!(
    unique_paths_with_obstacles(vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 0, 0]]),
    1
  );
  assert_eq!(
    unique_paths_with_obstacles(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 0]]),
    3
  );
  assert_eq!(
    unique_paths_with_obstacles(vec![vec![0], vec![0], vec![0]]),
    1
  );
}
