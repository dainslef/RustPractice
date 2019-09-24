/**
 * Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 *
 * Example 1:
 *
 * Input: num1 = "2", num2 = "3"
 * Output: "6"
 * Example 2:
 *
 * Input: num1 = "123", num2 = "456"
 * Output: "56088"
 * Note:
 *
 * The length of both num1 and num2 is < 110.
 * Both num1 and num2 contain only digits 0-9.
 * Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 * You must not use any built-in BigInteger library or convert the inputs to integer directly.
 */

fn multiply(num1: String, num2: String) -> String {
  if num1 == "0" || num2 == "0" {
    return "0".into();
  };

  use std::collections::VecDeque;

  let (column_size, row_size) = (num1.len(), num2.len());
  let mut nums = (0..row_size)
    .map(|_| (0..column_size).map(|_| 0).collect())
    .collect::<Vec<Vec<i32>>>();

  fn n(c: char) -> i32 {
    c as i32 - '0' as i32
  }

  for (x, c1) in num1.char_indices() {
    for (y, c2) in num2.char_indices() {
      let (row, column) = (num2.len() - y - 1, x);
      nums[row][column] = n(c1) * n(c2);
    }
  }

  let mut plus = 0;
  let mut result: VecDeque<char> = VecDeque::new();

  let mut update_result = |plus: i32| -> i32 {
    result.push_front(((plus % 10) as u8 + '0' as u8) as char);
    plus / 10
  };

  for x in (0..column_size).rev() {
    let mut offset = 0;
    while let Some(v) = nums.get(0 + offset).and_then(|v| v.get(x + offset)) {
      plus += v;
      offset += 1;
    }
    plus = update_result(plus);
  }

  for y in 1..row_size {
    let mut offset = 0;
    while let Some(v) = nums.get(y + offset).and_then(|v| v.get(0 + offset)) {
      plus += v;
      offset += 1;
    }
    plus = update_result(plus);
  }

  if plus != 0 {
    update_result(plus);
  }

  use std::iter::FromIterator;
  String::from_iter(result.into_iter())
}

#[test]
fn q43_test() {
  assert_eq!(multiply("909".into(), "990".into()), "899910");
  assert_eq!(multiply("123".into(), "4567".into()), "561741");
  assert_eq!(multiply("99".into(), "9".into()), "891");
  assert_eq!(multiply("99".into(), "999".into()), "98901");
  assert_eq!(multiply("123".into(), "45".into()), "5535");
  assert_eq!(multiply("0".into(), "100".into()), "0");
  assert_eq!(multiply("1".into(), "1".into()), "1");
  assert_eq!(multiply("2".into(), "3".into()), "6");
  assert_eq!(
    multiply("999999999999999".into(), "999999999999999999".into()),
    "999999999999998999000000000000001"
  );
}
