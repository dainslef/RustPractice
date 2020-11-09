/**
 * 87. Scramble String
 * https://leetcode.com/problems/scramble-string/
 *
 * We can scramble a string s to get a string t using the following algorithm:
 *
 * If the length of the string is 1, stop.
 * If the length of the string is > 1, do the following:
 * Split the string into two non-empty substrings at a random index, i.e., if the string is s, divide it to x and y where s = x + y.
 * Randomly decide to swap the two substrings or to keep them in the same order. i.e., after this step, s may become s = x + y or s = y + x.
 * Apply step 1 recursively on each of the two substrings x and y.
 * Given two strings s1 and s2 of the same length, return true if s2 is a scrambled string of s1, otherwise, return false.
 *
 *
 *
 * Example 1:
 *
 * Input: s1 = "great", s2 = "rgeat"
 * Output: true
 * Explanation: One possible scenario applied on s1 is:
 * "great" --> "gr/eat" // divide at random index.
 * "gr/eat" --> "gr/eat" // random decision is not to swap the two substrings and keep them in order.
 * "gr/eat" --> "g/r / e/at" // apply the same algorithm recursively on both substrings. divide at ranom index each of them.
 * "g/r / e/at" --> "r/g / e/at" // random decision was to swap the first substring and to keep the second substring in the same order.
 * "r/g / e/at" --> "r/g / e/ a/t" // again apply the algorithm recursively, divide "at" to "a/t".
 * "r/g / e/ a/t" --> "r/g / e/ a/t" // random decision is to keep both substrings in the same order.
 * The algorithm stops now and the result string is "rgeat" which is s2.
 * As there is one possible scenario that led s1 to be scrambled to s2, we return true.
 * Example 2:
 *
 * Input: s1 = "abcde", s2 = "caebd"
 * Output: false
 * Example 3:
 *
 * Input: s1 = "a", s2 = "a"
 * Output: true
 *
 *
 * Constraints:
 *
 * s1.length == s2.length
 * 1 <= s1.length <= 30
 * s1 and s2 consist of lower-case English letters.
 */

/**
 * Runtime: 4 ms
 * Memory Usage: 2.1 MB
 */
fn is_scramble(s1: String, s2: String) -> bool {
  use std::collections::HashMap;

  if s1 == s2 {
    return true;
  }
  if s1.len() == 1 || s1.len() != s2.len() {
    return false;
  }

  let (mut chars_map1, mut chars_map2) = (HashMap::new(), HashMap::new());
  for i in 0..s1.len() {
    if let (Some(k1), Some(k2)) = (s1.get(i..i + 1), s2.get(i..i + 1)) {
      *chars_map1.entry(k1).or_insert(0) += 1;
      *chars_map2.entry(k2).or_insert(0) += 1;
    }
  }
  if chars_map1 != chars_map2 {
    return false;
  }

  macro_rules! check {
    ($l1: expr, $r1: expr, $l2: expr, $r2: expr) => {{
      let mut result = false;
      if $l1.len() == $r1.len() {
        result |= is_scramble($l1.into(), $r1.into()) && is_scramble($l2.into(), $r2.into())
      }
      if !result && $l1.len() == $r2.len() {
        result |= is_scramble($l1.into(), $r2.into()) && is_scramble($l2.into(), $r1.into())
      }
      result
    }};
  }

  for i in 1..s1.len() {
    let (s1_head, s1_tail) = s1.split_at(i);
    let (s2_head1, s2_tail1) = s2.split_at(i);
    let (s2_head2, s2_tail2) = s2.split_at(s1.len() - i);
    if check!(s1_head, s2_head1, s1_tail, s2_tail1) || check!(s1_head, s2_head2, s1_tail, s2_tail2)
    {
      return true;
    }
  }

  false
}

#[test]
fn q87_test() {
  assert!(!is_scramble(
    "abcdefghijklmnopq".into(),
    "efghijklmnopqcadb".into()
  ));
  assert!(is_scramble("abb".into(), "bba".into()));
  assert!(is_scramble("great".into(), "rgeat".into()));
  assert!(!is_scramble("abcde".into(), "caebd".into()));
  assert!(is_scramble("a".into(), "a".into()));
}
