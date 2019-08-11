/**
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
 *
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 *
 *
 *
 * Example:
 *
 * Input: "23"
 * Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 */

fn letter_combinations(digits: String) -> Vec<String> {
  use std::collections::HashMap;

  let mut temp: Vec<String> = vec![];
  let nums = digits.bytes().map(|v| v - 48).collect::<Vec<u8>>();
  let number_to_chars = vec![
    (0, vec![' ']),
    (1, vec!['*']),
    (2, vec!['a', 'b', 'c']),
    (3, vec!['d', 'e', 'f']),
    (4, vec!['g', 'h', 'i']),
    (5, vec!['j', 'k', 'l']),
    (6, vec!['m', 'n', 'o']),
    (7, vec!['p', 'q', 'r', 's']),
    (8, vec!['t', 'u', 'v']),
    (9, vec!['w', 'x', 'y', 'z']),
  ]
  .into_iter()
  .collect::<HashMap<_, _>>();

  for n in nums {
    let mut out = vec![];
    for c in &number_to_chars[&n] {
      if temp.is_empty() {
        out.push(c.to_string());
      } else {
        for old in &temp {
          let mut new = old.clone();
          new.push(*c);
          out.push(new);
        }
      }
    }
    temp = out;
  }

  temp
}

#[test]
fn q17_test() {
  use super::check_element_eq;
  assert!(check_element_eq(
    letter_combinations("23".into()),
    string_vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
  ));
  assert!(check_element_eq(
    letter_combinations("".into()),
    vec![]
  ));
  assert!(check_element_eq(
    letter_combinations("01".into()),
    string_vec![" *"]
  ));
  assert!(check_element_eq(
    letter_combinations("1210".into()),
    string_vec!["*a* ", "*b* ", "*c* "]
  ));
}
