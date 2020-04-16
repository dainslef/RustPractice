/**
 * 59. Spiral Matrix II
 * https://leetcode.com/problems/spiral-matrix-ii/
 *
 * Given a positive integer n, generate a square matrix filled with elements from 1 to n2 in spiral order.
 *
 * Example:
 *
 * Input: 3
 * Output:
 * [
 *  [ 1, 2, 3 ],
 *  [ 8, 9, 4 ],
 *  [ 7, 6, 5 ]
 * ]
 */

fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
  enum Dir {
    Up,
    Down,
    Left,
    Right,
  }

  impl Dir {
    fn next_dir(&self) -> Dir {
      match self {
        Dir::Up => Dir::Right,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
        Dir::Right => Dir::Down,
      }
    }
    fn next_index(&self, x: usize, y: usize) -> Option<(usize, usize)> {
      match self {
        Dir::Up => y.checked_sub(1).map(|y| (x, y)),
        Dir::Down => y.checked_add(1).map(|y| (x, y)),
        Dir::Left => x.checked_sub(1).map(|x| (x, y)),
        Dir::Right => x.checked_add(1).map(|x| (x, y)),
      }
    }
  }

  let (mut x, mut y, mut value, mut dir) = (0, 0, 1, Dir::Right);
  let mut matrix = (0..n)
    .map(|y| {
      (0..n)
        .map(|x| if x == 0 && y == 0 { 1 } else { 0 })
        .collect()
    })
    .collect::<Vec<Vec<i32>>>();

  loop {
    value += 1;
    let mut next_index = dir.next_index(x, y);

    macro_rules! invalid {
      () => {
        next_index
          .and_then(|(x, y)| matrix.get(y).and_then(|v| v.get(x)).map(|v| *v > 0))
          .unwrap_or(true)
      };
    }

    if invalid!() {
      dir = dir.next_dir();
      next_index = dir.next_index(x, y);
    }

    if invalid!() {
      break matrix;
    }

    x = next_index.unwrap().0;
    y = next_index.unwrap().1;
    matrix[y][x] = value;
  }
}

#[test]
fn q59_test() {
  assert_eq!(generate_matrix(3), [[1, 2, 3], [8, 9, 4], [7, 6, 5]]);
}
