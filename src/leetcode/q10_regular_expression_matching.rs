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
    let (mut last, mut is_match) = (None, true);

    loop {
      match (last, pattern.pop_front()) {
        (None, v @ Some(_)) => last = v,
        // continue only if the current match state is true
        (Some(last_char), Some('*')) if is_match => {
          let mut next_input = input.clone();
          if last_char >= 'a' && last_char <= 'z' {
            while next_input.pop_front() == Some(last_char) {
              if deal_substring(next_input.clone(), pattern.clone()) {
                return true;
              }
            }
          } else if last_char == '.' {
            while next_input.pop_front().is_some() {
              if deal_substring(next_input.clone(), pattern.clone()) {
                return true;
              }
            }
          } else {
            is_match = false;
          }
          last = None;
        }
        (Some(last_char), v) => {
          // check if input is empty
          if let Some(current_char) = input.pop_front() {
            // check if char is match
            if last_char != '.' && current_char != last_char {
              is_match = false;
            }
          } else {
            is_match = false;
          }
          last = v;
        }
        (None, None) => return if input.is_empty() { is_match } else { false },
      }
    }
  }

  deal_substring(input, pattern)
}

#[test]
fn test_is_match() {
  assert_eq!(is_match("abb".to_string(), "bbb*".to_string()), false);
  assert_eq!(is_match("ab".to_string(), ".*c".to_string()), false);
  assert_eq!(is_match("mississippi".to_string(), "mis*is*ip*.".to_string()), true);
  assert_eq!(is_match("abaa".to_string(), "a*a".to_string()), false);
  assert_eq!(is_match("abaa".to_string(), "a*aa".to_string()), false);
  assert_eq!(is_match("abaa".to_string(), "ab*a".to_string()), false);
  assert_eq!(is_match("abaa".to_string(), "aba*".to_string()), true);
  assert_eq!(is_match("abaabb".to_string(), "a*ba*b*".to_string()), true);
  assert_eq!(is_match("abaabb".to_string(), "a*.*b.*a*b*".to_string()), true);
}
