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

  fn recurse(mut input: VecDeque<char>, mut pattern: VecDeque<char>) -> bool {
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
            if recurse(next.clone(), pattern.clone()) {
              return true;
            }
          }
          last = None;
        }
        (Some(last_char), v) => {
          match input.pop_front() {
            // get the current input char, check if char is match
            Some(current_char) if last_char != '.' && current_char != last_char => return false,
            // if the input areadly empty, means the pattern isn't match
            None => return false,
            _ => {}
          }
          last = v;
        }
        (None, None) => return input.is_empty(),
      }
    }
  }

  recurse(input, pattern)
}

#[test]
fn q10_test() {
  assert!(!is_match("abb".into(), "bbb*".into()));
  assert!(!is_match("ab".into(), ".*c".into()));
  assert!(is_match("mississippi".into(), "mis*is*ip*.".into()));
  assert!(!is_match("abaa".into(), "a*a".into()));
  assert!(!is_match("abaa".into(), "a*aa".into()));
  assert!(!is_match("abaa".into(), "ab*a".into()));
  assert!(is_match("abaa".into(), "aba*".into()));
  assert!(is_match("abaabb".into(), "a*ba*b*".into()));
  assert!(is_match("abaabb".into(), "a*.*b.*a*b*".into()));
}
