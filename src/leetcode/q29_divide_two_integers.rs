/**
 * Given two integers dividend and divisor, divide two integers without using multiplication, division and mod operator.
 *
 * Return the quotient after dividing dividend by divisor.
 *
 * The integer division should truncate toward zero.
 *
 * Example 1:
 *
 * Input: dividend = 10, divisor = 3
 * Output: 3
 * Example 2:
 *
 * Input: dividend = 7, divisor = -3
 * Output: -2
 * Note:
 *
 * Both dividend and divisor will be 32-bit signed integers.
 * The divisor will never be 0.
 * Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−2^31,  2^31 − 1]. For the purpose of this problem, assume that your function returns 2^31 − 1 when the division result overflows.
 */

fn divide(dividend: i32, divisor: i32) -> i32 {
  let negtive = |v| if v < 0 { v } else { 0 - v };

  let (mut num, div, mut count) = (negtive(dividend), negtive(divisor), 0);
  while num <= div {
    num -= div;
    count -= 1;
  }

  if dividend > 0 && divisor > 0 || dividend < 0 && divisor < 0 {
    0_i32.checked_sub(count).unwrap_or(std::i32::MAX)
  } else {
    count
  }
}

#[test]
fn test_divide() {
  assert_eq!(divide(0, 1), 0);
  assert_eq!(divide(1, 1), 1);
  assert_eq!(divide(3, -1), -3);
  assert_eq!(divide(100, 30), 3);
  assert_eq!(divide(-100, 30), -3);
  assert_eq!(divide(100, -30), -3);
  assert_eq!(divide(-100, -30), 3);
  assert_eq!(divide(0, -2147483648), 0);
  assert_eq!(divide(2147483647, -1), -2147483647);
  assert_eq!(divide(-2147483648, 2), -1073741824);
  assert_eq!(divide(-2147483648, 1), -2147483648);
  assert_eq!(divide(-2147483648, -1), 2147483647);
}
