/**
 * The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.
 *
 * Given an integer n, return the number of distinct solutions to the n-queens puzzle.
 *
 * Example:
 *
 * Input: 4
 * Output: 2
 * Explanation: There are two distinct solutions to the 4-queens puzzle as shown below.
 * [
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
 */

// use loop
fn total_n_queens(n: i32) -> i32 {
  let mut records = vec![vec![]];

  for next_row in 0..n {
    let mut next_records = vec![];
    for record in records {
      // test the all possible position (0 to n)
      for next in 0..n {
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
            next_records.push(next_record);
          }
        }
      }
    }
    records = next_records;
  }

  records.len() as i32
}

#[test]
fn test_total_n_queens() {
  assert_eq!(total_n_queens(4), 2);
  assert_eq!(total_n_queens(5), 10);
}
