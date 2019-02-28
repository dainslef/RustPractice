/**
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * And then read line by line: "PAHNAPLSIIGYIR"
 *
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 * string convert(string s, int numRows);
 * Example 1:
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 * Example 2:
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 *
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 */

fn convert(s: String, num_rows: i32) -> String {
  match num_rows {
    n if n > 1 => {
      let num_rows = num_rows as usize;
      let mut rows = (0..num_rows).map(|_| vec![]).collect::<Vec<Vec<char>>>();
      let unit_size = (num_rows - 1) * 2;

      for (i, c) in s.char_indices() {
        let m = i % unit_size;
        let row_index = match m {
          m if m < num_rows => m,
          m => 2 * (num_rows - 1) - m
        };
        rows[row_index].push(c);
      }

      let row_all = rows.iter().flat_map(|v| v).collect::<Vec<&char>>();

      use std::iter::FromIterator;
      String::from_iter(row_all)
    }
    _ => s
  }
}

#[test]
fn test_convert() {
  assert_eq!(convert("ABCDEFG".to_string(), 3), "AEBDFCG");
  assert_eq!(convert("ABCDEFG".to_string(), 4), "AGBFCED");
  assert_eq!(convert("PAYPALISHIRING".to_string(), 0), "PAYPALISHIRING");
  assert_eq!(convert("PAYPALISHIRING".to_string(), 1), "PAYPALISHIRING");
  assert_eq!(convert("PAYPALISHIRING".to_string(), 2), "PYAIHRNAPLSIIG");
  assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
  assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
}
