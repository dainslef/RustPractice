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
  let mut temp: Vec<String> = vec![];
  let nums = digits.bytes().map(|v| v - 48).collect::<Vec<u8>>();

  use std::collections::HashMap;
  let mut number_map = HashMap::new();
  number_map.insert(0, vec![' ']);
  number_map.insert(1, vec!['*']);
  number_map.insert(2, vec!['a', 'b', 'c']);
  number_map.insert(3, vec!['d', 'e', 'f']);
  number_map.insert(4, vec!['g', 'h', 'i']);
  number_map.insert(5, vec!['j', 'k', 'l']);
  number_map.insert(6, vec!['m', 'n', 'o']);
  number_map.insert(7, vec!['p', 'q', 'r', 's']);
  number_map.insert(8, vec!['t', 'u', 'v']);
  number_map.insert(9, vec!['w', 'x', 'y', 'z']);

  for n in nums {
    let mut out = vec![];
    for c in &number_map[&n] {
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
fn test_letter_combinations() {
  assert!(super::check_vec_val_eq(
    &letter_combinations("23".to_string()),
    &super::string_vec(&["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"])
  ));
  assert!(super::check_vec_val_eq(
    &letter_combinations("".to_string()),
    &super::string_vec(&[])
  ));
  assert!(super::check_vec_val_eq(
    &letter_combinations("01".to_string()),
    &super::string_vec(&[" *"])
  ));
  assert!(super::check_vec_val_eq(
    &letter_combinations("1210".to_string()),
    &super::string_vec(&["*a* ", "*b* ", "*c* "])
  ));
}
