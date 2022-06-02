/*!
[7. Reverse Integer](https://leetcode.com/problems/reverse-integer/)

Given a signed 32-bit integer `x`, return `x` with its digits reversed.
If reversing `x` causes the value to go outside the signed 32-bit integer range `[-2^31, 2^31 - 1]`, then return 0.

Example 1:

```html
Input: 123
Output: 321
```

Example 2:

```html
Input: -123
Output: -321
```

Example 3:

```html
Input: 120
Output: 21
```

Note:
Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
*/

fn reverse(x: i32) -> i32 {
  let mut new_str = String::new();

  for (_, c) in x.abs().to_string().char_indices() {
    new_str.insert(0, c);
  }

  if x < 0 {
    new_str.insert(0, '-');
  }

  new_str.parse::<i32>().unwrap_or(0)
}

#[test]
fn q7_test() {
  assert_eq!(reverse(123), 321);
  assert_eq!(reverse(87654321), 12345678);
}
