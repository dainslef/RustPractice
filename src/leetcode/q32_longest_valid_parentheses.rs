/**
 * Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.
 *
 * Example 1:
 *
 * Input: "(()"
 * Output: 2
 * Explanation: The longest valid parentheses substring is "()"
 * Example 2:
 *
 * Input: ")()())"
 * Output: 4
 * Explanation: The longest valid parentheses substring is "()()"
 */

fn longest_valid_parentheses(s: String) -> i32 {
  use std::collections::VecDeque;

  // use a stack to record the index of "()"
  let mut symbols = VecDeque::new();
  for (i, c) in s.char_indices() {
    match (i as i32, c) {
      v @ (_, '(') => symbols.push_back(v),
      v @ (_, ')') => match symbols.back() {
        Some((_, '(')) => drop(symbols.pop_back()),
        _ => symbols.push_back(v),
      },
      _ => {}
    }
  }

  // the normal indexes range is [0, length - 1], add the first index(-1) and the last index(length) to the indexes
  symbols.push_front((-1, '_'));
  symbols.push_back((s.len() as i32, '_'));

  let mut max_count = 0;
  for i in 1..symbols.len() {
    let count = symbols[i].0 - symbols[i - 1].0 - 1;
    if count > max_count {
      max_count = count;
    }
  }

  max_count
}

#[test]
fn q32_test() {
  assert_eq!(longest_valid_parentheses("()".to_string()), 2);
  assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
  assert_eq!(longest_valid_parentheses("()(()".to_string()), 2);
  assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
  assert_eq!(longest_valid_parentheses(")(()))((()))".to_string()), 6);
}
