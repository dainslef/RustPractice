/**
 * Given an array of strings, group anagrams together.
 *
 * Example:
 *
 * Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
 * Output:
 * [
 *   ["ate","eat","tea"],
 *   ["nat","tan"],
 *   ["bat"]
 * ]
 * Note:
 *
 * All inputs will be in lowercase.
 * The order of your output does not matter.
 */

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
  fn key(s: &String) -> Vec<u32> {
    let mut chars = ('a' as u8..='z' as u8)
      .map(|_| 0)
      .collect::<Vec<u32>>();

    for c in s.chars() {
      chars[c as usize - 'a' as usize] += 1;
    }

    chars
  }

  use std::collections::HashMap;
  let mut temp = HashMap::new();

  for s in strs {
    let t = temp.entry(key(&s)).or_insert(vec![]);
    t.push(s);
  }

  temp.into_iter().map(|v| v.1).collect()
}

#[test]
fn q49_test() {
  use super::check_element_eq;

  assert!(check_element_eq(
    group_anagrams(string_vec!["eat", "tea", "tan", "ate", "nat", "bat"]).iter(),
    vec![
      string_vec!["bat"],
      string_vec!["tan", "nat"],
      string_vec!["eat", "tea", "ate"]
    ]
    .iter(),
  ));
}
