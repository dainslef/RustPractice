/**
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * Example 1:
 *
 * Input: "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 * Example 2:
 *
 * Input: "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 * Example 3:
 *
 * Input: "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 *              Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
 */

fn length_of_longest_substring(s: String) -> i32 {
  use std::collections::VecDeque;

  let (mut count, mut temp) = (0, VecDeque::new());

  for c in s.chars() {
    while temp.contains(&c) {
      temp.pop_front();
    }
    temp.push_back(c);
    if temp.len() > count {
      count = temp.len();
    }
  }

  count as i32
}

#[test]
fn q3_test() {
  assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
  assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
  assert_eq!(length_of_longest_substring("aa789".into()), 4);
  assert_eq!(length_of_longest_substring("\"\"".into()), 1);
  assert_eq!(length_of_longest_substring("\" \"".into()), 2);
  assert_eq!(length_of_longest_substring("dvdf".into()), 3);
}
