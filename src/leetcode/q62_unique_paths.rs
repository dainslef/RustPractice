/**
 * 62. Unique Paths
 * https://leetcode.com/problems/unique-paths/
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 *
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 *
 * How many possible unique paths are there?
 *
 *
 * Above is a 7 x 3 grid. How many possible unique paths are there?
 *
 * Note: m and n will be at most 100.
 *
 * Example 1:
 *
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation:
 * From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down
 * 2. Right -> Down -> Right
 * 3. Down -> Right -> Right
 * Example 2:
 *
 * Input: m = 7, n = 3
 * Output: 28
 */

// fast solution, use the sum of adjacent nodes
fn unique_paths(m: i32, n: i32) -> i32 {
  let (row, column) = (m as usize, n as usize);
  let mut temp: Vec<Vec<i32>> = (0..column).map(|_| (0..row).map(|_| 0).collect()).collect();
  temp[0][0] = 1;

  for y in 0..column {
    for x in 0..row {
      // The number of unique paths to each square on the left border would be 1.
      // The number of unique paths to each square on the top border would be 1.
      // The number of unique paths to other squares on the left border would be the sum of the number of unique paths to its previous left square and top square.
      if x != 0 {
        // "x != 0" means you can add the value from left
        temp[y][x] += temp[y][x - 1];
      }
      if y != 0 {
        // "y != 0" means you can add the value from up
        temp[y][x] += temp[y - 1][x];
      }
    }
  }

  temp[column - 1][row - 1]
}

// too slow, TLE when m = 19, n = 13
fn unique_paths_recursion(m: i32, n: i32) -> i32 {
  fn recurse(p: (usize, usize), row: usize, column: usize) -> i32 {
    let mut count = 0;
    let (right, down) = ((p.0 + 1, p.1), (p.0, p.1 + 1));

    if p == (row - 1, column - 1) {
      count += 1;
    }
    if right.0 < row && right.1 < column {
      count += recurse(right, row, column);
    }
    if down.0 < row && down.1 < column {
      count += recurse(down, row, column);
    }

    count
  }

  recurse((0, 0), m as usize, n as usize)
}

#[test]
fn q62_test() {
  assert_eq!(unique_paths(1, 1), 1);
  assert_eq!(unique_paths(2, 1), 1);
  assert_eq!(unique_paths(3, 2), 3);
  assert_eq!(unique_paths(7, 3), 28);
  assert_eq!(unique_paths(10, 10), 48620);
  assert_eq!(unique_paths(100, 2), 100);
}
