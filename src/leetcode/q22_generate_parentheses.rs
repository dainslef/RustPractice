/**
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 *
 * For example, given n = 3, a solution set is:
 *
 * [
 *   "((()))",
 *   "(()())",
 *   "(())()",
 *   "()(())",
 *   "()()()"
 * ]
 */

fn generate_parenthesis(n: i32) -> Vec<String> {
  let mut temp = vec![(vec![], 0, 0)];

  for i in 0..2 * n {
    let mut next = vec![];
    for v in &temp {
      let mut add_parentheses = |is_left| {
        let mut new_item = v.clone();
        let (c, index) = if is_left {
          ('(', &mut new_item.1)
        } else {
          (')', &mut new_item.2)
        };
        new_item.0.push(c);
        *index += 1;
        next.push(new_item);
      };

      let (left, right) = (v.1, v.2);
      if left > right {
        if left < n || i < n {
          add_parentheses(true);
        }
        add_parentheses(false);
      } else {
        add_parentheses(true);
      }
    }
    temp = next;
  }

  use std::iter::FromIterator;
  temp.into_iter().map(|v| String::from_iter(v.0)).collect()
}

#[test]
fn test_generate_parenthesis() {
  assert_eq!(generate_parenthesis(0), vec![""]);
  assert_eq!(generate_parenthesis(1), vec!["()"]);
  assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
  assert_eq!(
    generate_parenthesis(3),
    vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
  );
  assert_eq!(
    generate_parenthesis(4),
    vec![
      "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
      "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
    ]
  );
  assert_eq!(
    generate_parenthesis(5),
    vec![
      "((((()))))",
      "(((()())))",
      "(((())()))",
      "(((()))())",
      "(((())))()",
      "((()(())))",
      "((()()()))",
      "((()())())",
      "((()()))()",
      "((())(()))",
      "((())()())",
      "((())())()",
      "((()))(())",
      "((()))()()",
      "(()((())))",
      "(()(()()))",
      "(()(())())",
      "(()(()))()",
      "(()()(()))",
      "(()()()())",
      "(()()())()",
      "(()())(())",
      "(()())()()",
      "(())((()))",
      "(())(()())",
      "(())(())()",
      "(())()(())",
      "(())()()()",
      "()(((())))",
      "()((()()))",
      "()((())())",
      "()((()))()",
      "()(()(()))",
      "()(()()())",
      "()(()())()",
      "()(())(())",
      "()(())()()",
      "()()((()))",
      "()()(()())",
      "()()(())()",
      "()()()(())",
      "()()()()()"
    ]
  );
}
