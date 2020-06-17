/**
 * The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.
 *
 * Given an integer n, return all distinct solutions to the n-queens puzzle.
 *
 * Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space respectively.
 *
 * Example:
 *
 * Input: 4
 * Output: [
 *  [".Q..",  // Solution 1
 *   "...Q",
 *   "Q...",
 *   "..Q."],
 *
 *  ["..Q.",  // Solution 2
 *   "Q...",
 *   "...Q",
 *   ".Q.."]
 * ]
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.
 */

// use recursion
fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
  // use a vec to record queen index in each row
  let mut records = Vec::new();

  fn recurse(record: Vec<i32>, records: &mut Vec<Vec<i32>>, n: i32) {
    let next_row = record.len() as i32;
    if next_row == n {
      records.push(record);
    } else {
      for next in 0..n {
        // check if row records aready contained this index
        if !record.contains(&next) {
          let mut is_valid = true;
          for row in 0..next_row {
            // compute the slope, use f32 to avoid accuracy decline
            let k = (next_row - row) as f32 / (next - record[row as usize]) as f32;
            // the slope shouldn't be 1 or -1
            is_valid = k.abs() != 1.0;
            if !is_valid {
              break;
            }
          }
          if is_valid {
            let mut next_record = record.clone();
            next_record.push(next);
            recurse(next_record, records, n);
          }
        }
      }
    }
  }

  recurse(vec![], &mut records, n);

  records
    .into_iter()
    .map(|v| {
      v.into_iter()
        .map(|q| (0..n).map(|i| if i == q { 'Q' } else { '.' }).collect())
        .collect()
    })
    .collect()
}

#[test]
fn q51_test() {
  assert_eq!(
    solve_n_queens(4),
    [
      [".Q..", "...Q", "Q...", "..Q."],
      ["..Q.", "Q...", "...Q", ".Q.."]
    ]
  );
  assert_eq!(
    solve_n_queens(5),
    [
      ["Q....", "..Q..", "....Q", ".Q...", "...Q."],
      ["Q....", "...Q.", ".Q...", "....Q", "..Q.."],
      [".Q...", "...Q.", "Q....", "..Q..", "....Q"],
      [".Q...", "....Q", "..Q..", "Q....", "...Q."],
      ["..Q..", "Q....", "...Q.", ".Q...", "....Q"],
      ["..Q..", "....Q", ".Q...", "...Q.", "Q...."],
      ["...Q.", "Q....", "..Q..", "....Q", ".Q..."],
      ["...Q.", ".Q...", "....Q", "..Q..", "Q...."],
      ["....Q", ".Q...", "...Q.", "Q....", "..Q.."],
      ["....Q", "..Q..", "Q....", "...Q.", ".Q..."]
    ]
  );
}
