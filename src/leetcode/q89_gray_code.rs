/**
 * 89. Gray Code
 * https://leetcode.com/problems/gray-code/
 *
 * The gray code is a binary numeral system where two successive values differ in only one bit.
 *
 * Given a non-negative integer n representing the total number of bits in the code, print the sequence of gray code. A gray code sequence must begin with 0.
 *
 * Example 1:
 *
 * Input: 2
 * Output: [0,1,3,2]
 * Explanation:
 * 00 - 0
 * 01 - 1
 * 11 - 3
 * 10 - 2
 *
 * For a given n, a gray code sequence may not be uniquely defined.
 * For example, [0,2,3,1] is also a valid gray code sequence.
 *
 * 00 - 0
 * 10 - 2
 * 11 - 3
 * 01 - 1
 * Example 2:
 *
 * Input: 0
 * Output: [0]
 * Explanation: We define the gray code sequence to begin with 0.
 *              A gray code sequence of n has size = 2^n, which for n = 0 the size is 2^0 = 1.
 *              Therefore, for n = 0 the gray code sequence is [0].
 */

fn gray_code(n: i32) -> Vec<i32> {
  let n = n as usize;
  let mut out = vec![0];

  for i in 0..n {
    let bit = 1 << i; // get the change bit position
    for j in (0..out.len()).rev() {
      // covert the existed number to new number
      // from the lastest number to the first number in collection
      out.push(out[j] | bit);
    }
  }

  out
}

#[test]
fn q89_test() {
  assert_eq!(gray_code(0), vec![0]);
  assert_eq!(gray_code(1), vec![0, 1]);
  assert_eq!(gray_code(2), vec![0, 1, 3, 2]);
  assert_eq!(gray_code(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
  assert_eq!(
    gray_code(4),
    vec![0, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15, 14, 10, 11, 9, 8]
  );
}
