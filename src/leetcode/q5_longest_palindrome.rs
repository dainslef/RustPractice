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

fn longest_palindrome_back(s: String) -> String {
  use std::collections::HashMap;

  let mut temp = "".to_string();
  let mut chars_map = HashMap::new();

  // group by chars
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
        for i_b in (i_a + 1..v.len()).rev() {
          let (first, last) = (v[i_a], v[i_b]);

          if last - first < temp.len() {
            break;
          }

          let sum = first + last + 1;
          let average = sum / 2;
          let (middle1, middle2) = if sum % 2 == 0 {
            (average - 1, average)
          } else {
            (average, average)
          };

          let left = s.get(first..=middle1).map(|s| s.to_string());
          let right = s
            .get(middle2..=last)
            .map(|s| s.chars().rev().collect::<String>());

          if left == right {
            if let Some(c) = s.get(first..=last) {
              temp = c.to_string();
            }
          }
        }
      }
      v.pop();
    }
  }

  if s.len() == 1 && temp.len() == 0 {
    temp = s.chars().last().unwrap().to_string();
  }

  temp
}

fn longest_palindrome(s: String) -> String {
  let mut temp = "";
  let chars = s.chars().collect::<Vec<char>>();

  for i in 0..chars.len() {
    // compute the target substring by the index and return the range and length
    let check_chars = |has_offset| {
      // if has_offset is "false", the structure of string might like "aba"
      // if has_offset is "true", the structure of string might like "abba"
      let (mut start, mut end) = (i, i + if has_offset { 1 } else { 0 });
      loop {
        if end < chars.len() && chars[start] == chars[end] {
          if start > 0 && end < chars.len() - 1 {
            start -= 1;
            end += 1;
          } else {
            break (start, end, end - start + 1);
          }
        } else {
          break (start + 1, end - 1, end - start - 1);
        }
      }
    };

    let (re1, re2) = (check_chars(false), check_chars(true));
    let re = if re1.2 > re2.2 { re1 } else { re2 };
    if re.2 > temp.len() {
      if let Some(ss) = s.get(re.0..=re.1) {
        temp = ss;
      }
    }
  }

  temp.to_string()
}

#[test]
fn test_longest_palindrome() {
  assert_eq!(longest_palindrome("acaca".to_string()), "acaca");
  assert_eq!(longest_palindrome("bbbbb".to_string()), "bbbbb");
  assert_eq!(longest_palindrome("aa789".to_string()), "aa");
  assert_eq!(longest_palindrome("babasssss".to_string()), "sssss");
  assert_eq!(longest_palindrome("\" \"".to_string()), "\" \"");
  assert_eq!(longest_palindrome("asdasdasdasdsad".to_string()), "dasdsad");
  assert_eq!(longest_palindrome("asskassa".to_string()), "assa");
  assert_eq!(longest_palindrome("a".to_string()), "a");
  assert_eq!(longest_palindrome("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
}
