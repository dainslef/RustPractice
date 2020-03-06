/**
 * Write a program to solve a Sudoku puzzle by filling the empty cells.
 *
 * A sudoku solution must satisfy all of the following rules:
 *
 * Each of the digits 1-9 must occur exactly once in each row.
 * Each of the digits 1-9 must occur exactly once in each column.
 * Each of the the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
 * Empty cells are indicated by the character '.'.
 *
 *
 * A sudoku puzzle...
 *
 *
 * ...and its solution numbers marked in red.
 *
 * Note:
 *
 * The given board contain only digits 1-9 and the character '.'.
 * You may assume that the given Sudoku puzzle will have a single unique solution.
 * The given board size is always 9x9.
 */

fn solve_sudoku(board: &mut Vec<Vec<char>>) {
  use std::collections::HashSet;

  const BOARD_SIZE: usize = 9;
  const CELL_SIZE: usize = 3;

  let nums = (49_u8..=57_u8)
    .map(|v| v as char)
    .collect::<HashSet<char>>();

  fn around_index(row: usize, column: usize) -> usize {
    row / CELL_SIZE * CELL_SIZE + column / CELL_SIZE
  }

  let (mut rows, mut columns, mut arounds): (Vec<_>, Vec<_>, Vec<HashSet<char>>) =
    (vec![], vec![], vec![]);
  for x in 0..BOARD_SIZE {
    rows.push(board[x].clone().into_iter().collect()); // add rows
    columns.push(HashSet::new()); // add new column
    for y in 0..BOARD_SIZE {
      columns[x].insert(board[y][x]); // add column content
      let i = around_index(x, y);
      if let Some(v) = arounds.get_mut(i) {
        if board[x][y] != '.' {
          v.insert(board[x][y]);
        }
      } else {
        arounds.push(HashSet::new());
      }
    }
  }

  fn recurse(
    row: usize,
    column: usize,
    input: &mut Vec<Vec<char>>,
    rows: &mut Vec<HashSet<char>>,
    columns: &mut Vec<HashSet<char>>,
    arounds: &mut Vec<HashSet<char>>,
    nums: &HashSet<char>,
  ) -> bool {
    let (next_row, next_column) = match (row + 1, column + 1) {
      (row, BOARD_SIZE) => (row, 0),
      (row, column) => (row - 1, column),
    };

    macro_rules! next {
      () => {
        // check if the sudoku is sovled, or run the next solve operate
        row == BOARD_SIZE - 1 && column == BOARD_SIZE - 1
          || recurse(next_row, next_column, input, rows, columns, arounds, nums)
      };
    }

    if input[row][column] != '.' {
      if next!() {
        return true;
      }
    } else {
      let around = around_index(row, column);
      let mut difference = nums.clone();
      difference.retain(|v| {
        // compute values match sudoku rules, use retain function instead of difference function
        !rows[row].contains(v) && !columns[column].contains(v) && !arounds[around].contains(v)
      });

      for v in difference {
        rows[row].insert(v);
        columns[column].insert(v);
        arounds[around].insert(v);
        input[row][column] = v;
        if next!() {
          return true;
        } else {
          // if the answer isn't found, unchange the values
          rows[row].remove(&v);
          columns[column].remove(&v);
          arounds[around].remove(&v);
          input[row][column] = '.';
        }
      }
    }

    false
  }

  recurse(0, 0, board, &mut rows, &mut columns, &mut arounds, &nums);
}

#[test]
fn q37_test() {
  let mut temp = vec![
    vec!['.', '.', '9', '7', '4', '8', '.', '.', '.'],
    vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
    vec!['.', '2', '.', '1', '.', '9', '.', '.', '.'],
    vec!['.', '.', '7', '.', '.', '.', '2', '4', '.'],
    vec!['.', '6', '4', '.', '1', '.', '5', '9', '.'],
    vec!['.', '9', '8', '.', '.', '.', '3', '.', '.'],
    vec!['.', '.', '.', '8', '.', '3', '.', '2', '.'],
    vec!['.', '.', '.', '.', '.', '.', '.', '.', '6'],
    vec!['.', '.', '.', '2', '7', '5', '9', '.', '.'],
  ];
  solve_sudoku(&mut temp);
  assert_eq!(
    temp,
    [
      ['5', '1', '9', '7', '4', '8', '6', '3', '2'],
      ['7', '8', '3', '6', '5', '2', '4', '1', '9'],
      ['4', '2', '6', '1', '3', '9', '8', '7', '5'],
      ['3', '5', '7', '9', '8', '6', '2', '4', '1'],
      ['2', '6', '4', '3', '1', '7', '5', '9', '8'],
      ['1', '9', '8', '5', '2', '4', '3', '6', '7'],
      ['9', '7', '5', '8', '6', '3', '1', '2', '4'],
      ['8', '3', '2', '4', '9', '1', '7', '5', '6'],
      ['6', '4', '1', '2', '7', '5', '9', '8', '3']
    ]
  );

  let mut temp = vec![
    vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
  ];
  solve_sudoku(&mut temp);
  assert_eq!(
    temp,
    [
      ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
      ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
      ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
      ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
      ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
      ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
      ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
      ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
      ['3', '4', '5', '2', '8', '6', '1', '7', '9']
    ]
  );
}
