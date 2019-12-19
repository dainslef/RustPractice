/**
 * Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.
 *
 * Example 1:
 *
 * Input:
 * [
 *  [ 1, 2, 3 ],
 *  [ 4, 5, 6 ],
 *  [ 7, 8, 9 ]
 * ]
 * Output: [1,2,3,6,9,8,7,4,5]
 * Example 2:
 *
 * Input:
 * [
 *   [1, 2, 3, 4],
 *   [5, 6, 7, 8],
 *   [9,10,11,12]
 * ]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 */

fn spiral_order_dir(matrix: Vec<Vec<i32>>) -> Vec<i32> {
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

  let (mut x, mut y, mut output, mut dir) = (0, 0, vec![], Dir::Right);
  let mut matrix = matrix
    .into_iter()
    .map(|r| r.into_iter().map(|v| Some(v)).collect())
    .collect::<Vec<Vec<Option<i32>>>>();

  loop {
    if let Some(Some(v)) = matrix.get(y).and_then(|v| v.get(x)) {
      output.push(*v);
      matrix[y][x] = None;
    }
    let mut next_index = dir.next_index(x, y);

    macro_rules! invalid {
      () => {
        next_index
          .and_then(|(x, y)| matrix.get(y).and_then(|v| v.get(x)).map(|v| v.is_none()))
          .unwrap_or(true)
      };
    }

    if invalid!() {
      dir = dir.next_dir();
      next_index = dir.next_index(x, y);
    }

    if invalid!() {
      break output;
    }

    x = next_index.unwrap().0;
    y = next_index.unwrap().1;
  }
}

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
  let (height, width) = match (matrix.len(), matrix.first().map(|v| v.len())) {
    (1, Some(_)) => return matrix[0].to_owned(),
    (_, Some(1)) => return matrix.into_iter().flatten().collect(),
    (_, None) => return vec![],
    (w, Some(h)) => (w, h),
  };

  let (mut x_index, mut y_index, mut temp) = (width - 1, height - 1, vec![]);
  let mut round = 0;

  while round < x_index && round < y_index {
    let (mut temp_x, mut temp_y) = (vec![], vec![]);

    for n in 0..x_index - round {
      temp.push(matrix[round][round + n]);
      temp_x.push(matrix[y_index][x_index - n]);
    }

    for n in 0..y_index - round {
      temp.push(matrix[round + n][x_index]);
      temp_y.push(matrix[y_index - n][round]);
    }

    temp.append(&mut temp_x);
    temp.append(&mut temp_y);

    round += 1;
    x_index -= 1;
    y_index -= 1;
  }

  if round == x_index && round == y_index {
    temp.push(matrix[width / 2][height / 2]);
  } else if round == x_index && y_index > round {
    for n in 0..=y_index - round {
      temp.push(matrix[round + n][x_index]);
    }
  } else if round == y_index && x_index > round {
    for n in 0..=x_index - round {
      temp.push(matrix[round][round + n]);
    }
  }

  temp
}

#[test]
fn q54_test() {
  fn test(spiral_order: impl Fn(Vec<Vec<i32>>) -> Vec<i32>) {
    assert_eq!(spiral_order(vec![vec![]]), vec![]);
    assert_eq!(spiral_order(vec![vec![1]]), vec![1]);
    assert_eq!(spiral_order(vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 4, 3]);
    assert_eq!(spiral_order(vec![vec![6], vec![9], vec![7]]), vec![6, 9, 7]);
    assert_eq!(spiral_order(vec![vec![6, 9, 7]]), vec![6, 9, 7]);
    assert_eq!(
      spiral_order(vec![vec![2, 5], vec![8, 4], vec![0, -1]]),
      vec![2, 5, 4, -1, 0, 8]
    );
    assert_eq!(
      spiral_order(vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15]
      ]),
      vec![1, 2, 3, 4, 5, 10, 15, 14, 13, 12, 11, 6, 7, 8, 9]
    );
    assert_eq!(
      spiral_order(vec![
        vec![2, 3, 4],
        vec![5, 6, 7],
        vec![8, 9, 10],
        vec![11, 12, 13],
        vec![14, 15, 16]
      ]),
      vec![2, 3, 4, 7, 10, 13, 16, 15, 14, 11, 8, 5, 6, 9, 12]
    );
    assert_eq!(
      spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
      vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(
      spiral_order(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12]
      ]),
      vec![1, 2, 3, 6, 9, 12, 11, 10, 7, 4, 5, 8]
    );
    assert_eq!(
      spiral_order(vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12]
      ]),
      vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
    assert_eq!(
      spiral_order(vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16]
      ]),
      vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
    );
  }

  test(spiral_order);
  test(spiral_order_dir);
}
