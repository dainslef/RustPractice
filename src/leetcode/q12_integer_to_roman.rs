/**
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty seven is written as XXVII, which is XX + V + II.
 *
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 * I can be placed before V (5) and X (10) to make 4 and 9.
 * X can be placed before L (50) and C (100) to make 40 and 90.
 * C can be placed before D (500) and M (1000) to make 400 and 900.
 * Given an integer, convert it to a roman numeral. Input is guaranteed to be within the range from 1 to 3999.
 *
 * Example 1:
 *
 * Input: 3
 * Output: "III"
 * Example 2:
 *
 * Input: 4
 * Output: "IV"
 * Example 3:
 *
 * Input: 9
 * Output: "IX"
 * Example 4:
 *
 * Input: 58
 * Output: "LVIII"
 * Explanation: L = 50, V = 5, III = 3.
 * Example 5:
 *
 * Input: 1994
 * Output: "MCMXCIV"
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 */

fn int_to_roman(num: i32) -> String {
  use std::collections::HashMap;

  let (mut re, mut temp, mut values) = (num, vec![], vec![1, 10, 100, 1000]);
  let mut char_to_value = HashMap::new();
  char_to_value.insert(1, "I");
  char_to_value.insert(4, "IV");
  char_to_value.insert(5, "V");
  char_to_value.insert(9, "IX");
  char_to_value.insert(10, "X");
  char_to_value.insert(40, "XL");
  char_to_value.insert(50, "L");
  char_to_value.insert(90, "XC");
  char_to_value.insert(100, "C");
  char_to_value.insert(400, "CD");
  char_to_value.insert(500, "D");
  char_to_value.insert(900, "CM");
  char_to_value.insert(1000, "M");

  while let Some(step) = values.pop() {
    while re / step > 0 {
      match re / step {
        n @ 4 | n @ 9 => temp.push(char_to_value[&(n * step)]),
        n @ _ => {
          let start = if n < 5 {
            0
          } else {
            temp.push(char_to_value[&(5 * step)]);
            5
          };
          for _ in start..n {
            temp.push(char_to_value[&step]);
          }
        }
      };
      re %= step;
    }
  }

  use std::iter::FromIterator;
  String::from_iter(temp)
}

#[test]
fn test_int_to_roman() {
  assert_eq!(int_to_roman(3), "III");
  assert_eq!(int_to_roman(58), "LVIII");
  assert_eq!(int_to_roman(1994), "MCMXCIV");
  assert_eq!(int_to_roman(3999), "MMMCMXCIX");
}
