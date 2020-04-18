/**
 * 200. Number of Islands
 * https://leetcode.com/problems/number-of-islands/
 *
 * Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
 *
 * Example 1:
 *
 * Input:
 * 11110
 * 11010
 * 11000
 * 00000
 *
 * Output: 1
 * Example 2:
 *
 * Input:
 * 11000
 * 11000
 * 00100
 * 00011
 *
 * Output: 3
 */

fn num_islands(grid: Vec<Vec<char>>) -> i32 {
  if grid.is_empty() || grid[0].is_empty() {
    return 0;
  }

  use std::collections::HashSet;
  let mut points: HashSet<(usize, usize)> = Default::default();

  // save all the location related points
  fn save_points(grid: &Vec<Vec<char>>, points: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
    if x < grid[0].len() && y < grid.len() {
      // check if the index is valid
      let v = grid[y][x];
      // check if the point is valid and doesn't have been saved
      if v == '1' && points.insert((x, y)) {
        // recurse to save the points surrounding this point
        save_points(grid, points, x + 1, y);
        save_points(grid, points, x, y + 1);
        if x > 0 {
          save_points(grid, points, x - 1, y);
        }
        if y > 0 {
          save_points(grid, points, x, y - 1);
        }
      }
    }
  }

  let mut count = 0;
  for y in 0..grid.len() {
    for x in 0..grid[0].len() {
      // check if the current point is in saved points
      if grid[y][x] == '1' && !points.contains(&(x, y)) {
        count += 1;
        save_points(&grid, &mut points, x, y);
      }
    }
  }

  count
}

#[test]
fn q200_test() {
  assert_eq!(num_islands(vec![vec!['0']]), 0);
  assert_eq!(num_islands(vec![vec!['1']]), 1);
  assert_eq!(num_islands(vec![vec!['1', '0'], vec!['0', '1'],]), 2);
  assert_eq!(
    num_islands(vec![
      vec!['1', '1', '1'],
      vec!['0', '1', '0'],
      vec!['1', '1', '1']
    ]),
    1
  );
  assert_eq!(
    num_islands(vec![
      vec!['0', '1', '0'],
      vec!['1', '0', '1'],
      vec!['0', '1', '0']
    ]),
    4
  );
  assert_eq!(
    num_islands(vec![
      vec!['1', '1', '1', '1', '0'],
      vec!['1', '1', '0', '1', '0'],
      vec!['0', '0', '0', '0', '0'],
      vec!['0', '0', '0', '0', '0'],
    ]),
    1
  );
  assert_eq!(
    num_islands(vec![
      vec!['1', '1', '0', '0', '0'],
      vec!['1', '1', '0', '0', '0'],
      vec!['0', '0', '1', '0', '0'],
      vec!['0', '0', '0', '1', '1'],
    ]),
    3
  );
}
