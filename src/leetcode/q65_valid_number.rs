/**
 * Validate if a given string can be interpreted as a decimal number.
 *
 * Some examples:
 * "0" => true
 * " 0.1 " => true
 * "abc" => false
 * "1 a" => false
 * "2e10" => true
 * " -90e3   " => true
 * " 1e" => false
 * "e3" => false
 * " 6e-1" => true
 * " 99e2.5 " => false
 * "53.5e93" => true
 * " --6 " => false
 * "-+3" => false
 * "95a54e53" => false
 *
 * Note: It is intended for the problem statement to be ambiguous. You should gather all requirements up front before implementing one. However, here is a list of characters that can be in a valid decimal number:
 *
 * Numbers 0-9
 * Exponent - "e"
 * Positive/negative sign - "+"/"-"
 * Decimal point - "."
 * Of course, the context of these characters also matters in the input.
 */

/**
 * Valid number regex format:
 * [sign][integral-digits][.[fractional-digits]][e[sign]exponentiation-digits]
 */
fn is_number(s: String) -> bool {
  use std::collections::{HashMap, VecDeque};

  fn chars_to_map(s: &str, flag: u8) -> Option<(HashMap<char, Vec<usize>>, usize, usize)> {
    let mut count = HashMap::new();
    let (mut start, mut end) = (None, None);
    for (i, c) in s.char_indices() {
      if let '0'..='9' = c {
        if start.is_none() {
          start = Some(i);
        }
        end = Some(i);
      }
      if let '0'..='9' | 'e' | '+' | '-' | '.' | ' ' = c {
        count.entry(c).or_insert(vec![]).push(i);
      } else {
        return None;
      }
    }
    if let Some(indexes) = count.remove(&' ') {
      let mut deque = VecDeque::from(indexes);
      if flag & 0b10 > 0 && deque.front() == Some(&0) {
        let mut i = 0;
        while deque.len() > i + 1 && deque[i + 1] - deque[i] == 1 {
          deque.pop_front();
          i += 1;
        }
        deque.pop_front();
      }
      if flag & 0b01 > 0 && deque.back() == Some(&(s.len() - 1)) {
        let mut i = deque.len() - 1;
        while deque.len() > 1 && deque[i] - deque[i - 1] == 1 {
          deque.pop_back();
          i -= 1;
        }
        deque.pop_back();
      }
      if !deque.is_empty() {
        return None;
      }
    }

    if let (Some(start), Some(end)) = (start, end) {
      Some((count, start, end))
    } else {
      None // count is empty means no valid value
    }
  }

  fn check_symbol(count: &HashMap<char, Vec<usize>>, start: usize) -> bool {
    match (count.get(&'+'), count.get(&'-')) {
      (Some(v), None) | (None, Some(v)) if v.len() == 1 && v[0] < start => true,
      (None, None) => true,
      _ => false,
    }
  }

  fn check_dot(count: &HashMap<char, Vec<usize>>, start: usize, end: usize) -> bool {
    match count.get(&'.').map(|v| v.as_slice()) {
      Some([i]) if start.checked_sub(1).map(|v| *i >= v).unwrap_or(true) && *i <= end + 1 => true,
      None => true,
      _ => false,
    }
  }

  fn check_no_dot(count: &HashMap<char, Vec<usize>>) -> bool {
    count.get(&'.').is_none()
  }

  match *s.split('e').collect::<Vec<&str>>() {
    [n] => chars_to_map(n, 0b11)
      .map(|(c, start, end)| check_symbol(&c, start) && check_dot(&c, start, end))
      .unwrap_or(false),
    [n1, n2] if n1 != "" && n2 != "" => chars_to_map(n1, 0b10)
      .and_then(|(c1, start1, end1)| {
        chars_to_map(n2, 0b01).map(|(c2, start2, _)| {
          check_symbol(&c1, start1)
            && check_dot(&c1, start1, end1)
            && check_symbol(&c2, start2)
            && check_no_dot(&c2)
        })
      })
      .unwrap_or(false),
    _ => false,
  }
}

#[test]
fn q65_test() {
  assert_eq!(is_number(" .6 ".into()), true);
  assert_eq!(is_number(" 6. ".into()), true);
  assert_eq!(is_number("0.1".into()), true);
  assert_eq!(is_number(" +.6 ".into()), true);
  assert_eq!(is_number(" .6- ".into()), false);
  assert_eq!(is_number(" .6+ ".into()), false);
  assert_eq!(is_number(" 1e6. ".into()), false);
  assert_eq!(is_number(" 1e.6 ".into()), false);
  assert_eq!(is_number(" 1e6. ".into()), false);
  assert_eq!(is_number(" 1e. ".into()), false);
  assert_eq!(is_number(" +".into()), false);
  assert_eq!(is_number("- ".into()), false);
  assert_eq!(is_number(" + ".into()), false);
  assert_eq!(is_number(" .".into()), false);
  assert_eq!(is_number(". ".into()), false);
  assert_eq!(is_number(" . ".into()), false);
  assert_eq!(is_number(" 6. 1 ".into()), false);
  assert_eq!(is_number(" 6 . 1 ".into()), false);
  assert_eq!(is_number(" 6+ ".into()), false);
  assert_eq!(is_number(" 6 - ".into()), false);
  assert_eq!(is_number(" 6 e -1 ".into()), false);
  assert_eq!(is_number(".".into()), false);
  assert_eq!(is_number(" ".into()), false);
  assert_eq!(is_number("   ".into()), false);
  assert_eq!(is_number(" +1".into()), true);
  assert_eq!(is_number("-1 ".into()), true);
  assert_eq!(is_number(" +1 ".into()), true);
  assert_eq!(is_number(" 0 ".into()), true);
  assert_eq!(is_number(" 1.e1 ".into()), true);
  assert_eq!(is_number(" 0 e 1 ".into()), false);
  assert_eq!(is_number("0 ".into()), true);
  assert_eq!(is_number(" 0e1 ".into()), true);
  assert_eq!(is_number("0".into()), true);
  assert_eq!(is_number("bac".into()), false);
  assert_eq!(is_number("1 a".into()), false);
  assert_eq!(is_number("2e10".into()), true);
  assert_eq!(is_number("-90e3".into()), true);
  assert_eq!(is_number("1e".into()), false);
  assert_eq!(is_number("e3".into()), false);
  assert_eq!(is_number("6e-1".into()), true);
  assert_eq!(is_number("99e2.5".into()), false);
  assert_eq!(is_number("53.5e93".into()), true);
  assert_eq!(is_number(" --6".into()), false);
  assert_eq!(is_number("-+3".into()), false);
  assert_eq!(is_number("95a54e53".into()), false);
  assert_eq!(is_number("+0".into()), true);
  assert_eq!(is_number("++0".into()), false);
}
