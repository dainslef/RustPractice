/**
 * Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
 *
 * Each row must contain the digits 1-9 without repetition.
 * Each column must contain the digits 1-9 without repetition.
 * Each of the 9 3x3 sub-boxes of the grid must contain the digits 1-9 without repetition.
 *
 * A partially filled sudoku which is valid.
 *
 * The Sudoku board could be partially filled, where empty cells are filled with the character '.'.
 *
 * Example 1:
 *
 * Input:
 * [
 *   ["5","3",".",".","7",".",".",".","."],
 *   ["6",".",".","1","9","5",".",".","."],
 *   [".","9","8",".",".",".",".","6","."],
 *   ["8",".",".",".","6",".",".",".","3"],
 *   ["4",".",".","8",".","3",".",".","1"],
 *   ["7",".",".",".","2",".",".",".","6"],
 *   [".","6",".",".",".",".","2","8","."],
 *   [".",".",".","4","1","9",".",".","5"],
 *   [".",".",".",".","8",".",".","7","9"]
 * ]
 * Output: true
 * Example 2:
 *
 * Input:
 * [
 *   ["8","3",".",".","7",".",".",".","."],
 *   ["6",".",".","1","9","5",".",".","."],
 *   [".","9","8",".",".",".",".","6","."],
 *   ["8",".",".",".","6",".",".",".","3"],
 *   ["4",".",".","8",".","3",".",".","1"],
 *   ["7",".",".",".","2",".",".",".","6"],
 *   [".","6",".",".",".",".","2","8","."],
 *   [".",".",".","4","1","9",".",".","5"],
 *   [".",".",".",".","8",".",".","7","9"]
 * ]
 * Output: false
 * Explanation: Same as Example 1, except with the 5 in the top left corner being
 *     modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
 * Note:
 *
 * A Sudoku board (partially filled) could be valid but is not necessarily solvable.
 * Only the filled cells need to be validated according to the mentioned rules.
 * The given board contain only digits 1-9 and the character '.'.
 * The given board size is always 9x9.
 */

fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
  macro_rules! check {
    ($c:expr,$container:expr) => {
      if *$c != '.' && !$container.insert($c) {
        return false;
      }
    };
  }

  // Use Vec as container, has same time cost in LeetCode
  use std::collections::HashSet;

  const BOARD_SIZE: usize = 9;
  const CELL_SIZE: usize = 3;

  let (mut line, mut sub_box) = (HashSet::new(), HashSet::new());

  for x in 0..BOARD_SIZE {
    for c in &board[x] {
      // check the repetition of row
      check!(c, line);
    }
    line.clear();

    for y in 0..BOARD_SIZE {
      // check the repetition of column
      check!(&board[y][x], line);
      // check the index, compute the repeatability of every 9 cell
      if x % CELL_SIZE == 0 && y % CELL_SIZE == 0 {
        // cell range: 3 * 3
        for x in x..x + CELL_SIZE {
          for y in y..y + CELL_SIZE {
            check!(&board[x][y], sub_box);
          }
        }
        sub_box.clear();
      }
    }
    line.clear();
  }

  true
}

#[test]
fn q35_test() {
  assert_eq!(
    is_valid_sudoku(vec![
      vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ]),
    true
  );
  assert_eq!(
    is_valid_sudoku(vec![
      vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
      vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
      vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
      vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
      vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
      vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
      vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
      vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
      vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
    ]),
    false
  );
  assert_eq!(
    is_valid_sudoku(vec![
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
      vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']
    ]),
    true
  );
}
