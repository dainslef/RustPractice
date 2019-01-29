/**
 * Implement atoi which converts a string to an integer.
 *
 * The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
 *
 * The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
 *
 * If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.
 *
 * If no valid conversion could be performed, a zero value is returned.
 *
 * Note:
 *
 * Only the space character ' ' is considered as whitespace character.
 * Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. If the numerical value is out of the range of representable values, INT_MAX (231 − 1) or INT_MIN (−231) is returned.
 * Example 1:
 *
 * Input: "42"
 * Output: 42
 * Example 2:
 *
 * Input: "   -42"
 * Output: -42
 * Explanation: The first non-whitespace character is '-', which is the minus sign.
 *              Then take as many numerical digits as possible, which gets 42.
 * Example 3:
 *
 * Input: "4193 with words"
 * Output: 4193
 * Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
 * Example 4:
 *
 * Input: "words and 987"
 * Output: 0
 * Explanation: The first non-whitespace character is 'w', which is not a numerical
 *              digit or a +/- sign. Therefore no valid conversion could be performed.
 * Example 5:
 *
 * Input: "-91283472332"
 * Output: -2147483648
 * Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
 *              Thefore INT_MIN (−231) is returned.
 */

fn my_atoi(s: String) -> i32 {
  let (mut temp, mut nums, mut is_valid, mut is_negative) = (Some(0), vec![], true, false);

  if let Some(num_str) = s.split_whitespace().next() {
    for (i, c) in num_str.char_indices() {
      if i == 0 && (c == '+' || c == '-') {
        if c == '-' {
          is_negative = true;
        }
        continue;
      }

      if let Some(n) = c.to_digit(10) {
        nums.push(n as i32);
      } else {
        if c != '.' {
          is_valid = false;
        }
        break;
      }
    }
  }

  fn checked_pow(mut exp: usize) -> Option<i32> {
    let (mut base, mut acc) = (10, 1_i32);

    while exp > 1 {
      if (exp & 1) == 1 {
        acc = acc.checked_mul(base)?;
      }
      exp /= 2;
      base = base.checked_mul(base)?;
    }

    if exp == 1 {
      acc = acc.checked_mul(base)?;
    }

    Some(acc)
  }

  if is_valid {
    nums.reverse();
    for i in 0..nums.len() {
      let update_temp = || checked_pow(i)?.checked_mul(nums[i])?.checked_add(temp?);
      temp = update_temp();
    }
  }

  temp
    .map(|v| if is_negative { 0 - v } else { v })
    .unwrap_or(std::i32::MIN)
}

#[test]
fn test_my_atoi() {
  assert_eq!(my_atoi("".to_string()), 0);
  assert_eq!(my_atoi("   ".to_string()), 0);
  assert_eq!(my_atoi("42".to_string()), 42);
  assert_eq!(my_atoi("3.14159".to_string()), 3);
  assert_eq!(my_atoi("      -42".to_string()), -42);
  assert_eq!(my_atoi("4193 with words".to_string()), 4193);
  assert_eq!(my_atoi("words and 987".to_string()), 0);
  assert_eq!(my_atoi("91283472332".to_string()), -2147483648);
}