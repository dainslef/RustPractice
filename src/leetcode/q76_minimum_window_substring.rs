/**
 * Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).
 *
 * Example:
 *
 * Input: S = "ADOBECODEBANC", T = "ABC"
 * Output: "BANC"
 * Note:
 *
 * If there is no such window in S that covers all characters in T, return the empty string "".
 * If there is such window, you are guaranteed that there will always be only one unique minimum window in S.
 */

fn min_window(s: String, t: String) -> String {
  use std::collections::{HashMap, VecDeque};

  let mut targets: HashMap<char, i32> = HashMap::new();
  let mut char_sequence: VecDeque<(char, usize)> = VecDeque::new();
  let mut char_indexes: HashMap<char, i32> = HashMap::new();

  for c in t.chars() {
    *targets.entry(c).or_insert(0) += 1;
  }

  let mut range: Option<(usize, usize)> = None;

  for (i, c) in s.char_indices() {
    if targets.contains_key(&c) {
      char_sequence.push_back((c, i));
      *char_indexes.entry(c).or_insert(0) += 1;

      let check_vaild = {
        let mut eq = true;
        for v in &targets {
          match char_indexes.get(&v.0) {
            Some(count) if count < v.1 => {
              eq = false;
              break;
            }
            None => {
              eq = false;
              break;
            }
            _ => {}
          }
        }
        eq
      };

      if check_vaild {
        let (first, last) = (
          *char_sequence.front().unwrap(),
          *char_sequence.back().unwrap(),
        );
        range.get_or_insert((first.1, last.1));
        let mut new_first = first;
        while char_indexes[&new_first.0] > targets[&new_first.0] {
          char_sequence.pop_front();
          char_indexes.get_mut(&new_first.0).map(|v| *v -= 1);
          new_first = *char_sequence.front().unwrap();
        }
        if (last.1 - new_first.1) < (range.unwrap().1 - range.unwrap().0) {
          range = Some((new_first.1, last.1));
        }
      }
    }
  }

  range
    .and_then(|v| s.get(v.0..=v.1))
    .unwrap_or("")
    .to_string()
}

#[test]
fn test_min_window() {
  assert_eq!(min_window("".to_string(), "".to_string()), "");
  assert_eq!(min_window("bba".to_string(), "ab".to_string()), "ba");
  assert_eq!(
    min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
    "BANC"
  );
  assert_eq!(
    min_window("ABABAAABAA".to_string(), "AAAA".to_string()),
    "ABAAA"
  );
}
