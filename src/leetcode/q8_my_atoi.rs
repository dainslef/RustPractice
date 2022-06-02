/*!
[8. String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/)

Implement the `myAtoi(string s)` function,
which converts a string to a 32-bit signed integer (similar to C/C++'s 1atoi1 function).

The algorithm for `myAtoi(string s)1 is as follows:

1. Read in and ignore any leading whitespace.
1. Check if the next character (if not already at the end of the string) is `'-`' or `'+'`.
Read this character in if it is either.
This determines if the final result is negative or positive respectively.
Assume the result is positive if neither is present.
1. Read in next the characters until the next non-digit character or the end of the input is reached.
The rest of the string is ignored.
1. Convert these digits into an integer (i.e. `"123" -> 123`, `"0032" -> 32`).
If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
1. If the integer is out of the 32-bit signed integer range `[-2^31, 2^31 - 1]`,
 then clamp the integer so that it remains in the range. Specifically,
integers less than `-2^31` should be clamped to `-2^31`,
and integers greater than `2^31 - 1` should be clamped to `2^31 - 1`.
Return the integer as the final result.

Note:

Only the space character ' ' is considered a whitespace character.
Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.

Example 1:

```html
Input: "42"
Output: 42
```

Example 2:

```html
Input: "   -42"
Output: -42
Explanation: The first non-whitespace character is '-', which is the minus sign.
             Then take as many numerical digits as possible, which gets 42.
```

Example 3:

```html
Input: "4193 with words"
Output: 4193
Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
```

Example 4:

```html
Input: "words and 987"
Output: 0
Explanation: The first non-whitespace character is 'w', which is not a numerical
             digit or a +/- sign. Therefore no valid conversion could be performed.
```

Example 5:

```html
Input: "-91283472332"
Output: -2147483648
Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
             Thefore INT_MIN (−231) is returned.
```
*/

fn my_atoi(s: String) -> i32 {
  let (mut temp, mut nums, mut is_negative) = (Some(0), vec![], false);

  // split context by space, get the first substring
  if let Some(num_str) = s.split_whitespace().next() {
    let mut not_zero = false;
    for (i, c) in num_str.char_indices() {
      if i == 0 && matches!(c, '+' | '-') {
        // check the first char
        if c == '-' {
          is_negative = true;
        }
      } else if let Some(n) = c.to_digit(10) {
        // check other chars
        if n > 0 || not_zero {
          nums.push(n as i32);
          not_zero = true;
        }
      } else {
        break;
      }
    }
  }

  for i in 0..nums.len() {
    let num = nums[nums.len() - i - 1];
    temp = {
      || {
        10_i32
          .checked_pow(i as u32)?
          .checked_mul(num)?
          .checked_add(temp?)
      }
    }();
  }

  temp
    .map(|v| if is_negative { 0 - v } else { v })
    .unwrap_or(if is_negative { i32::MIN } else { i32::MAX })
}

#[test]
fn q8_test() {
  assert_eq!(my_atoi("".into()), 0);
  assert_eq!(my_atoi(" 010  ".into()), 10);
  assert_eq!(my_atoi("  -0012a42".into()), -12);
  assert_eq!(my_atoi("   00000420000".into()), 420000);
  assert_eq!(my_atoi("  0000000000012345678".into()), 12345678);
  assert_eq!(my_atoi("3.14159".into()), 3);
  assert_eq!(my_atoi("      -42".into()), -42);
  assert_eq!(my_atoi("4193 with words".into()), 4193);
  assert_eq!(my_atoi("words and 987".into()), 0);
  assert_eq!(my_atoi("-91283472332".into()), -2147483648);
  assert_eq!(my_atoi("2147483648".into()), 2147483647);
}
