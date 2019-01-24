/**
 * Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
 *
 * Example 1:
 *
 * Input: "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 * Example 2:
 *
 * Input: "cbbd"
 * Output: "bb"
 */

fn longest_palindrome(s: String) -> String {
  use std::collections::HashMap;

  let mut temp = "".to_string();
  let mut chars_map = HashMap::new();

  for (i, c) in s.char_indices() {
    match chars_map.get_mut(&c) {
      None => {
        chars_map.insert(c, vec![i]);
      }
      Some(v) => {
        v.push(i);
      }
    }
  }

  for (_, mut v) in chars_map {
    while v.len() >= 2 {
      for i_a in 0..v.len() - 1 {
        for i_b in i_a + 1..v.len() {
          let (first, last) = (v[i_a], v[i_b]);
          let length = last - first + 1;
          if length > temp.len() {
            let (mut first_index, mut last_index) = (first, last);
            while last_index >= first_index
              && s.get(first_index..=first_index) == s.get(last_index..=last_index)
            {
              first_index += 1;
              last_index -= 1;
            }
            let offset = (last_index + 1) - (first_index - 1);
            if offset == 0 || offset == 1 {
              if let Some(c) = s.get(first..=last) {
                temp = c.to_string();
              }
            }
          }
        }
      }
      v.pop();
    }
  }

  if s.len() >= 1 && temp.len() == 0 {
    temp = s.chars().last().unwrap().to_string();
  }

  temp
}

#[test]
fn test_longest_palindrome() {
  assert_eq!(longest_palindrome("acaca".to_string()), "acaca");
  assert_eq!(longest_palindrome("bbbbb".to_string()), "bbbbb");
  assert_eq!(longest_palindrome("aa789".to_string()), "aa");
  assert_eq!(longest_palindrome("babasssss".to_string()), "sssss");
  assert_eq!(longest_palindrome("\" \"".to_string()), "\" \"");
  assert_eq!(longest_palindrome("asdasdasdasdsad".to_string()), "dasdsad");
  assert_eq!(longest_palindrome("".to_string()), "");
  assert_eq!(longest_palindrome("a".to_string()), "a");
}
