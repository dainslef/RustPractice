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
  let mut char_size: HashMap<char, i32> = HashMap::new();

  // record the index of target chars
  for c in t.chars() {
    *targets.entry(c).or_insert(0) += 1;
  }

  let mut range: Option<(usize, usize)> = None;

  for (i, c) in s.char_indices() {
    if targets.contains_key(&c) {
      // add all target chars into a sequence and compute the size of target chars
      char_sequence.push_back((c, i));
      *char_size.entry(c).or_insert(0) += 1;

      // check if the input string contains the target string
      let check_vaild = {
        let mut eq = true;
        for (target_char, target_index) in &targets {
          match char_size.get(target_char) {
            Some(count) => {
              // check if the char size of the input string is less than the target index
              if count < target_index {
                // less means the input string hasn't enough target chars
                eq = false;
                break;
              }
            }
            None => {
              eq = false;
              break;
            }
          }
        }
        eq
      };

      if check_vaild {
        // init the range of the target chars
        let (first, last) = (char_sequence[0], char_sequence[char_sequence.len() - 1]);
        range.get_or_insert((first.1, last.1));

        let mut new_first = first;
        while char_size[&new_first.0] > targets[&new_first.0] {
          // remove the first char in the sequence
          char_sequence.pop_front();
          // update the count of the target char
          char_size.get_mut(&new_first.0).map(|v| *v -= 1);
          new_first = char_sequence[0];
        }

        if let Some((start, end)) = range {
          let (new_start, new_end) = (new_first.1, last.1);
          // check if the window is smaller than the previous window
          if new_end - new_start < end - start {
            range = Some((new_start, new_end));
          }
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
