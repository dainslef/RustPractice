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

  // use 2-dimensional arrays to save the product of each of the two numbers
  // size: the length of num1 multiply the length of num2
  let (column_size, row_size) = (num1.len(), num2.len());
  let mut nums = (0..row_size)
    .map(|_| (0..column_size).map(|_| 0).collect())
    .collect::<Vec<Vec<u32>>>();

  // save the product of each of the two numbers
  for (x, c1) in num1.char_indices() {
    for (y, c2) in num2.char_indices() {
      let (row, column) = (num2.len() - y - 1, x);
      if let (Some(n1), Some(n2)) = (c1.to_digit(10), c2.to_digit(10)) {
        nums[row][column] = n1 * n2;
      }
    }
  }

  let mut plus = 0; // save the carry
  let mut result: VecDeque<u8> = VecDeque::new(); // save the ascii code of each digit of the result

  // compute the sum of the products
  macro_rules! compute {
    ($seq:expr, $row:expr, $column:expr) => {
      for n in $seq {
        let mut offset = 0;
        while let Some(v) = nums
          .get($row(n, offset))
          .and_then(|v| v.get($column(n, offset)))
        {
          plus += v;
          offset += 1;
        }
        // only calculate and record the product of one digit at a time
        result.push_front((plus % 10) as u8 + '0' as u8);
        plus /= 10;
      }
    };
  }

  // accumulate all the values
  compute!((0..column_size).rev(), |_, s| s, |v, s| v + s);
  compute!(1..row_size, |v, s| v + s, |_, s| s);

  // check if the last carry exists
  if plus != 0 {
    result.push_front((plus % 10) as u8 + '0' as u8);
  }

  String::from_utf8(result.into()).unwrap()
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
