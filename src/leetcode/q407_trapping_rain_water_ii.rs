/**
 * Given an m x n matrix of positive integers representing the height of each unit cell in a 2D elevation map, compute the volume of water it is able to trap after raining.
 *
 *
 *
 * Note:
 *
 * Both m and n are less than 110. The height of each unit cell is greater than 0 and is less than 20,000.
 *
 *
 *
 * Example:
 *
 * Given the following 3x6 height map:
 * [
 *   [1,4,3,1,3,2],
 *   [3,2,1,3,2,4],
 *   [2,3,3,2,3,1]
 * ]
 *
 * Return 4.
 *
 *
 * The above image represents the elevation map [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]] before the rain.
 *
 *
 *
 *
 *
 * After the rain, water is trapped between the blocks. The total volume of water trapped is 4.
 */

fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
  use std::{cmp::Ordering, collections::BinaryHeap};

  if height_map.len() == 0 {
    return 0;
  }

  #[derive(std::fmt::Debug, PartialEq, Eq, Ord)]
  struct Cell {
    x: usize,
    y: usize,
    h: i32,
  }

  impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Option::Some(match (self.h, other.h) {
        (h1, h2) if h1 > h2 => Ordering::Less,
        (h1, h2) if h1 < h2 => Ordering::Greater,
        _ => Ordering::Equal,
      })
    }
  }

  const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

  let (row, column) = (height_map.len(), height_map[0].len());
  let mut heap: BinaryHeap<Cell> = BinaryHeap::new();
  let mut visited = (0..row)
    .map(|_| (0..column).map(|_| false).collect())
    .collect::<Vec<Vec<bool>>>();

  for y in 0..row {
    let (x1, x2) = (0, column - 1);
    heap.push(Cell {
      x: x1,
      y,
      h: height_map[y][x1],
    });
    heap.push(Cell {
      x: x2,
      y,
      h: height_map[y][x2],
    });
    visited[y][x1] = true;
    visited[y][x2] = true;
  }

  for x in 1..column {
    let (y1, y2) = (0, row - 1);
    heap.push(Cell {
      x,
      y: y1,
      h: height_map[y1][x],
    });
    heap.push(Cell {
      x,
      y: y2,
      h: height_map[y2][x],
    });
    visited[y1][x] = true;
    visited[y2][x] = true;
  }

  let mut water = 0;
  while let Some(cell) = heap.pop() {
    for (offset_x, offset_y) in &DIRS {
      let (x, y) = (
        (cell.x as i32 + offset_x) as usize,
        (cell.y as i32 + offset_y) as usize,
      );
      if x > 0 && x < column && y > 0 && y < row && !visited[y][x] {
        let (current_h, next_h) = (cell.h, height_map[y][x]);
        heap.push(Cell {
          x,
          y,
          h: if current_h > next_h {
            water += current_h - next_h;
            current_h
          } else {
            next_h
          },
        });
        visited[y][x] = true;
      }
    }
  }

  water
}

#[test]
fn test_q407() {
  assert_eq!(trap_rain_water(vec![]), 0);
  assert_eq!(
    trap_rain_water(vec![
      vec![1, 4, 3, 1, 3, 2],
      vec![3, 2, 1, 3, 2, 4],
      vec![2, 3, 3, 2, 3, 1]
    ]),
    4
  );
}
