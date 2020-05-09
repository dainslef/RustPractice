/**
 * 844. Backspace String Compare
 * https://leetcode.com/problems/backspace-string-compare/
 *
 * Given two strings S and T, return if they are equal when both are typed into empty text editors. # means a backspace character.
 *
 * Example 1:
 *
 * Input: S = "ab#c", T = "ad#c"
 * Output: true
 * Explanation: Both S and T become "ac".
 * Example 2:
 *
 * Input: S = "ab##", T = "c#d#"
 * Output: true
 * Explanation: Both S and T become "".
 * Example 3:
 *
 * Input: S = "a##c", T = "#a#c"
 * Output: true
 * Explanation: Both S and T become "c".
 * Example 4:
 *
 * Input: S = "a#c", T = "b"
 * Output: false
 * Explanation: S becomes "c" while T becomes "b".
 * Note:
 *
 * 1 <= S.length <= 200
 * 1 <= T.length <= 200
 * S and T only contain lowercase letters and '#' characters.
 * Follow up:
 *
 * Can you solve it in O(N) time and O(1) space?
 */

// O(1) space means you can't create another new data structure
fn backspace_compare_recursion(s: String, t: String) -> bool {
  fn recurse(mut s: Vec<u8>, mut t: Vec<u8>) -> bool {
    let (sharp, sharp_ref) = (Some('#' as u8), Some(&('#' as u8)));

    macro_rules! remove_sharp {
      ($bytes: expr) => {
        if $bytes.last() == sharp_ref {
          $bytes.pop();
          let mut count = 1;
          while count > 0 {
            if $bytes.pop() == sharp {
              count += 1;
            } else {
              count -= 1;
            }
          }
        }
      };
    }

    remove_sharp!(s);
    remove_sharp!(t);

    if s.last() == sharp_ref || t.last() == sharp_ref {
      recurse(s, t) // if last element still contains '#', run a new round of recursion
    } else if s.is_empty() || t.is_empty() {
      s == t // some one is empty means compare is end
    } else {
      s.pop() == t.pop() && recurse(s, t) // if both last elements are normal chars, compare and run a new round of recrsion (if the result of compare is true)
    }
  }

  recurse(s.into_bytes(), t.into_bytes())
}

// loop solution (need confirm !!!)
fn backspace_compare(s: String, t: String) -> bool {
  let (mut s, mut t) = (s.into_bytes(), t.into_bytes());
  let (sharp, sharp_ref) = (Some('#' as u8), Some(&('#' as u8)));

  loop {
    macro_rules! remove_sharp {
      ($bytes: expr) => {
        if $bytes.last() == sharp_ref {
          $bytes.pop();
          let mut count = 1;
          while count > 0 {
            if $bytes.pop() == sharp {
              count += 1;
            } else {
              count -= 1;
            }
          }
        }
      };
    }

    remove_sharp!(s);
    remove_sharp!(t);

    if s.last() != sharp_ref && t.last() != sharp_ref {
      if s.is_empty() || t.is_empty() {
        break s == t;
      } else if s.pop() != t.pop() {
        break false;
      }
    }
  }
}

#[test]
fn q844_test() {
  fn test(backspace_compare: impl Fn(String, String) -> bool) {
    assert_eq!(backspace_compare("a##c".into(), "#a#c".into()), true);
    assert_eq!(backspace_compare("ab##".into(), "c#d#".into()), true);
    assert_eq!(
      backspace_compare("xywrrmp".into(), "xywrrmu#p".into()),
      true
    );
  }

  test(backspace_compare);
  test(backspace_compare_recursion);
}
