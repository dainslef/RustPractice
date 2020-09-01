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
  match num_rows as usize {
    n if n > 1 => {
      let mut zipzag_rows: Vec<Vec<char>> = vec![vec![]; n];
      let unit_size = (n - 1) * 2;

      for (i, c) in s.char_indices() {
        let zipzag_index = match i % unit_size {
          m if m < n => m,
          m => unit_size - m,
        };
        zipzag_rows[zipzag_index].push(c);
      }

      zipzag_rows.into_iter().flat_map(|v| v).collect::<String>()
    }
    _ => s,
  }
}

#[test]
fn q6_test() {
  assert_eq!(convert("ABCDEFG".into(), 3), "AEBDFCG");
  assert_eq!(convert("ABCDEFG".into(), 4), "AGBFCED");
  assert_eq!(convert("PAYPALISHIRING".into(), 0), "PAYPALISHIRING");
  assert_eq!(convert("PAYPALISHIRING".into(), 1), "PAYPALISHIRING");
  assert_eq!(convert("PAYPALISHIRING".into(), 2), "PYAIHRNAPLSIIG");
  assert_eq!(convert("PAYPALISHIRING".into(), 3), "PAHNAPLSIIGYIR");
  assert_eq!(convert("PAYPALISHIRING".into(), 4), "PINALSIGYAHRPI");
}
