/**
 * Implement pow(x, n), which calculates x raised to the power n (xn).
 *
 * Example 1:
 *
 * Input: 2.00000, 10
 * Output: 1024.00000
 * Example 2:
 *
 * Input: 2.10000, 3
 * Output: 9.26100
 * Example 3:
 *
 * Input: 2.00000, -2
 * Output: 0.25000
 * Explanation: 2-2 = 1/22 = 1/4 = 0.25
 * Note:
 *
 * -100.0 < x < 100.0
 * n is a 32-bit signed integer, within the range [−231, 231 − 1]
 */

fn my_pow(x: f64, n: i32) -> f64 {
  match n {
    0 => 1.0,
    1 => x,
    -1 => 1.0 / x,
    _ => {
      let v = my_pow(x, n / 2); // avoid duplicate computation
      v * v * my_pow(x, n % 2) // split computation process into smaller parts
    }
  }
}

#[test]
fn q50_test() {
  assert_eq!(my_pow(0.0, 0), 1.0);
  assert_eq!(my_pow(0.0, 2147483647), 0.0);
  assert_eq!(my_pow(0.00001, 2147483647), 0.0);
  assert_eq!(my_pow(1111.0, 0), 1.0);
  assert_eq!(my_pow(2.0, 10), 1024.0);
  assert_eq!(my_pow(2.0, -2), 0.25);
}
