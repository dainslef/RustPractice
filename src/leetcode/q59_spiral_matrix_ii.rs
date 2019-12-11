/**
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

  struct Position {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    value: i32,
    dir: Dir,
    matrix: Vec<Vec<i32>>,
  }

  impl Position {
    fn new(size: usize) -> Position {
      Position {
        x: 0,
        y: 0,
        width: size,
        height: size,
        value: 1,
        dir: Dir::Right,
        matrix: (0..size)
          .map(|y| {
            (0..size)
              .map(|x| if x == 0 && y == 0 { 1 } else { 0 })
              .collect()
          })
          .collect::<Vec<Vec<i32>>>(),
      }
    }
    fn next(&mut self) -> bool {
      self.value += 1;
      let mut next_index = self.dir.next_index(self.x, self.y);

      macro_rules! invalid {
        () => {
          next_index
            .and_then(|(x, y)| self.matrix.get(y).and_then(|v| v.get(x)).map(|v| *v > 0))
            .unwrap_or(true)
        };
      }

      if invalid!() {
        self.dir = self.dir.next_dir();
        next_index = self.dir.next_index(self.x, self.y);
      }

      if invalid!() {
        false
      } else {
        self.x = next_index.unwrap().0;
        self.y = next_index.unwrap().1;
        self.matrix[self.y][self.x] = self.value;
        true
      }
    }
  }

  let mut position = Position::new(n as usize);
  loop {
    if !position.next() {
      return position.matrix;
    }
  }
}

#[test]
fn q59_test() {
  assert_eq!(generate_matrix(3), [[1, 2, 3], [8, 9, 4], [7, 6, 5]]);
}
