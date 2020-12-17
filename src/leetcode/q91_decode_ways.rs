/**
 * 91. Decode Ways
 * https://leetcode.com/problems/decode-ways/
 *
 * A message containing letters from A-Z is being encoded to numbers using the following mapping:
 *
 * 'A' -> 1
 * 'B' -> 2
 * ...
 * 'Z' -> 26
 * Given a non-empty string containing only digits, determine the total number of ways to decode it.
 *
 * The answer is guaranteed to fit in a 32-bit integer.
 *
 *
 *
 * Example 1:
 *
 * Input: s = "12"
 * Output: 2
 * Explanation: It could be decoded as "AB" (1 2) or "L" (12).
 * Example 2:
 *
 * Input: s = "226"
 * Output: 3
 * Explanation: It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
 * Example 3:
 *
 * Input: s = "0"
 * Output: 0
 * Explanation: There is no character that is mapped to a number starting with '0'. We cannot ignore a zero when we face it while decoding. So, each '0' should be part of "10" --> 'J' or "20" --> 'T'.
 * Example 4:
 *
 * Input: s = "1"
 * Output: 1
 *
 *
 * Constraints:
 *
 * 1 <= s.length <= 100
 * s contains only digits and may contain leading zero(s).
 */

/**
 * Runtime: 0 ms, faster than 100.00% of Rust online submissions for Decode Ways.
 * Memory Usage: 2 MB, less than 68.42% of Rust online submissions for Decode Ways.
 *
 * Don't use the solution which based on recursion;
 * don't try to copy or split the origin string.
 *
 * Some test case has a large size of elements,
 * too much recursion operations may cause the "stack overflow" excception.
 */
fn num_decodings(s: String) -> i32 {
  let codes: Vec<_> = s.chars().map(|v| v.to_digit(10).unwrap()).collect();
  let mut counts = vec![];

  for i in 0..s.len() {
    let n = codes[i];
    let mut count = if n == 0 {
      0 // if the current number is 0, means current index can't direct use as a valid char, so count is 0
    } else if counts.is_empty() {
      1 // if the last count is not exist (and current number is not 0), set the init count
    } else {
      counts[i - 1] // or get the last count
    };
    if i > 0 {
      // if current index is larger than 0,
      // check weaher the last number and the current number can combine to a char
      let last_n = codes[i - 1];
      // the sum must between 1 to 26, and the mode "0 n" is invalid (last number can't be 0)
      if last_n != 0 && last_n * 10 + n <= 26 {
        count += if i == 1 { 1 } else { counts[i - 2] }
      }
    }
    counts.push(count);
  }

  *counts.last().unwrap_or(&0) as i32
}

#[test]
fn q91_test() {
  assert_eq!(num_decodings("1010102101012012010210210".into()), 1);
  assert_eq!(num_decodings("1230123102301023013010".into()), 0);
  assert_eq!(num_decodings("10".into()), 1);
  assert_eq!(num_decodings("12".into()), 2);
  assert_eq!(num_decodings("226".into()), 3);
  assert_eq!(num_decodings("0".into()), 0);
  assert_eq!(num_decodings("1".into()), 1);
  assert_eq!(num_decodings("12312312312".into()), 54);
  assert_eq!(num_decodings("2010316520123123123102".into()), 54);
  assert_eq!(num_decodings("111111".into()), 13);
  assert_eq!(
    num_decodings("111111111111111111111111111111111111111111111".into()),
    1836311903
  );
}
