/**
 * Perform String Shifts
 * https://leetcode.com/explore/featured/card/30-day-leetcoding-challenge/529/week-2/3299/
 *
 * You are given a string s containing lowercase English letters, and a matrix shift, where shift[i] = [direction, amount]:
 *
 * direction can be 0 (for left shift) or 1 (for right shift).
 * amount is the amount by which string s is to be shifted.
 * A left shift by 1 means remove the first character of s and append it to the end.
 * Similarly, a right shift by 1 means remove the last character of s and add it to the beginning.
 * Return the final string after all operations.
 *
 *
 *
 * Example 1:
 *
 * Input: s = "abc", shift = [[0,1],[1,2]]
 * Output: "cab"
 * Explanation:
 * [0,1] means shift to left by 1. "abc" -> "bca"
 * [1,2] means shift to right by 2. "bca" -> "cab"
 * Example 2:
 *
 * Input: s = "abcdefg", shift = [[1,1],[1,1],[0,2],[1,3]]
 * Output: "efgabcd"
 * Explanation:
 * [1,1] means shift to right by 1. "abcdefg" -> "gabcdef"
 * [1,1] means shift to right by 1. "gabcdef" -> "fgabcde"
 * [0,2] means shift to left by 2. "fgabcde" -> "abcdefg"
 * [1,3] means shift to right by 3. "abcdefg" -> "efgabcd"
 *
 *
 * Constraints:
 *
 * 1 <= s.length <= 100
 * s only contains lower case English letters.
 * 1 <= shift.length <= 100
 * shift[i].length == 2
 * 0 <= shift[i][0] <= 1
 * 0 <= shift[i][1] <= 100
 */

fn string_shift(mut s: String, shift: Vec<Vec<i32>>) -> String {
  let mut offset = 0;

  for v in shift {
    match v.as_slice() {
      [0, s] => offset -= s, // move left, offset is negative
      [1, s] => offset += s, // move right, offset is positive
      _ => {}
    }
  }

  let length = s.len() as i32;
  let mut rem = offset % length; // it is no meaning if shfit beyond the length of current string
  if rem < 0 {
    // check the move direction
    rem += length; // move left "n" equals move right "length - n"
  }

  for _ in 0..rem {
    if let Some(c) = s.pop() {
      s.insert(0, c);
    }
  }

  s
}

#[test]
fn string_shift_test() {
  assert_eq!(string_shift("abc".into(), vec![vec![0, 1], vec![1, 2]]), "cab");
  assert_eq!(
    string_shift(
      "abcdefg".into(),
      vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]]
    ),
    "efgabcd"
  );
}
