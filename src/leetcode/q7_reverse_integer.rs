/**
 * Given a 32-bit signed integer, reverse digits of an integer.
 *
 * Example 1:
 *
 * Input: 123
 * Output: 321
 * Example 2:
 *
 * Input: -123
 * Output: -321
 * Example 3:
 *
 * Input: 120
 * Output: 21
 * Note:
 * Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
 */

fn reverse(x: i32) -> i32 {
  let mut new_str = String::new();

  for (_, c) in x.abs().to_string().char_indices() {
    new_str.insert(0, c)
  }

  if x < 0 {
    new_str.insert(0, '-')
  }

  new_str.parse::<i32>().unwrap_or(0)
}

#[test]
fn reverse_integer_test() {
  assert_eq!(reverse(123), 321);
  assert_eq!(reverse(87654321), 12345678);
}
