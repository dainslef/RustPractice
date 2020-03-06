/**
 * Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*'.
 *
 * '?' Matches any single character.
 * '*' Matches any sequence of characters (including the empty sequence).
 * The matching should cover the entire input string (not partial).
 *
 * Note:
 *
 * s could be empty and contains only lowercase letters a-z.
 * p could be empty and contains only lowercase letters a-z, and characters like ? or *.
 * Example 1:
 *
 * Input:
 * s = "aa"
 * p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 * Example 2:
 *
 * Input:
 * s = "aa"
 * p = "*"
 * Output: true
 * Explanation: '*' matches any sequence.
 * Example 3:
 *
 * Input:
 * s = "cb"
 * p = "?a"
 * Output: false
 * Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
 * Example 4:
 *
 * Input:
 * s = "adceb"
 * p = "*a*b"
 * Output: true
 * Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".
 * Example 5:
 *
 * Input:
 * s = "acdcb"
 * p = "a*c?b"
 * Output: false
 */

fn is_match(s: String, p: String) -> bool {
  // save the input chars and pattern have been matched
  let (s_chars, p_chars): (Vec<char>, Vec<char>) = (s.chars().collect(), p.chars().collect());
  // save the match content index
  let (mut s_i, mut p_i) = (0, 0);
  // backup the last match index with the pattern '*'
  let mut backup = None;

  while let Some(current_s) = s_chars.get(s_i) {
    let current_p = p_chars.get(p_i);
    if current_p
      .map(|v| v == current_s || *v == '?')
      .unwrap_or(false)
    {
      // normal match, update the match index
      s_i += 1;
      p_i += 1;
    } else if current_p.map(|v| *v == '*').unwrap_or(false) {
      // match the pattern '*', backup the match index and update pattern index
      backup = Some((s_i, p_i));
      p_i += 1;
    } else if let Some((backup_s_i, backup_p_i)) = backup {
      // if current pattern match nothing, try recovering data from backup
      p_i = backup_p_i;
      s_i = backup_s_i + 1;
    } else {
      break;
    }
  }

  while Some(&'*') == p_chars.get(p_i) {
    // if the current last pattern is '*', can up the pattern index
    p_i += 1;
  }

  // check if the final index match the pattern
  s_i == s_chars.len() && p_i == p_chars.len()
}

fn is_match_recursion(s: String, p: String) -> bool {
  fn recurse(
    s: &Vec<char>,
    p: &Vec<char>,
    s_i: usize,
    mut p_i: usize,
    backup: Option<(usize, usize)>,
  ) -> bool {
    match (s.get(s_i), p.get(p_i)) {
      // macth any content
      (Some(_), Some('*')) => recurse(s, p, s_i, p_i + 1, Some((s_i, p_i))),
      // match the char equals the char in partten
      (Some(s_char), Some(p_char)) if p_char == s_char || p_char == &'?' => {
        recurse(s, p, s_i + 1, p_i + 1, backup)
      }
      // if the target char doesn't match the partten, check if have backup, try recovering from backup
      (Some(_), _) if backup.is_some() => {
        recurse(s, p, backup.unwrap().0 + 1, backup.unwrap().1, backup)
      }
      _ => {
        s_i == s.len() && {
          while Some(&'*') == p.get(p_i) {
            p_i += 1;
          }
          p_i == p.len()
        }
      }
    }
  }

  recurse(&s.chars().collect(), &p.chars().collect(), 0, 0, None)
}

#[test]
fn test_q44() {
  fn test(is_match: impl Fn(String, String) -> bool) {
    assert_eq!(is_match("adceb".into(), "*a*b".into()), true);
    assert_eq!(is_match("".into(), "***".into()), true);
    assert_eq!(is_match("".into(), "*?".into()), false);
    assert_eq!(is_match("aa".into(), "a".into()), false);
    assert_eq!(is_match("aa".into(), "*".into()), true);
    assert_eq!(is_match("aa".into(), "***".into()), true);
    assert_eq!(is_match("aa".into(), "a*".into()), true);
    assert_eq!(is_match("aa".into(), "*a".into()), true);
    assert_eq!(is_match("aa".into(), "?*".into()), true);
    assert_eq!(is_match("aa".into(), "*?".into()), true);
    assert_eq!(is_match("aa".into(), "??".into()), true);
    assert_eq!(is_match("aa".into(), "*a??".into()), false);
    assert_eq!(is_match("acdcb".into(), "a*c?b".into()), false);
    assert_eq!(is_match("acdcb".into(), "a*?cb".into()), true);
    assert_eq!(
      is_match(
        "bbbababbabbbbabbbbaabaaabbbbabbbababbbbababaabbbab".into(),
        "b******b****".into()
      ),
      true
    );
    assert_eq!(
      is_match(
        "bbbababbabbbbabbbbaabaaabbbbabbbababbbbababaabbbab".into(),
        "a******b*".into()
      ),
      false
    );
    assert_eq!(is_match("abaabaaaabbabbaaabaabababbaabaabbabaaaaabababbababaabbabaabbbbaabbbbbbbabaaabbaaaaabbaabbbaaaaabbbabb".into(),
  "ab*aaba**abbaaaa**b*b****aa***a*b**ba*a**ba*baaa*b*ab*".into()), false);
  }

  test(is_match);
  test(is_match_recursion);
}
