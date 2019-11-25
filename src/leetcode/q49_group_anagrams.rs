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
  // calculate the chars of different string
  fn key(s: &String) -> [u32; 26] {
    let mut chars = [0; 26]; // use array to record the size of chars
    for c in s.chars() {
      chars[c as usize - 'a' as usize] += 1; // update the size of chars
    }
    chars
  }

  use std::collections::HashMap;
  let mut temp = HashMap::new(); // use HashMap to group the strings

  for s in strs {
    // update the string group by chars info
    temp.entry(key(&s)).or_insert(vec![]).push(s);
  }

  // collect the values of hash map
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
