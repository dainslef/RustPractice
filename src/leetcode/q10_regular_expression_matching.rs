/**
 * Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.
 *
 * '.' Matches any single character.
 * '*' Matches zero or more of the preceding element.
 * The matching should cover the entire input string (not partial).
 *
 * Note:
 *
 * s could be empty and contains only lowercase letters a-z.
 * p could be empty and contains only lowercase letters a-z, and characters like . or *.
 * Example 1:
 *
 * Input:
 * s = "aa"
 * p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 * Example 2:
 *
 * Input:
 * s = "aa"
 * p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the precedeng element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 * Example 3:
 *
 * Input:
 * s = "ab"
 * p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 * Example 4:
 *
 * Input:
 * s = "aab"
 * p = "c*a*b"
 * Output: true
 * Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore it matches "aab".
 * Example 5:
 *
 * Input:
 * s = "mississippi"
 * p = "mis*is*p*."
 * Output: false
 */

fn is_match(s: String, p: String) -> bool {
  use std::collections::VecDeque;

  let (input, pattern) = (
    s.chars().collect::<VecDeque<char>>(),
    p.chars().collect::<VecDeque<char>>(),
  );

  fn deal_substring(mut input: VecDeque<char>, mut pattern: VecDeque<char>) -> bool {
    let mut last = None;

    loop {
      match (last, pattern.pop_front()) {
        (None, v @ Some(_)) => last = v,
        // continue only if the current match state is true
        (Some(last_char), Some('*'))
          if last_char >= 'a' && last_char <= 'z' || last_char == '.' =>
        {
          let mut next = input.clone();
          while if let Some(next_char) = next.pop_front() {
            last_char == '.' || last_char == next_char
          } else {
            false
          } {
            // continue with a new substring
            if deal_substring(next.clone(), pattern.clone()) {
              return true;
            }
          }
          last = None;
        }
        (Some(last_char), v) => {
          // get the current input char
          if let Some(current_char) = input.pop_front() {
            // check if char is match
            if last_char != '.' && current_char != last_char {
              return false;
            }
          } else {
            // if the input areadly empty, means the pattern isn't match
            return false;
          }
          last = v;
        }
        (None, None) => return input.is_empty(),
      }
    }
  }

  deal_substring(input, pattern)
}

#[test]
fn test_is_match() {
  assert_eq!(is_match("abb".to_string(), "bbb*".to_string()), false);
  assert_eq!(is_match("ab".to_string(), ".*c".to_string()), false);
  assert_eq!(
    is_match("mississippi".to_string(), "mis*is*ip*.".to_string()),
    true
  );
  assert_eq!(is_match("abaa".to_string(), "a*a".to_string()), false);
  assert_eq!(is_match("abaa".to_string(), "a*aa".to_string()), false);
  assert_eq!(is_match("abaa".to_string(), "ab*a".to_string()), false);
  assert_eq!(is_match("abaa".to_string(), "aba*".to_string()), true);
  assert_eq!(is_match("abaabb".to_string(), "a*ba*b*".to_string()), true);
  assert_eq!(
    is_match("abaabb".to_string(), "a*.*b.*a*b*".to_string()),
    true
  );
}
